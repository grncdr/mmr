# mmr 

## What it is

`mmr` is a little CLI tool that reads a `.mmr` file in your current directory. If the file exists and hasn't been modified in a while, it will print it out. You can control how much is printed, and how long "a while" is with some command line arguments (run `mmr -h` to see what's available). Running `mmr` with no arguments will open the `.mmr` file in your current directory (regardless of whether it exists) in your `$EDITOR`.

## Install it

For now just `git clone` and `cargo install`. Real releases later maybe?

## Use it

Recommended usage is to run `mmr remind` before displaying your command prompt. This will cause mmr to automatically print reminders you haven't seen recently as you `cd` around the file system. 

### bash

Add this in your `.bashrc` or `.profile` or whatever (I still can't remember which files bash will use when):

```sh
PROMPT_COMMAND="mmr remind; $PROMPT_COMMAND"
```

### zsh

Add this in your `.zshrc` (or `.profile` etc):

```zsh
precmd() {
  mmr remind
}
```

If you already have a `precmd`, put `mmr remind` near the start.

## Other commands

```
mmr 0.1
Stephen Sugden <me@stephensugden.com>
Leave reminders for yourself in directories

USAGE:
    mmr [FLAGS] [SUBCOMMAND]

FLAGS:
    -h, --help         Prints help information
    -r, --recursive    Recursively search for .mmr file up to the root of the
                       filesystem instead of only the current directory.
    -V, --version      Prints version information

SUBCOMMANDS:
    add       Append a line to the .mmr file, creating it if necessary.
    edit      Open the .mmr file in your $EDITOR
    help      Prints this message or the help of the given subcommand(s)
    print     Print the contents of the .mmr file regardless of it's age
    remind    Check for a .mmr file and print the contents if it's old enough
```