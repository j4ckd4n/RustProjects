use std::env;
fn main() {
  let args: Vec<String> = env::args().collect();
  // combine all arguments but the first one into a single string
  let output = args[1..].join(" ");
  println!("{}", output);
}