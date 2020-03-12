use rust_algorithm::gcd_of_strings;

fn main () {
  let result = gcd_of_strings(String::from("ABCABC"), String::from("ABC"));
  println!("{}", result.unwrap());
}
