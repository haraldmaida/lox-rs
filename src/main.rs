// workaround for false positive 'unused crate dependencies' warnings until
// Rust issue [#95513](https://github.com/rust-lang/rust/issues/95513) is fixed
mod dummy_extern_uses {
    #[cfg(test)]
    use asserting as _;
    #[cfg(test)]
    use proptest as _;
    use thiserror as _;
}

use lox_rs as _;

fn main() {
    todo!()
}
