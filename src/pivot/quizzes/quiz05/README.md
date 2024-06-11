# Pivot quiz 05

## query prices on @coingecko with Rust 

Now: 

* we have the coingecko API ids and the API-key (put that in the env 
`COIN_GECKO_API_KEY` var)
* we can GET from a REST endpoint

Write a Rust program to get the prices for the tokens listed in 
[pivots.csv](https://raw.githubusercontent.com/logicalgraphs/crypto-n-rust/pivot-quiz-04/data-files/csv/pivots.csv).

As part of your answer for this pop-quiz to query the @coingecko REST endpoint 
for token-prices, libraryitize the work you did before (which was to create a 
dictionary of @coingecko API ids to token-symbols from pivots.csv-data).
