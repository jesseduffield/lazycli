# lazycli

![Demo Animation](../assets/demo.gif?raw=true)

Turn static CLI commands into TUIs with ease

Demo:

[<img src="../assets/demo-thumbnail.png" width="380px">](https://www.youtube.com/watch?v=CRzcOpjuYSs&ab_channel=JesseDuffield)

## Usage

Pick a command that spits out either a list or table of content, like `ls`, `docker ps`, `git branch`, or `git status --short`. Then run `lazygit -- <YOUR COMMAND>`
```
lazycli -- ls
```

If you find yourself often using lazycli with a specific command, you can easily alias it like so:

```
echo "alias lcd=\"lazycli -- docker ps\"" >> ~/.zshrc
source ~/.zshrc
lcd
```

Right now some default keybindings are defined for common commands like `ls`, `docker ps`, `git branch`, `git status --short`, etc. But you can customise it for any commands you like! Just open the config file from within the program with `$` and start playing around.

lazycli is best suited towards any command-line program that spits out a list of items or a table. In your commands, simply refer to the column number by $0 for the first column, $1 for the second column, etc, and lazycli will do the rest. There are plenty of starting examples in the config that you'll be able to draw from.

## Installation

### Via Cargo

```
cargo install lazycli
```


### Via binary

Download the binary from the [Releases Page](https://github.com/jesseduffield/lazycli/releases)


### Building from source

1) clone the repo:
```
git clone https://github.com/jesseduffield/lazycli.git
```
2) install
```
cargo install --path .
```
3) run
```
lazycli -- ls
```t

## QandA
* Q: Isn't this what fzf does?
* A: Not quite: fzf requires you to know the command ahead of time whereas lazycli lets you run commands after presenting you the data, and the content is refreshed after you run the command rather than the program closing (admittedly I haven't used fzf but I'm pretty sure that's all correct).
