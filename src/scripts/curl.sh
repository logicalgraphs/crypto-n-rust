OUT_FILE=$2

ENDPOINT=$1

curl -H "X-CMC_PRO_API_KEY: $COIN_MARKET_CAP_API_KEY" \
     -H "Accept: application/json" \
     -G $ENDPOINT > $OUT_FILE

echo "Downloaded to $OUT_FILE."
