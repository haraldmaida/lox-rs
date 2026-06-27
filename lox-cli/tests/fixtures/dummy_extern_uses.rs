//! workaround for false positive 'unused crate dependencies' warnings until
//! Rust issue [#95513](https://github.com/rust-lang/rust/issues/95513) is fixed

use asserting as _;
use clap as _;
use lox_core as _;
use miette as _;
