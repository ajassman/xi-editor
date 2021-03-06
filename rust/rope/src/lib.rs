// Copyright 2016 Google Inc. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Trees for text.

#![cfg_attr(feature = "cargo-clippy", allow(
    collapsible_if,
    len_without_is_empty,
    many_single_char_names,
    needless_range_loop,
    new_without_default_derive,
    should_implement_trait,
    wrong_self_convention,
))]

extern crate bytecount;
extern crate memchr;
extern crate serde;
#[macro_use] extern crate serde_derive;
#[cfg(test)] extern crate serde_test;
#[cfg(test)] extern crate serde_json;

pub mod tree;
pub mod breaks;
pub mod interval;
pub mod delta;
pub mod rope;
pub mod spans;
pub mod multiset;
pub mod engine;
pub mod find;
#[cfg(test)]
mod test_helpers;

pub use rope::Rope;
