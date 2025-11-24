
# this shell scripts assumes you have an exported variable, RUST_BOOK,
# that points to the root directory of this repository

export LE_DATE=`date +%Y-%m-%d`

DATA_DIR=$RUST_BOOK/data-files

export ORCA_DIR=$DATA_DIR/ORCA
export FIN_DIR=$DATA_DIR/FIN
export PORT_DIR=$DATA_DIR/blockaverse

export CSV_DIR=$DATA_DIR/csv

export COLORS=$CSV_DIR/supported_colors.tsv
export CSV_LISTING=$CSV_DIR/alles/cmc_listings.csv
export PORT_LISTING=$CSV_DIR/portfolio/my_portfolio.csv
export HOLDINGS=$CSV_DIR/portfolio_coins.lsv
export PIVOTS=$CSV_DIR/pivots.csv

export FIN_TICKERS=https://api.kujira.app/api/coingecko/tickers
export GECKO_API=https://api.coingecko.com/api/v3
export GECKO_PRICES=$GECKO_API/simple/price

# -- ALIASES -----------------------------------------------------------------

alias rusty="cd $RUST_BOOK"
alias cmc="rusty; cmc_prices > $CSV_LISTING; cmc_filter $CSV_LISTING $HOLDINGS > $PORT_LISTING; git add $CSV_LISTING $PORT_LISTING; cat $PORT_LISTING; cd -"

# -- PIVOTS -----------------------------------------------------------------

alias geck="rusty; gecko $LE_DATE >> $PIVOTS; git add $PIVOTS; tail -n 3 $PIVOTS; echo 'new pivot row added; REMEMBER TO PUSH TO REPOSITORY NOW!'; cd -"
alias oracle="rusty; status > $PORT_LISTING; git add $PORT_LISTING; cat $PORT_LISTING; cd -"

alias bae=geck
