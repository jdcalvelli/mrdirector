use turbo::prelude::*;

/// Options as to where to put the textbox used to display the current line.
///
/// TOP: Top of the screen as defined by `[resolution]` in `turbo::cfg!`.
///
/// BOTTOM: Bottom of the screen as defined by `[resolution]` in `turbo::cfg!`. More traditional Visual Novel style.
pub enum TextOverlayLocation {
	TOP,
	BOTTOM
}

/// An object to hold data associated with the TextOverlay that displays the assessed lines from the `script.director` file.
///
/// Used in conjunction with the `assess_current_line` function.
///
/// # Example
/// mrdirector::assess_current_line(
///     &mut state.director_state, 
///     mrdirector::TextOverlay::new(mrdirector::TextOverlayLocation::BOTTOM, 0x000000ff, 0xffffffff)
/// );
pub struct TextOverlay {
	pub location: TextOverlayLocation,
	pub overlay_color: u32,
	pub text_color: u32,
	tb_padding: u16,
	choice_options: (String, String, String, String),
}

impl TextOverlay {
	/// This function returns a TextOverlay object to be used with the `assess_current_line` function.
	pub fn new(loc:TextOverlayLocation, overlay_color_arg:u32, text_color_arg:u32) -> Self {
		Self {
			location: loc,
			overlay_color: overlay_color_arg,
			text_color: text_color_arg,
			tb_padding: 8,
			choice_options: ("<".to_string(), ">".to_string(), "^".to_string(), "v".to_string())
		}
	}
	pub fn render_textbox(self, dialogue: &Vec<String>) -> () {
		let tb_x: u16;
		let tb_y: u16;
		
		match self.location {
			TextOverlayLocation::TOP => {
				tb_x = 0;
				tb_y = 0;
			},
			TextOverlayLocation::BOTTOM => {
				tb_x = 0;
				tb_y = (resolution().1 as u16) - (23 + 2 * self.tb_padding);
			}
		}
		
		rect!(
			w = resolution().0 as u16,
			h = 23 + 2 * self.tb_padding,
			x = tb_x,
			y = tb_y,
			color = self.overlay_color
		);
		text!(
			dialogue[1].as_str(),
			x = tb_x + self.tb_padding,
			y = tb_y + self.tb_padding,
			color = self.text_color,
			font = "small"
		);
	}
	pub fn render_choice_textbox(self, choices: &Vec<String>) -> () {
		let tb_x: u16;
		let tb_y: u16;
		
		match self.location {
			TextOverlayLocation::TOP => {
				tb_x = 0;
				tb_y = 0;
			},
			TextOverlayLocation::BOTTOM => {
				tb_x = 0;
				tb_y = (resolution().1 as u16) - (23 + 2 * self.tb_padding);
			}
		}
		
		rect!(
			w = resolution().0 as u16,
			h = 23 + 2 * self.tb_padding,
			x = tb_x,
			y = tb_y,
			color = self.overlay_color
		);
		
		choices.iter().enumerate().for_each(|(index, choice)| {
			
			let choice_sanitized: &str;
			if choice.starts_with("~") {
				// remove the ~ for display
				choice_sanitized = &choice[1..choice.len()];
			}
			else {
				choice_sanitized = choice;
			}
			
			match index {
				0 => {
					text!(
						"{} {}", self.choice_options.0, choice_sanitized;
						x = tb_x + self.tb_padding,
						y = tb_y + self.tb_padding,
						color = self.text_color,
						font = "medium"
					);
					
					if choice.starts_with("~") {
						path!(
							start = (tb_x + self.tb_padding, tb_y + self.tb_padding + 3), 
							end = (resolution().0 as u16 / 2 - self.tb_padding, tb_y + self.tb_padding + 3),
							size = 1,
							color = self.text_color,
						);
					}
					
				},
				1 => {
					text!(
						"{} {}", self.choice_options.1, choice_sanitized;
						x = tb_x + self.tb_padding,
						y = tb_y + 16 + self.tb_padding,
						color = self.text_color,
						font = "medium"
					);
					
					if choice.starts_with("~") {
						path!(
							start = (tb_x + self.tb_padding, tb_y + 16 + self.tb_padding + 3), 
							end = (resolution().0 as u16 / 2 - self.tb_padding, tb_y + 16 + self.tb_padding + 3),
							size = 1,
							color = self.text_color,
						);
					}
					
				},
				2 => {
					text!(
						"{} {}", self.choice_options.2, choice_sanitized;
						x = resolution().0 as u16 / 2 + self.tb_padding, 
						y = tb_y + self.tb_padding,
						color = self.text_color,
						font = "medium"
					);
					
					if choice.starts_with("~") {
						path!(
							start = (resolution().0 as u16 / 2 + self.tb_padding, tb_y + self.tb_padding + 3), 
							end = (resolution().0 as u16 - self.tb_padding, tb_y + self.tb_padding + 3),
							size = 1,
							color = self.text_color,
						);
					}
				},
				3 => {
					text!(
						"{} {}", self.choice_options.3, choice_sanitized;
						x = resolution().0 as u16 / 2 + self.tb_padding, 
						y = tb_y + 16 + self.tb_padding,
						color = self.text_color,
						font = "medium"
					);
					
					if choice.starts_with("~") {
						path!(
							start = (resolution().0 as u16 + self.tb_padding, tb_y + 16 + self.tb_padding + 3), 
							end = (resolution().0 as u16 - self.tb_padding, tb_y + 16 + self.tb_padding + 3),
							size = 1,
							color = self.text_color,
						);
					}
				},
				_ => {panic!("CRITICAL: Number of choices exceeds allowed 4.")},
			}
		});
	}
}