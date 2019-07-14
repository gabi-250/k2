// Copyright (c) 2019 Gabriela Alexandra Moldovan
// Copyright (c) 2019 King's College London.
// Created by the Software Development Team https://soft-dev.org
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0>, or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, or the UPL-1.0 license <http://opensource.org/licenses/UPL>
// at your option. This file may not be copied, modified, or distributed except according to those
// terms.

use std::time::Duration;

/// The configuration that specifies how to run the benchmarks.
#[derive(Debug)]
pub(crate) struct Config {
    /// The path of the result file.
    pub result_path: String,
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
    pub fn new() -> Config {
        Config {
            result_path: "default/path".to_string(),
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
