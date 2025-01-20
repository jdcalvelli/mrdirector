use turbo::prelude::*;
use crate::DirectorState;
use crate::TextOverlay;

pub fn print_current_line(state:&mut DirectorState, text_overlay: TextOverlay) {
    // split at the : to get character and line
    let statement: Vec<String> = state.lines[state.current_line]
        .split(":")
        .filter(|&element| element != "")
        .map(|element| element.trim().to_string())
        .collect();
    // determine speaking character
    state.speaking_char = statement[0].clone();
    
    // draw textbox
    text_overlay.render_textbox(&statement);
    
    // move this maybe into a bespoke input checker?
    if gamepad(0).start.just_pressed() {
        state.current_line += 1;
    }
}