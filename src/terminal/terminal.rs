mod state;

pub fn start_terminal<'state>(path_str: String) {
	let mut _state = state::State::new(path_str);
}
