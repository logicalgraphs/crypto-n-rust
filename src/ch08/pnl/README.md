# PnL

PnL ('profit-n-loss') from yesterdæg's Kujira Trades

## Setup

0. [BUIDL `tab2csv`](../../ch04)
1. `$ cd $CARGO_HOME/ch08/pnl; cargo build`
2. establish [your initial assets](data/assets.csv)

## Run

1. copypasta your trades from yesterday. Do not include the last (totals) line.
`pnl` expects CSV with headers:


Trade,type,date,Asset,amount,for,amount,at price,valuation,tx_id,commission,fee,KUJI ($),fee ($),commission ($),ratio,inverted,premium,

where

* `Trade` is the index [1..]
* `type` is one of { BUY, SELL, LIQUIDATION}
* `Asset` is bought
* `for` is sold
* `at price` is sold-price
* `valuation` is formula: 
  * = `sold amount` * `at price` / `bought amount`
* `fee ($)` is formula
  * = `KUJI ($)` * `fee`
* `commisssion ($)` is formula:
  * = `commission` * `valuation`
* `ratio` is formula
  * = `valuation`
* `inverted` is formula
  * = 1 / `ratio`
* `premium` is the liquidation premium and only set if `type` is `LIQUIDATION`

For example:

![Kujira FIN trades, 2022-12-28](Kujira-FIN-trades-2022-12-28.png)

2. `$ echo *multiline copypasta'd trades | tab2csv > data/trades.csv`
3. `$ cargo run data/assets.csv data/trades.csv`

Your report should look something like (includes per-trade run-off):

![trade report, 2022-12-28](trade-report-2022-12-28.png)
