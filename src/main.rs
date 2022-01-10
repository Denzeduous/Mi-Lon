#[path = "./terminal/terminal.rs"]
mod terminal;

use clap::{Arg, App};

fn main() {
	let matches = App::new("Mi Lon")
		.version("0.1.0")
		.author("Denzeduous <denzeduous@tutanota.com>")
		.about("A simple shortcut-based text editor.")
		.arg(Arg::new("PATH")
			 .help("Path of the file or directory to open")
			 .required(true)
			 .index(1))
		.get_matches();

	terminal::start_terminal(String::from(matches.value_of("PATH").unwrap()));
}