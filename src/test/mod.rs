#[test]
pub fn run_and_exit() {
    empty_test();

    std::process::exit(0);
}

#[test]
pub fn empty_test() {
    assert_eq!(true, true);
}
