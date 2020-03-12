use rust_algorithm::gcd_of_strings;

#[test]
fn test() {
    let result = gcd_of_strings(String::from("ABCABC"), String::from("ABC"));
    println!("{}", result.unwrap());
    // assert_eq!("hello", hello());
}