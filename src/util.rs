// Copyright (c) 2019 Gabriela Alexandra Moldovan
// Copyright (c) 2019 King's College London.
// Created by the Software Development Team https://soft-dev.org
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0>, or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, or the UPL-1.0 license <http://opensource.org/licenses/UPL>
// at your option. This file may not be copied, modified, or distributed except according to those
// terms.

use crate::error::K2Error;

use libc::c_char;
use std::{ffi, ptr};

/// Return the absolute path of `bin_name` by searching ${PATH}.
pub fn find_executable(bin_name: &str) -> String {
    which::which(bin_name)
        .unwrap_or_else(|_| panic!("Could not find {}.", bin_name))
        .to_str()
        .expect("Path must be a utf-8 string.")
        .into()
}

/// Reboot, if `hardware_reboot` is `true`. Otherwise, replace the current process
/// with a fresh copy of itself.
pub fn reboot(hardware_reboot: bool) -> K2Error {
    if hardware_reboot {
        unimplemented!("reboot")
    } else {
        let args = std::env::args();
        let mut cstrs = Vec::with_capacity(args.len());
        for arg in args {
            cstrs.push(ffi::CString::new(arg).unwrap());
        }
        let mut argv: Vec<*const c_char> = cstrs.iter().map(|arg| arg.as_ptr()).collect();
        argv.push(ptr::null());
        unsafe { libc::execv(argv[0], argv.as_ptr()) };
        K2Error::Unknown
    }
}

/// Return the number of digits in `value`.
pub fn num_digits(value: usize) -> usize {
    if value == 0 {
        1
    } else {
        (value as f64).log10().floor() as usize + 1
    }
}
