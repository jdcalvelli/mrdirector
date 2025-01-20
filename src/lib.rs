//! # mrdirector
//! `mrdirector` is an extension package for the turbo game engine
//! to help streamline the creation of games with branching narratives
//! warning: this project is in hyper omega alpha, so use at your own risk

// need this for text overlay rs
use turbo::cam;
use turbo::canvas_size;

mod assess_current_line;
mod evaluate_choice;
mod evaluate_command;
mod print_current_line;
mod director_state;
mod text_overlay;

pub use assess_current_line::*;
pub use evaluate_choice::*;
pub use evaluate_command::*;
pub use print_current_line::*;
pub use director_state::*;
pub use text_overlay::*;