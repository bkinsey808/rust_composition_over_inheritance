# an attempt to implement Composition over Inheritance with rust
inspired by this amazing Fun Fun Function video: https://www.youtube.com/watch?v=wfMtDGfHWpA

## compile the binary crate

shorter compile time, for dev

    cargo build

## run the binary crate

    ./target/debug/fp_rust

## compile the binary crate with optimizations

longer compile time, for prod

    cargo build --release

## run the optimized binary crate

    ./target/release/fp_rust

## update all dependencies

    cargo update

## run tests

    cargo test

## develop watch

    cargo watch -x run

## watch tests

    cargo watch -x test
