# Pivot quiz 25 answer: dusk

## Pivot anatomy

First, let's start with the anatomy of a pivot.

What is a pivot?

Easy:

* an open pivot trade then the close pivot trade. So: two trades, basically.

![Pivot trades in action](imgs/02-pivot-anatomy.png)

Not so easy: 

* well, that's where we get to work, eh? 😎 

The good news is that ./dusk need only concern itself with open pivots (close pivots are closed, so are dead 💀 to ./dusk for disposition decisions), so our parsing problem becomes monadic(/simplified):

* accept (and parse) open pivots
* reject everything else (close pivots)

Further good news is that, yes, ./dusk will eventually open and close pivots (and report on its actions of same), but that is a down-the-road problem, so we don't need to formulate open pivot, with all that pesky data marshaling, we just need simply read and parse open pivots.

Okay, good. Let's get crackin'!

We need a sample data-file, which I've created [here](data/btc-paxg.tsv).

Right off the bat, we have funny numbers to parse, including

* dollar amounts with commas: $6,123.97
* δ-ranges: -21.13%

Let's see if we already have parsers for both.

... we do have parsers for both percentages and USD, so I wrote these two little
programs to test both parsers: 

* [answer25a_percentage.rs](answer25a_percentage.rs)
* [answer25b_usd.rs](answer25b_usd.rs)
