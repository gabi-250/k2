// Copyright (c) 2019 Gabriela Alexandra Moldovan
// Copyright (c) 2019 King's College London.
// Created by the Software Development Team https://soft-dev.org
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0>, or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, or the UPL-1.0 license <http://opensource.org/licenses/UPL>
// at your option. This file may not be copied, modified, or distributed except according to those
// terms.

/// Return the absolute path of `bin_name` by searching ${PATH}.
pub fn find_executable(bin_name: &str) -> String {
    which::which(bin_name)
        .expect(&format!("Could not find {}.", bin_name))
        .to_str()
        .expect("Path must be a utf-8 string.")
        .into()
}
