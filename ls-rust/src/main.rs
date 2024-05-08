use std::fs::{self, FileType, Permissions};
use std::env;
use std::io::{self};
use std::path::PathBuf;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use chrono::{DateTime, Local};

struct FileMetadata {
	path: PathBuf,
	file_type: FileType,
	permissions: Permissions,
	created: SystemTime,
	modified: SystemTime,
}

fn get_metadata(path: &PathBuf) -> io::Result<FileMetadata>{
	let metadata = fs::metadata(path)?;
	let file_type = metadata.file_type();
	let permissions = metadata.permissions();
	let created = metadata.created()?;
	let modified = metadata.modified()?;
	Ok(FileMetadata { path: path.clone(), file_type, permissions, created, modified })
}

fn main() -> io::Result<()> {
	let args: Vec<String> = env::args().collect();
	if args.len() < 2 {
		eprintln!("Usage: ls-rust <path>");
		std::process::exit(1);
	}
	let path = &args[1];

	let full_path = PathBuf::from(path);

	let entries = fs::read_dir(&full_path)?
		.map(|res| res.map(|e| e.path()))
		.collect::<Result<Vec<_>, io::Error>>()?;

	let mut files: Vec<FileMetadata> = Vec::new();

	for entry in entries {
		let metadata = get_metadata(&entry)?;
		files.push(metadata);
	}

	// calculate the longest path for formatting
	let longest_path = files.iter().map(|f| f.path.to_str().unwrap().len()).max().unwrap();

	// structure the output in a table format
	println!("{path:<width$} {:<20} {:<20} {:<20} {:<20}", "Type", "Read Only", "Created", "Modified", path="Path", width=longest_path);
	for file in files {
		let path = file.path.to_str().unwrap();
		let file_type = if file.file_type.is_dir() { "Directory" } else { "File" };
		let permissions = file.permissions;
		let created = DateTime::<Local>::from(UNIX_EPOCH + file.created.duration_since(UNIX_EPOCH).unwrap());
		let modified = DateTime::<Local>::from(UNIX_EPOCH + file.modified.duration_since(UNIX_EPOCH).unwrap());

		let created_str = created.format("%Y-%m-%d %H:%M:%S").to_string();
		let modified_str = modified.format("%Y-%m-%d %H:%M:%S").to_string();
		println!("{path:<width$} {:<20} {:<20} {:<20} {:<20}", file_type, permissions.readonly(), created_str, modified_str, path=path, width=longest_path);
	}

	Ok(())
}