use basics::syntax::*;

#[test]
fn test_hello() {
    let results = hello_basics();
    let expected = "Hello Basics!";
    assert_eq!(results, expected, "The function did not return the expected value")
}
