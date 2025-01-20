use crate::DirectorState;
use crate::TextOverlay;
use crate::evaluate_choice;
use crate::evaluate_command;
use crate::print_current_line;

/// A function to retrieve and display the current line out of the `script.director` file.
///
/// Will probably most often be utilized inside of the `turbo::go!{}` macro.
///
/// First argument should be a `&mut` reference to the `DirectorState` object located in your local state.
///
/// Second argument should a `new` `TextOverlay` object.
///
/// # Example
/// ```
/// mrdirector::assess_current_line(
///     &mut state.director_state, 
///     mrdirector::TextOverlay::new(mrdirector::TextOverlayLocation::BOTTOM, 0x000000ff, 0xffffffff)
/// );
/// ```
pub fn assess_current_line(state:&mut DirectorState, text_overlay: TextOverlay) {
        match &state.lines[state.current_line] {
            line if line.starts_with("<<") || line.starts_with("#") || line == "" => {
                // is a passage, send, comment, or blank line, so increment on to next line
                state.current_line += 1;
            },
            line if line.starts_with(">>") => {
                // get divert text value
                let mut divert_text = state.lines[state.current_line].chars();
                divert_text.next();
                divert_text.next();
                // move to divert area!
                let new_knot_index: usize = state.lines
                    .iter()
                    .position(|line| *line == format!("{}{}", "<< ", divert_text.as_str().trim()))
                    .unwrap();
                state.current_line = new_knot_index;
            },
            line if line.starts_with("]>") => {
                // choice logic
                evaluate_choice::evaluate_choice(state, text_overlay);
            },
            line if line.starts_with("!") => {
                // command block
                evaluate_command::evaluate_command(state);
            },
            line if line.starts_with("-- end") => {
                // set character to none
                state.speaking_char = "".to_string();
                // game end situation?
                state.script_finished = true;
            }
            _ => {
                // regular line
                print_current_line::print_current_line(state, text_overlay);
            },
        }
    }