//! workaround for false positive 'unused crate dependencies' warnings until
//! Rust issue [#95513](https://github.com/rust-lang/rust/issues/95513) is fixed

use anyhow as _;
use asserting as _;
use clap as _;
use lox_rs as _;
use proptest as _;
use thiserror as _;
use utf8_chars as _;
