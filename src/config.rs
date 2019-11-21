use std::{path::PathBuf, time::Duration};

/// The configuration that specifies how to run the benchmarks.
#[derive(Debug)]
pub(crate) struct Config {
    /// The path of the directory where to store the results and the manifest.
    pub results_dir: PathBuf,
    /// Run the benchmarks in quick mode (for development/testing purposes).
    pub quick: bool,
    /// Don't actually run the benchmarks (for development/testing purposes).
    pub dry_run: bool,
    /// Automatically reboot between pexecs.
    pub reboot: bool,
    /// The list of emails to send notifications/errors to.
    pub mail_to: Vec<String>,
    /// The number of in-process iterations.
    pub in_proc_iters: usize,
    /// The number of process executions.
    pub pexecs: usize,
    /// The amount of time to wait before taking the initial temperature reading.
    pub temp_read_pause: Duration,
}

impl Config {
    pub fn new(results_dir: PathBuf) -> Config {
        Config {
            results_dir,
            quick: false,
            dry_run: false,
            reboot: false,
            mail_to: Default::default(),
            in_proc_iters: 40,
            pexecs: 1,
            temp_read_pause: Duration::from_secs(60),
        }
    }
}
