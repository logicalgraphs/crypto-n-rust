# BUIDL'n `despace`, `stamp`, and `tab2csv`

## BUIDL `despace`

1. execute:

`$ rustc despace.rs -o despace`

2. put `despace` into one of your execution directories. Mine went to 
`$HOME/bin`

## BUIDL `tab2csv`

Do the same thing for `tab2csv.rs`. You're smart. You'll figure it out.

## BUIDL `stamp`

`stamp` depends on the book's libraries, so make sure you have `$CARGO_HOME`
in your `env`, you know: by reading the
[README.md on the main page](../../README.md). Then you simply:

1. `$ cd $CARGO_HOME/ch04/stamp; cargo build`
2. `$ mv target/debug/stamp $HOME/bin` ... or to wherever you host your
executables.
