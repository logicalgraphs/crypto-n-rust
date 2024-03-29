
# this shell scripts assumes you have an exported variable, RUST_BOOK,
# that points to the root directory of this repository

export LE_DATE=`date +%Y-%m-%d`
DATA_DIR=$RUST_BOOK/data-files

export CSV_DIR=$DATA_DIR/csv
export ORCA_DIR=$DATA_DIR/ORCA
export FIN_DIR=$DATA_DIR/FIN
export PORT_DIR=$DATA_DIR/portfolio

export COLORS=$CSV_DIR/supported_colors.tsv
export CSV_LISTING=$DATA_DIR/csv/alles/cmc_listings.csv
export PORT_LISTING=$DATA_DIR/csv/portfolio/my_portfolio.csv
export HOLDINGS=$DATA_DIR/csv/portfolio_coins.lsv

export FIN_TICKERS=https://api.kujira.app/api/coingecko/tickers

# -- ALIASES -----------------------------------------------------------------

alias rusty="cd $RUST_BOOK"
alias cocaine="cd $RUST_BOOK/src/ch09/lsd; lsd $LE_DATE > data/stride.csv; cat data/stride.csv; git add data/stride.csv; cd -"
alias cmc="rusty; cmc_prices > $CSV_LISTING; cmc_filter $CSV_LISTING $HOLDINGS > $PORT_LISTING; git add $CSV_LISTING $PORT_LISTING; cat $PORT_LISTING; cd -"
alias kfc="rusty; kfin $LE_DATE $FIN_DIR/prices.tsv > /tmp/prices.tsv; mv /tmp/prices.tsv $FIN_DIR/prices.tsv; cat $FIN_DIR/prices.tsv; git add $FIN_DIR/prices.tsv; cd -"
alias orca="rusty; kfc; cillaz $LE_DATE $FIN_DIR/prices.tsv $ORCA_DIR/liquidations.lsv > $ORCA_DIR/report.csv; git add $ORCA_DIR/liquidations.lsv $ORCA_DIR/report.csv; git commit -m 'ORCA liquidations report'; cd -"
