mod encrypt;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        eprintln!("Usage: {} [d|e] <input_file> <output_file> <password>", args[0]);
        return;
    }

    let mode: &String = &args[1];
    let input_file: &String = &args[2];
    let output_file: &String = &args[3];
    let password: &String = &args[4];

    if mode.eq("e") {
        match encrypt::encrypt_file(input_file, output_file, password) {
            Ok(_) => println!("File encrypted successfully!"),
            Err(e) => eprintln!("[ERROR] Error encrypting file: {}", e),
        }
    } else if mode.eq("d") {
        match encrypt::decrypt_file(input_file, output_file, password) {
            Ok(_) => println!("File decrypted successfully!"),
            Err(e) => eprintln!("[ERROR] Error decrypting file: {}", e),
        }
    } else {
        eprintln!("Invalid mode: {}", mode);
    }
}


