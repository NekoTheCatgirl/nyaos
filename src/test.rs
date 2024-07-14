//! Tests are important, but can cause the kernel binary to be a bit bigger than desired!
//! Don't compile release kernel with the tests unless absolutely nessesary

mod box_alloc;

pub fn exec_tests() {
    run_test(box_alloc::BoxAlloc);
}

fn run_test(test: impl Test) {
    test.test();
}

// Trait required to register a new test
pub trait Test {
    fn test(self);
}