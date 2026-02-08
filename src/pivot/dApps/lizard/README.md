# lizard

What happens when I've missed a few days of harvesting data from @coingecko
using [`gecko`](../gecko)? This is where `lizard` comes in.

`lizard` checks `$QUOTES` and harvests all token-prices for all days missing.

![Bulk-load of token-prices](../../quizzes/quiz19/imgs/08b-pivots.png)

Now, 

* If there're no days missing, no data should be harvested.
* If there's only one day missing, `gecko` should be called (it's 1 REST call).
* If there're more than one day's worth of data missing, `lizard` gets to flex.
