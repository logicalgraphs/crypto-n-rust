# Daily Reports

## Setup

Make sure you have `FIN_DIR` in your `env` as

* `export FIN_DIR=$CARGO_HOME/ch08/efficacy/data`

## Reports

1. The first report is the quotes of my crypto bag that have prices on
[coinmarketcap.com](https://coinmarketcap.com/).

Run `make`.

That was hard.

If you have the quotes already, simply run `make filter`.

The quotes are derived from tokens listed in 
[data-files/csv/portfolio_coins.lsv](data-files/csv/portfolio_coins.lsv).

2. [Daily quotes from Kujira FIN](src/ch08/bases/)

