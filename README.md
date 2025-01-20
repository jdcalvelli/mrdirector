# mrdirector

`mrdirector` is an extension package for the turbo game engine to help streamline the creation of games with branching narratives

warning: this project is in hyper omega alpha, so use at your own risk

## INTEGRATION

current steps for integration:

1. create a `scripts` folder at the root of the turbo project directory

2. within the `scripts` folder, create a `script.director` file, within which the story will be written

3. add `use mrdirector;` to the top of your lib.rs

4. insure that your turbo init gamestate struct contains a property of type `DirectorState`

5. ensure that within the `turbo::go!` macro is the `mrdirector::assess_current_line` function

6. should now be ready to rock and roll!

## DSL SYNTAX

### `<< [name]` is the name of a passage

### `>> [name]` is a send to the passage of corresponding name

- sends can live on their own, or in conjunction with a choice

### `]> [choice]` denotes a choice, director can handle up to 4 choices

- choice texts that follow ]> are written out to text box

- a set of choices choice must be followed by a set of sends on the next line, in the order that corresponds to the order of choices

	- e.g. line one: `]> choice one ]> choice two`

	- e.g. line two: `]> first send ]> second send`

- choices can be crossed out by prepending a ~ in front of the text of the choice

	- e.g. `]> ~text to be crossed out`

- choices that are displayed, but not actually available to the player, or are otherwise not intended to be selectable must send to the NULL send

	- e.g. `]> ~this choice shouldn't be clickable     ]> this choice should be`

	- e.g. `>> NULL                                    >> choice two send`
		- n.b. white space in between choices and/or sends are ignored, so feel free to space them out to be more legible

### `[char]: [text]` lines are statements

- currently, assumption is two characters, NOAH or MYLAN

	- to change character names at present, need to change the match statement in print_current_line.rs

- char determines portrait display

- text is written out to text boxes

### `! [cmd] / [arg]` are command lines that execute more complicated actions

- currently implemented:

	- WAIT / [TIME IN SECONDS] - pause dialogue for a number of seconds, for pacing purposes

### `-- end` denotes end of passage

- once the script game hits an -- end block, the game ends

- so all passages must send somewhere, or end the game

### `#` starts a comment

- keep in mind: the dsl is read at game runtime, so comments are not compiled out

- this means that comments can, at present, affect execution time, so use them intentionally

### blank lines are ignored

- keep in mind: the dsl is read at game runtime, so blank lines are not compiled out