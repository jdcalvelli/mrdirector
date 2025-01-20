use turbo::borsh;
use turbo::borsh::BorshSerialize;
use turbo::borsh::BorshDeserialize;

#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct DirectorState {
	pub speaking_char: String,
	pub lines: Vec<String>,
	pub current_line: usize,
	pub wait_timer: u16,
	pub script_finished: bool,
}

impl DirectorState {
	pub fn new(script_path: &str) -> Self {
		Self {
			script_finished: false,
			speaking_char: "".to_string(),
			lines: script_path.split("\n")
				.filter(|line| *line != "" && !line.starts_with("#"))
				.map(|line| line.to_string())
				.collect(),
			current_line: 0,
			wait_timer: 0,
		}
	}
}