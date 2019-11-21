use crate::{benchmark::Benchmark, config::Config, db::K2Store, util::num_digits};

use rand::{self, seq::SliceRandom};

use std::{
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
};

#[derive(Debug, Copy, Clone)]
pub(crate) enum JobStatus {
    Outstanding,
    Done,
    Error,
}

#[derive(Debug)]
pub(crate) struct Job {
    /// The unique identifier of the job. This is used as a primary key for the `job`
    /// table.
    pub id: usize,
    /// A string that identifies the benchmark/language implementation associated
    /// with this job.
    pub key: String,
    /// The status of this job.
    pub status: JobStatus,
}

impl Job {
    pub fn new(id: usize, bench: &Benchmark) -> Job {
        Job {
            id,
            key: bench.results_key(),
            status: JobStatus::Outstanding,
        }
    }
}

/// The `num_reboots` field of the manifest header.
const NUM_REBOOTS: &str = "num_reboots";
/// The `num_reboots` field has a fixed width of 8 bytes.
const NUM_REBOOTS_BYTES: usize = 8;
/// The `next_idx` field of the manifest header.
const NEXT_IDX: &str = "next_idx";
/// The `next_idx` field has a fixed width of 4 bytes.
const NEXT_IDX_BYTES: usize = 4;
/// The `ordering` field of the manifest header.
const ORDERING: &str = "ordering";

/// The type of an offset in the manifest header file.
type Offset = u64;

/// Format `value` as a string of `width` bytes, padding with zeroes if necessary.
///
/// # Panics
///
/// Panics if the string representation of `value` exceeds `width` bytes.
fn format_int_field(value: usize, width: usize) -> String {
    let bytes = num_digits(value);
    assert!(bytes <= width, "{} <= {} is false", bytes, width);
    let padding = width - bytes;
    format!("{}{}", "0".repeat(padding as usize), value)
}

#[derive(Debug)]
struct ManifestHeader {
    /// The path of the header.
    hdr_path: PathBuf,
    /// The value of the `num_reboots` field.
    num_reboots: usize,
    /// The offset of the `num_reboots` field.
    num_reboots_offset: Offset,
    /// The value of the `next_idx` field.
    next_idx: usize,
    /// The offset of the `next_idx` field.
    next_idx_offset: Offset,
    /// The value of the `ordering` field. This field indicates the order in which
    /// to run the jobs.
    ordering: Vec<usize>,
}

impl ManifestHeader {
    /// The name of the manifest header file.
    const MANIFEST_HDR: &'static str = "manifest.k2";

    pub fn new<P: AsRef<Path>>(results_dir: P, num_jobs: usize) -> ManifestHeader {
        let hdr_path = results_dir.as_ref().join(Self::MANIFEST_HDR);
        if !Path::new(&hdr_path).exists() {
            // Create a blank manifest header file. The `ordering` field contains a
            // permutation of the numbers from 0 to `num_jobs` (the jobs are run in
            // random order).
            ManifestHeader {
                hdr_path: hdr_path.clone(),
                num_reboots: 0,
                num_reboots_offset: 0,
                next_idx: 0,
                next_idx_offset: 0,
                ordering: ManifestHeader::random_ordering(num_jobs),
            }
            .write();
        }
        // Parse the file to work out the actual field offsets.
        ManifestHeader::parse(&hdr_path)
    }

    fn parse<P: AsRef<Path>>(path: P) -> ManifestHeader {
        // The fields of the manifest header.
        let mut num_reboots: Option<(usize, Offset)> = None;
        let mut next_idx: Option<(usize, Offset)> = None;
        let mut ordering: Option<Vec<usize>> = None;
        let file = File::open(&path).expect("Failed to read manifest header");
        // The offset of the current line.
        let mut offset = 0;
        for line in BufReader::new(file).lines() {
            let line = line.expect("Failed to read line");
            // Each line is a key-value pair.
            let mut pair = line.split('=');
            let key = pair.next().expect("No key specified");
            let value = pair.next().expect("No value specified");
            assert!(pair.next().is_none(), "Broken line: {}", line);
            match key {
                ORDERING => {
                    let value = value
                        .split(',')
                        .map(|x| x.parse::<usize>().unwrap())
                        .collect();
                    ordering = Some(value)
                }
                key => {
                    // Get the actual width of this field.
                    let val_bytes = value.len();
                    let value = value
                        .to_string()
                        .parse::<usize>()
                        .unwrap_or_else(|_| panic!("{} must be a usize", key));
                    // The offset of this field. Add 1 to skip over the '='.
                    let val_offset = (offset + key.len() + 1) as Offset;
                    // Get the expected width of this field.
                    let width = match key {
                        NUM_REBOOTS => {
                            num_reboots = Some((value, val_offset));
                            NUM_REBOOTS_BYTES
                        }
                        NEXT_IDX => {
                            next_idx = Some((value, val_offset));
                            NEXT_IDX_BYTES
                        }
                        &_ => panic!("Unexpected key {}", key),
                    };
                    // Make sure the `num_reboots` and `next_idx` fields have the
                    // expected width.
                    assert!(val_bytes == width);
                }
            }
            // Add 1 to the offset of the current line to account for newline character.
            offset += line.len() + 1;
        }
        let (num_reboots, num_reboots_offset) = num_reboots
            .unwrap_or_else(|| panic!("{} key not set", NUM_REBOOTS));
        let (next_idx, next_idx_offset) = next_idx
            .unwrap_or_else(|| panic!("{} key not set", NEXT_IDX));
        ManifestHeader {
            hdr_path: PathBuf::from(path.as_ref()),
            num_reboots,
            num_reboots_offset,
            next_idx,
            next_idx_offset,
            ordering: ordering.expect("ordering key not set"),
        }
    }

    /// Create the manifest header file.
    fn write(&self) {
        let num_reboots = format_int_field(self.num_reboots, NUM_REBOOTS_BYTES);
        let next_idx = format_int_field(self.next_idx, NEXT_IDX_BYTES);
        if !Path::new(&self.hdr_path).exists() {
            let manifest_hdr = format!("{}={}\n{}={}\n{}={}",
                NUM_REBOOTS, num_reboots,
                NEXT_IDX, next_idx,
                ORDERING, self.ordering_str());
            fs::write(&self.hdr_path, manifest_hdr).expect("Failed to write the manifest header");
        }
    }

    fn ordering_str(&self) -> String {
        let ordering: Vec<String> = self.ordering.iter().map(|x| x.to_string()).collect();
        ordering.join(",")
    }

    /// Update the `num_reboots` and `next_idx` fields.
    fn sync(&self) {
        let num_reboots = format_int_field(self.num_reboots, NUM_REBOOTS_BYTES);
        let next_idx = format_int_field(self.next_idx, NEXT_IDX_BYTES);
        match OpenOptions::new().write(true).open(&self.hdr_path) {
            Ok(mut f) => {
                f.seek(SeekFrom::Start(self.num_reboots_offset)).unwrap();
                f.write(num_reboots.as_bytes()).unwrap();
                f.seek(SeekFrom::Start(self.next_idx_offset)).unwrap();
                f.write(next_idx.as_bytes()).unwrap();
            }
            Err(err) => panic!("Failed to open manifest header: {}", err),
        }
    }

    /// Generate a random permutation for the job ordering.
    fn random_ordering(num_jobs: usize) -> Vec<usize> {
        let mut ordering: Vec<usize> = (0..num_jobs).collect();
        ordering.shuffle(&mut rand::thread_rng());
        ordering
    }
}

pub(crate) struct ManifestManager {
    /// The manifest header.
    manifest_hdr: ManifestHeader,
    /// The status of the current job.
    cur_status: JobStatus,
}

impl ManifestManager {
    pub fn new(config: &Config, benchmarks: &[&'_ Benchmark]) -> ManifestManager {
        let num_jobs = config.pexecs * benchmarks.len();
        let manifest_hdr = ManifestHeader::new(&config.results_dir, num_jobs);
        ManifestManager {
            manifest_hdr,
            cur_status: JobStatus::Outstanding,
        }
    }

    /// Returns the index of the next job to run, or `None` if there are no more
    /// outstanding jobs.
    pub fn next_job(&self) -> Option<usize> {
        if self.manifest_hdr.next_idx < self.manifest_hdr.ordering.len() {
            Some(self.manifest_hdr.ordering[self.manifest_hdr.next_idx])
        } else {
            None
        }
    }

    /// Updates the status of the current job to `status`.
    pub fn update_status(&mut self, status: JobStatus) {
        self.cur_status = status;
        match status {
            JobStatus::Done | JobStatus::Error => {
                self.manifest_hdr.next_idx += 1;
                let bytes = num_digits(self.manifest_hdr.next_idx);
                assert!(bytes <= NEXT_IDX_BYTES, "{} <= {} is false", bytes, NEXT_IDX_BYTES);
            }
            _ => {}
        }
    }

    /// Increments the number of reboots.
    pub fn update_num_reboots(&mut self) {
        let bytes = num_digits(self.manifest_hdr.num_reboots);
        assert!(bytes <= NUM_REBOOTS_BYTES, "{} <= {} is false", bytes, NUM_REBOOTS_BYTES);
        self.manifest_hdr.num_reboots += 1;
    }

    /// Writes the manifest header and the status of the current job.
    pub fn sync(&self, store: &mut K2Store) {
        self.manifest_hdr.sync();
        store.update_status(
            self.manifest_hdr.ordering[self.manifest_hdr.next_idx - 1],
            self.cur_status,
        );
    }
}
