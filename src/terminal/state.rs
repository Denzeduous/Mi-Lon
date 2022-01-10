use std::io;
use std::path::{Path, PathBuf};

pub struct State {
	stdout      : io::Stdout,
	path        : PathBuf,
	is_directory: bool,
	curr_file   : PathBuf,
	longest_line: u64,        // Length of the longest line
	offset      : (u64, u64), // Tuple of (vertical, horizontal) for the total offset from top left. Top left is (1, 1)
	len         : (u16, u16), // Tuple of (columns, rows) from the crossterm::terminal::size function
}

impl State {
	pub fn new(path_str: String) -> State {
		let path = Path::new(&path_str);

		if !path.exists() {
			println!("Path {0} either does not exists or you do not have permissions to edit it.", path.to_string_lossy());
			std::process::exit(1);
		}

		let curr_file_str = if path.is_dir() { String::new() } else { path.to_str().unwrap().to_string() };

		let mut curr_file = PathBuf::new();
		curr_file.push(Path::new(&curr_file_str));

		let mut main_path = PathBuf::new();
		main_path.push(path);

		let state = State {
			stdout       : io::stdout(),
			path         : main_path,
			is_directory : path.is_dir(),
			curr_file    : curr_file,
			longest_line : 0,
			offset       : (1, 1),
			len          : (0, 0),
		};

		return state;
	}
}