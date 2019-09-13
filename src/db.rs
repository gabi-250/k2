// Copyright (c) 2019 Gabriela Alexandra Moldovan
// Copyright (c) 2019 King's College London.
// Created by the Software Development Team https://soft-dev.org
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0>, or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, or the UPL-1.0 license <http://opensource.org/licenses/UPL>
// at your option. This file may not be copied, modified, or distributed except according to those
// terms.

use crate::{
    benchmark::Benchmark,
    config::Config,
    manifest::{Job, JobStatus},
};

use rusqlite::{self, params, Connection};

use std::path::{Path, PathBuf};

/// A wrapper around the database connection.
pub(crate) struct K2Store {
    connection: Option<Connection>,
    db_path: PathBuf,
}

impl<'a> K2Store {
    /// The k2 database file.
    pub const K2_DB: &'static str = "k2.db";

    pub fn new<P: AsRef<Path>>(k2_dir: P) -> K2Store {
        // The database connection is not created until it's actually needed.
        K2Store {
            connection: None,
            db_path: k2_dir.as_ref().join(Self::K2_DB),
        }
    }

    /// Open a new connection to the SQLite database, and return a reference to it.
    fn connection(&mut self) -> &Connection {
        let db_path = &self.db_path;
        self.connection.get_or_insert_with(|| {
            Connection::open(db_path).expect("Failed to connect to the k2 database")
        })
    }

    /// Create the `job` table.
    ///
    /// The table created by this function records the status and key of each job.
    pub fn create_job_table(&mut self, config: &Config, benchmarks: &[&'_ Benchmark]) {
        let connection = self.connection();
        connection
            .execute("CREATE TABLE job(
                        job_id INTEGER PRIMARY KEY,
                        key TEXT NOT NULL,
                        status INTEGER NOT NULL);", rusqlite::NO_PARAMS)
            .expect("Failed to create the job table");
        let mut stmt = connection
            .prepare("INSERT INTO job VALUES ($1, $2, $3)")
            .expect("Failed to prepare query.");
        let mut id = 0;
        for _ in 0..config.pexecs {
            for bench in benchmarks {
                let job = Job::new(id, &bench);
                id += 1;
                stmt
                    .execute(params![job.id as i64, job.key, job.status as i64])
                    .expect("Failed to populate the job table");
            }
        }
    }

    /// Set the status of the job with identifier `id` to `status`.
    pub fn update_status(&mut self, id: usize, status: JobStatus) {
        let connection = self.connection();
        let mut stmt = connection
            .prepare("UPDATE job SET status = $1 WHERE job_id = $2;")
            .expect("Failed to prepare query.");
        stmt
            .execute(params![status as i64, id as i64])
            .expect("Failed to create the job table");
    }
}
