# rekt

## WIP

Makes BUY/SELL-trade recommendations.

`rekt` is so-called because if you just blindly follow these recommendations,
neither knowing the how behind these calls nor trading coins of
[value](https://logicalgraphs.blogspot.com/2022/03/value-dialog.html), then
you're gonna get rekt, and it'll be all your fault.

Dependence on any automaton implicitly accepts the model within which the
automaton operates, and all models, by definition, are ... _wrong_ ... it's
just that some models inform us better than others.

_caveat emptor, ... LITERALLY!_

## HOW

The way `rekt` works is that it uses the
[EMA](https://www.investopedia.com/terms/e/ema.asp)20-vs-the tokens ratio,
thusly:

![EMA20 above ratio](imgs/ema-high.png)

* When the EMA20 is _above_ the ratio, this indicates the _current 'price'_
(ratio) is _below_ the trend, indicating a bargain: *BUY*!

![EMA20 below ratio](imgs/ema-low.png)

* When the EMA20 is _below_ the ratio, this indicates the _current 'price'_
(ratio) is _above_ the trend, indicating your token is overpriced: *SELL*!

## Assumption

Now, this whole approach depends on an assumption that both tokens of the
pivot are _tokens of value_. What do I mean by this? I mean that one token
may go down in value relative to the other token of the pivot for a while,
but (and here's the assumption), the first token will _eventually_ bounce back
and its value will return above the second's.

The assumption we have oscillating values of the token-pair pivoted.

## Risk

If the first token _never_ bounces back in value, then what happens is the
second token is drained, eventually, to zero, buying the first token that,
itself, devalues with respect to the second. I've seen this play out with
Ψ-LUNA, Ψ-ETH, GRAIL-ONE, and ECHO-rETH, where the protocol token (Ψ, GRAIL,
and ECHO) plummetted in value, and I got an inordinate amount of those devalued
tokens but lost my $LUNA, $ETH, and $ONE.

I was afraid, for more than a month, that the ARB/ETH pivot was a bad one,
because I kept trading $ETH for $ARB as $ARB's value continued to slump as
compared to $ETH's.

![ARB's multi-month value-plummet](imgs/arb-eth-EMA20.png)

*That's why* pivot-arbitrage/trading works only when _both_ tokens are tokens
of value.

How do you know a token is one of value? A rule of thumb is that it's a token
of value if you don't mind getting it in exchange for the base-token.

That's why BTC/ETH LPs are things of wonder! No matter which side is strong,
I don't mind getting a lot of the other token in balance.
