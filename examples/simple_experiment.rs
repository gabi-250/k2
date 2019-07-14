// Copyright (c) 2019 Gabriela Alexandra Moldovan
// Copyright (c) 2019 King's College London.
// Created by the Software Development Team https://soft-dev.org
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0>, or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, or the UPL-1.0 license <http://opensource.org/licenses/UPL>
// at your option. This file may not be copied, modified, or distributed except according to those
// terms.

use k2::{
    benchmark::Benchmark, experiment::ExperimentBuilder, lang_impl::GenericScriptingVm,
    limit::Limit, util::find_executable,
};

use clap::{App, Arg};

fn main() {
    // Note: `find_executable` relies on $PATH. For a real experiment, you will
    // probably want to use absolute paths instead.
    let python_bin = find_executable("python");
    let pypy_bin = find_executable("pypy");
    let luajit_bin = find_executable("luajit");
    let expb = setup();
    let cpython = GenericScriptingVm::new(&python_bin);
    let pypy = GenericScriptingVm::new(&pypy_bin);
    let luajit = GenericScriptingVm::new(&luajit_bin);
    let cpython_bench = Benchmark::new("./benchmarks/binarytrees/binarytrees.py", &cpython)
        .tag("benchmark_name", "binarytrees");
    let pypy_bench = Benchmark::new("./benchmarks/binarytrees/binarytrees.py", &pypy)
        .tag("benchmark_name", "binarytrees");
    let lua_bench = Benchmark::new("./benchmarks/binarytrees/binarytrees.lua", &luajit)
        .tag("benchmark_name", "binarytrees")
        .stack_lim(Limit::KiB(8.192))
        .heap_lim(Limit::GiB(2.097152));
    let exp = expb
        .benchmark(&cpython_bench)
        .benchmark(&pypy_bench)
        .benchmark(&lua_bench)
        .build();
    // `run` outputs the result in the k2 internal format.
    let _ = exp.run().expect("Failed to run the experiment");
}

fn setup<'a>() -> ExperimentBuilder<'a> {
    let expb = parse_args(ExperimentBuilder::new());
    // These could've been command-line arguments too.
    expb.pexecs(2).in_proc_iters(40)
}

fn parse_args(expb: ExperimentBuilder) -> ExperimentBuilder {
    // Parse the args and create a `Config`.
    let matches = App::new("k2")
        .arg(Arg::with_name("quick")
                .short("q")
                .long("quick")
                .help("Run the benchmarks straight away. For development only."))
        .arg(Arg::with_name("dry-run")
                .short("d")
                .long("dry-run")
                .help("Don't really run the benchmarks. For development only"))
        .arg(Arg::with_name("reboot")
                .long("reboot")
                .help("Reboot before each benchmark."))
        .get_matches();
    expb.quick(matches.is_present("quick"))
        .dry_run(matches.is_present("dry-run"))
        .reboot(matches.is_present("reboot"))
}
