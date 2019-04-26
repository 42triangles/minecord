# minecord

Minesweeper for Discord

Remember to compile it with `cargo build --release`.

The resulting binary (in `target/release/minecord`) can be run with the following option (also documented by `structopt`):
* `-o`: Open up the safest spot already
* width: The width
* height: The height
* count: The amount of mines to place
* bomb emote: The emote to use for bombs. This option is optional, it defaults to `:bomb:`. Please note that not all emotes work well, since they have to appear with the same width as the number emotes for the whole thing to display correctly.

## Roadmap
* ~~Create some sort of fix for resulting puzzles that aren't solvable~~ This is a thing to generate minesweeper games for Discord. I'm not going to do that.
