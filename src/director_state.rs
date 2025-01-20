use turbo::borsh;
use turbo::borsh::BorshSerialize;
use turbo::borsh::BorshDeserialize;

/// A state object to hold information required by the `mrdirector` package.
///
/// The local state object created in `turbo init!{}` should contain an instance of this.
///
/// # Example
/// ```rust
/// turbo::init! {
/// 	struct GameState {
/// 		...
/// 		...
/// 		director_state: mrdirector::DirectorState
/// 	} = Self {
/// 		...
/// 		...
/// 		director_state: mrdirector::DirectorState::new(std::include_str!("../scripts/script.director"))
/// 	}
/// }
/// ```
#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct DirectorState {
	pub speaking_char: String,
	pub lines: Vec<String>,
	pub current_line: usize,
	pub wait_timer: u16,
	pub script_finished: bool,
}

impl DirectorState {
	/// This function takes an argument a `&' static str` which is the path to your `script.director` file.
	///
	/// It returns a DirectorState object.
	///
	/// # Example
	/// ```rust
	/// mrdirector::DirectorState::new(std::include_str!("../scripts/script.director"))
	/// ```
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