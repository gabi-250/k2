// Copyright (c) 2019 Gabriela Alexandra Moldovan
// Copyright (c) 2019 King's College London.
// Created by the Software Development Team https://soft-dev.org
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0>, or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, or the UPL-1.0 license <http://opensource.org/licenses/UPL>
// at your option. This file may not be copied, modified, or distributed except according to those
// terms.

use crate::{benchmark::Benchmark, config::Config, error::K2Error, limit::Limit};

use std::time::Duration;

/// The experiment runner.
pub struct Experiment<'a> {
    /// The configuration variables.
    config: Config,
    /// The benchmarks to run.
    benchmarks: Vec<&'a Benchmark<'a>>,
}

impl<'a> Experiment<'a> {
    // Private: experiments should always be created through the ExperimentBuilder.
    fn new(config: Config, benchmarks: Vec<&'a Benchmark>) -> Self {
        Experiment { config, benchmarks }
    }

    /// Run the experiment. If experiment completes successfully, return a String
    /// which represents the path of the results file; otherwise, return a `K2Error`.
    pub fn run(self) -> Result<String, K2Error> {
        unimplemented!("run");
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
    pub fn new() -> Self {
        ExperimentBuilder {
            config: Config::new(),
            benchmarks: Default::default(),
        }
    }

    pub fn result_path(mut self, result_path: &str) -> Self {
        self.config.result_path = result_path.to_string();
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
