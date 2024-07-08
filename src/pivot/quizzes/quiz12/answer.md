# Pivot quiz 12 answer: EMA

## Computing the EMA (Exponential Movie Average) around collected pivot data

### [answer](answer12.rs)

The brunt of the work fell upon the 
[EMAs-make-function in types.rs](https://github.com/logicalgraphs/crypto-n-rust/blob/main/src/pivot/swerve/types.rs#L107-L142).

Now, I'm sure using comonads would obviate the need for the `prev` and `emas0`
State(/'mutable') variables, but Rust doesn't have a good comonad 
implementation, so I fell back to using these State-variables and the good
ol' `for`-loop.

p.s.: most of the work was pushed to 
[swerve/snarf.rs `snarf_emas()`](https://github.com/logicalgraphs/crypto-n-rust/blob/pivot-quiz-15/src/pivot/swerve/snarf.rs#L45-L76)

### TradingView charts

Also, during this exercise, I explored various charting solutions and fell,
finally, to TradingView. They provide so much functionality and make it easy
to use.

![TradingView ETH/BTC chart with calculated EMA-20 smooth-line](tv-01-ema.png)

Now, am I happy using a commercial solution, instead of a free one?

Hellz, to-the-ya!

Their free- tier simply requires attribution, and they have code-samples that
show how to do just that. Cool beans for me!
