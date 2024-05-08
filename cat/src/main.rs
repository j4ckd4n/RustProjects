use std::fs::File;
use std::env::{self};
use std::path::PathBuf;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()>{
  // read a file based on a provided path and print it to the console
  let args: Vec<String> = env::args().collect();
  let path = &args[1];
  // include support for relative paths
  let full_path = PathBuf::from(path);

  let file = File::open(full_path)?;
  let reader = BufReader::new(file);
  
  for line in reader.lines() {
    println!("{}", line?);
  }

  Ok(())
}