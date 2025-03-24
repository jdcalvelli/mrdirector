//! # mrdirector
//! `mrdirector` is an extension package for the turbo game engine
//! to help streamline the creation of games with branching narratives
//! warning: this project is in hyper omega alpha, so use at your own risk

// for text_overlay

mod assess_current_line;
mod evaluate_choice;
mod evaluate_command;
mod print_current_line;
mod director_state;
mod text_overlay;

pub use assess_current_line::*;
pub use director_state::*;
pub use text_overlay::*;