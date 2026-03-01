# gecko

`gecko` reads the token-prices from coingecko REST endpoint.

`$ gecko <date> [branch]`

where:

* `<date>` (required) is the date for which you want to stamp the quotes fetched
now. `$LE_DATE` renders today's date.
* `[branch]` is the branch of the repository from which to fetch which quotes
to fetch (defaulted to `main`).

![gecko run](imgs/gecko.png)

"Which token-prices?" you ask.

Fair question.

It reads the token-prices for the coingecko API IDs listed in
[`$QUOTES`](../../../data-files/csv/quotes.csv)

As this dApp accesses the coingecko API, an authetication is required. I put
the authentication-token into the `COIN_GECKO_API_KEY` environmental variable.

## Construction

I demonstrate building this dApp through a [series of quizzes](../../quizzes).
Working your way through these quizzes talks to its design.

