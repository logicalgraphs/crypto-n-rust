# Pivot quiz 06

## Parsing JSON token-prices

We have the JSON back from @coingecko, no matter that it violates good 
data-structuring principles.

FUN!

1. Parse the JSON into:

(token-id,price)-pairs,

2. translate the token-id-values to token symbols, e.g.: 

* bitcoin -> BTC
* fantom -> FTM,

... etc.

Print the result as a report.

## Data

For those of you wishing to test your JSON-parser directly, I've added a 
[sample response of token-prices-as-JSON](https://raw.githubusercontent.com/logicalgraphs/crypto-n-rust/pivot-quiz-06/src/pivot/quizzes/quiz06/data/sample-response-token-prices.json)
from @coingecko REST /simple/price endpoint.

* [answer](answer.md)
