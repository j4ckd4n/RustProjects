use std::io::{self, Write};

fn flush_stdout() -> Result<(), io::Error> { io::stdout().flush() }
fn read_input(input: &mut String) -> Result<usize, io::Error> { io::stdin().read_line(input) }

fn main() {
  let mut input = String::new();

  loop {
    print!("> ");
    if let Err(e) = flush_stdout() { eprintln!("Error flushing stdout: {}", e); break; }

    input.clear();
    if let Err(e) = read_input(&mut input) { eprintln!("Error getting user input: {}", e); continue; }
    
    if input.trim() == "::exit" { break; }
    println!("[[ROBOT]]: {}", input);
  }
}