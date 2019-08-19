// Copyright (c) 2019 Gabriela Alexandra Moldovan
// Copyright (c) 2019 King's College London.
// Created by the Software Development Team https://soft-dev.org
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0>, or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, or the UPL-1.0 license <http://opensource.org/licenses/UPL>
// at your option. This file may not be copied, modified, or distributed except according to those
// terms.

use crate::benchmark::Benchmark;

use std::{collections::HashMap, path::PathBuf, process::Command};

pub trait LangImpl {
    fn results_key(&self) -> &str;
    /// Run the language implementation on the specified benchmark.
    fn invoke(&self, benchmark: &Benchmark);
}

pub struct GenericScriptingVm {
    /// The path of the interpreter.
    interp_path: PathBuf,
    /// The environment to use when running the VM.
    env: HashMap<String, String>,
}

impl GenericScriptingVm {
    pub fn new(path: &str) -> GenericScriptingVm {
        GenericScriptingVm {
            interp_path: PathBuf::from(path),
            env: Default::default(),
        }
    }

    pub fn env(mut self, k: &str, v: &str) -> GenericScriptingVm {
        self.env.insert(k.to_string(), v.to_string());
        self
    }
}

impl LangImpl for GenericScriptingVm {
    fn results_key(&self) -> &str {
        self.interp_path
            .to_str()
            .expect("The path should be valid unicode!")
    }

    fn invoke(&self, benchmark: &Benchmark) {
        let _ = Command::new(&self.interp_path)
            .arg(benchmark.path())
            .args(benchmark.args())
            .envs(&self.env)
            .output()
            .expect("failed to execute process");
    }
}

pub struct GenericNativeCode {
    /// The environment to use.
    pub env: HashMap<String, String>,
}

impl GenericNativeCode {
    pub fn new() -> GenericNativeCode {
        GenericNativeCode {
            env: Default::default(),
        }
    }

    pub fn env(mut self, k: &str, v: &str) -> GenericNativeCode {
        self.env.insert(k.to_string(), v.to_string());
        self
    }
}

impl LangImpl for GenericNativeCode {
    fn results_key(&self) -> &str {
        unimplemented!("results_key");
    }

    fn invoke(&self, _benchmark: &Benchmark) {
        unimplemented!("invoke");
    }
}
