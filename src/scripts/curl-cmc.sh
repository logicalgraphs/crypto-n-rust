OUT_FILE=$2

ENDPOINT=https://pro-api.coinmarketcap.com/v1

curl -H "X-CMC_PRO_API_KEY: $COIN_MARKET_CAP_API_KEY" \
     -H "Accept: application/json" \
     -G $ENDPOINT/$1 > $OUT_FILE

echo "Wrote latest listings to $OUT_FILE."
