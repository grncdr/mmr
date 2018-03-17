# mmr 

_member?_
![Member Berry illustration](https://raw.githubusercontent.com/msolomonTMG/echo-memberberries/master/member%20berries.png)

_Character & image copyright South Park Studios, don't sue me._

## What it is

`mmr` is a little CLI file that reads a `.mmr` file in your current directory. If the file exists and hasn't been modified in a while, it will print it out. You can control how much is printed, and how long "a while" is with some command line arguments (run `mmr -h` to see what's available). Running `mmr` with no arguments will open the `.mmr` file in your current directory (regardless of whether it exists) in your `$EDITOR`.

Advised usage is to stick it in front of your prompt like this (bash) `PS1="$(mmr remind)$PS1"`. This will cause mmr to automatically print reminders you haven't seen recently as you `cd` around the file system. 
