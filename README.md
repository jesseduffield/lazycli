# lazycli

![Demo Animation](../assets/demo.gif?raw=true)

Turn static CLI commands into TUIs with ease

Test this out by cloning the repo and running `cargo run -- -- docker ps` or `cargo run -- -- git branch`. or install with `cargo install --path .` and then invoke as `lazycli`.

Right now some default keybindings are defined for common commands like `ls`, `docker ps`, `git branch`, `git status --short`, etc. But you can customise it for any commands you like! Just open the config file from within the program with `$` and start playing around.

lazycli is best suited towards any command-line program that spits out a list of items or a table. In your commands, simply refer to the column number by $0 for the first column, $1 for the second column, etc, and lazycli will do the rest.
