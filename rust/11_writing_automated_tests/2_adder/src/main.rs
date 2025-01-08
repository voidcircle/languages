// this file does not get provided with any integration tests by Rust.
// Rust assumes that the src/main.rs file is a binary crate, which is a small amount of code that
// should be run by itself. Only the src/lib.rs file that exposes public APIs should be tested with
// integration tests. Rust expects that the src/main.rs code imports the APIs from the src/lib.rs,
// which is tested with integration tests, which means src/main.rs file should also be ok with
// being run without any bugs.
