// Copyright (c) 2019 Gabriela Alexandra Moldovan
// Copyright (c) 2019 King's College London.
// Created by the Software Development Team https://soft-dev.org
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0>, or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, or the UPL-1.0 license <http://opensource.org/licenses/UPL>
// at your option. This file may not be copied, modified, or distributed except according to those
// terms.

use crate::lang_impl::LangImpl;
use crate::limit::Limit;

use std::collections::HashMap;

/// The key of the path tag.
pub const TAG_PATH: &str = "path";

/// A collection of tags associated with a benchmark.
///
/// A tag is a key-value pair. It records both arbitrary values set by the user,
/// and the results of a benchmark.
pub type TagStore = HashMap<String, String>;

/// A benchmark, which consists of a set of tags, and a list of language
/// implementations the benchmark will be run on.
pub struct Benchmark<'a> {
    tags: TagStore,
    lang_impl: &'a dyn LangImpl,
    /// The stack size limit. `None` by default.
    pub stack_lim: Option<Limit>,
    /// The heap size limit. `None` by default.
    pub heap_lim: Option<Limit>,
}

impl<'a> Benchmark<'a> {
    /// Create a new benchmark with the specified path.
    pub fn new(path: &str, lang_impl: &'a dyn LangImpl) -> Benchmark<'a> {
        let b = Benchmark {
            tags: Default::default(),
            lang_impl,
            stack_lim: None,
            heap_lim: None,
        };
        // The path tag is mandatory (k2 can't run the benchmark without knowing
        // the path).
        b.tag("path", path)
    }

    /// The path of the benchmark.
    pub fn path(&self) -> &str {
        self.tags.get(TAG_PATH).expect("Benchmark path not set.")
    }

    /// Add tag `t` with value `val`.
    pub fn tag(mut self, t: &str, val: &str) -> Self {
        self.tags.insert(t.to_string(), val.to_string());
        self
    }

    /// Get the value of the tag with key `t`.
    fn tag_value(&self, t: &str) -> &str {
        &self
            .tags
            .get(t)
            .expect(&format!("Tag key {} doesn't have an associated value!", t))
    }

    /// Check if the value of the tag identified by `t` matches `val`.
    fn matches_tag(&self, t: &str, val: &str) -> bool {
        // This function could implement a more sophisticated check to decide whether
        // `val` is a match.
        self.tag_value(t) == val
    }

    pub fn stack_lim(mut self, stack_lim: Limit) -> Self {
        self.stack_lim = Some(stack_lim);
        self
    }

    pub fn heap_lim(mut self, heap_lim: Limit) -> Self {
        self.heap_lim = Some(heap_lim);
        self
    }
}
