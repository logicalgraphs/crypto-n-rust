# pivot

A very slow 'arbitrage'-trade from one token, back to that token, via a 
pivot-token.

## Determinor

The determinor to execute the trade is the
[EMA](https://www.investopedia.com/terms/e/ema.asp)-20 ratio between the
traded-token and the pivot-token.

The determinor of how much to trade is the ratio between current δ to the
max (or min) δ.

The practice of this type of trading which I call pivot-trading or 
pivot-arbitrage, is that: once a day, I check the EMA-20s then execute
the trades based upon those indicators.

## Efficacy

The way that I know this strategy /works/ is that I compare like-for-like:

If I traded 10 $AVAX for, say, 16,000 $QI on day x, then, days, weeks, or 
months later, on day y, ...

> (I did say this was 'very slow,' didn't I?)

I trade-back the 16,000 $QI for 11.5 $AVAX, I then have an ROI of 15%

> (11.5 - 10) / 10 = 15%

and the APR of ROI * 365 / (y - x)

This way I measure the success of my pivot-arbitrage

## Quizzes

I show how to BUIDL this system from the ground up in a series of
[quizzes](quizzes).