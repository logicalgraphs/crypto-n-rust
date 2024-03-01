# top_order_books

![top order books by volume](imgs/top-order-books.png)

Computes the top order books by 24-hour volume from @TeamKujira FIN order book
data obtained from the 
[tickers REST endpoint](https://api.kujira.app/api/coingecko/tickers).

<code>$ ./top_order_books [--raw] <date></code>

The <code>--raw</code>-option shows all the order books and their 24h-volumes.

## TODO

The volumes are messed up, mang! I had assumed all volumes were in USD. That
is not the case for some order books: the volumes are in
<code>target_currency</code>, so, when <code>target_currency</code> is
<code>$axlUSDC</code> or another similar stable (*cough* <code>$USK</code>),
then the volumes are accurate. When the volumes are in, say
<code>$wETH</code>, I'm hella-off!
