use crate::{
    benchmark::Benchmark,
    config::Config,
    db::K2Store,
    error::K2Error,
    manifest::{JobStatus, ManifestManager},
    util,
};

use std::{
    fs,
    path::{Path, PathBuf},
    time::Duration,
};

/// The experiment runner.
pub struct Experiment<'a> {
    /// The configuration variables.
    config: Config,
    /// The benchmarks to run.
    benchmarks: Vec<&'a Benchmark<'a>>,
    /// An interface to the manifest used to schedule benchmark execution.
    manifest: ManifestManager,
    /// Whether is is the first run of the experiment.
    first_run: bool,
    /// An interface to the underlying database.
    store: K2Store,
}

impl<'a> Experiment<'a> {
    // Private: experiments should always be created through the ExperimentBuilder.
    fn new(config: Config, benchmarks: Vec<&'a Benchmark>) -> Self {
        let first_run = if Path::new(&config.results_dir).exists() {
            false
        } else {
            // Create a directory to store the results and the manifest.
            fs::create_dir(&config.results_dir).expect("Failed to create results dir");
            true
        };
        let manifest = ManifestManager::new(&config, &benchmarks);
        let store = K2Store::new(&config.results_dir);
        Experiment {
            config,
            benchmarks,
            manifest,
            first_run,
            store,
        }
    }

    /// Run the experiment. If experiment completes successfully, return a String
    /// which represents the path of the results file; otherwise, return a `K2Error`.
    pub fn run(mut self) -> Result<PathBuf, K2Error> {
        // Run the next outstanding benchmark.
        if let Some(job) = self.manifest.next_job() {
            // `job` is the index of the next job to run. Each benchmark is run
            // `config.pexecs` times, so we use modular arithmetic to work out the
            // index of the next benchmark to run.
            let bench = &self.benchmarks[job % self.benchmarks.len()];
            let result = bench.run(&self.config);
            let status = match result {
                Ok(_) => JobStatus::Done,
                Err(K2Error::RerunError) => JobStatus::Outstanding,
                Err(_) => JobStatus::Error,
            };
            // If we've just run the first job, create all the necessary tables.
            if self.first_run {
                // Create a table to store the status of each job.
                self.store.create_job_table(&self.config, &self.benchmarks);
                // FIXME: create a table for the measurements too.
            }
            // Update the status of the job we've just run.
            self.manifest.update_status(status);
            // Increment `num_reboots`, since we are about to reboot before running
            // the next job.
            self.manifest.update_num_reboots();
            // FIXME: Record the measurements for this benchmark.
            // Persist all the changes.
            self.manifest.sync(&mut self.store);
            // Reboot before running the next job.
            Err(util::reboot(self.config.reboot))
        } else {
            // There are no more benchmarks to run: return the path.
            Ok(self.config.results_dir.join(K2Store::K2_DB))
        }
    }
}

/// A builder used to construct an `Experiment`.
///
/// This populates a `Config` struct with values, and collects the benchmarks
/// to run.
pub struct ExperimentBuilder<'a> {
    config: Config,
    benchmarks: Vec<&'a Benchmark<'a>>,
}

impl<'a> ExperimentBuilder<'a> {
    /// Set up a new experiment builder.
    ///
    /// The experiment results and manifest are stored in `results_dir`.
    pub fn new<P: AsRef<Path>>(results_dir: P) -> Self {
        ExperimentBuilder {
            config: Config::new(results_dir.as_ref().into()),
            benchmarks: Default::default(),
        }
    }

    pub fn results_dir<P: AsRef<Path>>(mut self, results_dir: P) -> Self {
        self.config.results_dir = results_dir.as_ref().to_path_buf();
        self
    }

    pub fn quick(mut self, quick: bool) -> Self {
        self.config.quick = quick;
        self
    }

    pub fn dry_run(mut self, dry_run: bool) -> Self {
        self.config.dry_run = dry_run;
        self
    }

    pub fn reboot(mut self, reboot: bool) -> Self {
        self.config.reboot = reboot;
        self
    }

    pub fn mail_to(mut self, mail_to: Vec<String>) -> Self {
        self.config.mail_to = mail_to;
        self
    }

    pub fn in_proc_iters(mut self, in_proc_iters: usize) -> Self {
        self.config.in_proc_iters = in_proc_iters;
        self
    }

    pub fn pexecs(mut self, pexecs: usize) -> Self {
        self.config.pexecs = pexecs;
        self
    }

    pub fn temp_read_pause(mut self, temp_read_pause: Duration) -> Self {
        self.config.temp_read_pause = temp_read_pause;
        self
    }

    /// Add `bench` to the list of benchmarks to run.
    pub fn benchmark(mut self, bench: &'a Benchmark) -> Self {
        self.benchmarks.push(bench);
        self
    }

    /// Consume the builder and create an `Experiment` with the `config` and
    /// `benchmarks` recorded.
    pub fn build(self) -> Experiment<'a> {
        Experiment::new(self.config, self.benchmarks)
    }
}
