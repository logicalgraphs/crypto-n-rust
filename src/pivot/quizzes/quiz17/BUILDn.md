# HOWTO BUIDL `lizard`

Okay, let's solve this problem.

The problem has a couple of domains. One is to fetch the data from the 
@coingecko REST endpoint. The other to process the fetched data.

Let's separate concerns.

Focusing on processing, I download a sample [data-set](data/eth.json).

Let's examine the structure of the JSON.

The response comes with prices, market caps, and volumes.

![coingecko JSON response of chart-data](imgs/03a-json-structure.png)

I'm concentrating on the prices.

Gracious! what a mess!

Prices come stamped with a (bigint) timestamp, but are these values labeled at 
all?

![stamped prices as an undifferentiated list](imgs/03b-stamped-price.png)

Of course not! They're in a LIST?!??

Data structures 101: don't do this --^

or:

Label each datum, that is: don't use a list index to TYPE YOUR DATA?

e.g.:

```JSON
prices: [
 { "quote": { "usd": 3420.796671 },
    "stamp": 1721340740565
 }, ...]
```

The reason why JSON exists is for an uniform SELF-DESCRIBING data-exchange.

If you're not going to use JSON for its intended purpose, then pass the data 
back as CSV, ... WHICH IS STILL SELF-DESCRIBING WHEN YOU INCLUDE THE HEADER-ROW!

Why are we still permitting children, who want to save three bytes, so end up 
ADDING WORK, be allowed anywhere near ICDs?

All righty, then. We have what we have. How do we approach solving this problem?

Two common approaches.

1. top-down: we ingest everything, then refine
2. bottom-up: we make one element work, then take on more and more until 
everything's ingested.

I'm going with top-down.

First thing we need to do, top-down, is to ingest the JSON from file to see 
that we have the sections as Rust-values.

Fortunately, we have 
[an example that gets us most of the way there](https://docs.rs/serde_json/latest/serde_json/de/fn.from_reader.html)

Let's implement this and see what we get.

... _implement, implement, implement_ ...

What we get is this:

![Coingecko response JSON read](imgs/04-json-read.png)

You see, top-down approach, that my data-structure in
[this first solution that simply reads the JSON and parses the results
into sections](answer17a_sections.rs) is simple:

```Rust
type StampedData = Vec<Vec<f64>>;
type Chart = HashMap<String, StampedData>;
```

Simply a `HashMap` with a `Vec<Vec<f64>>` as values to each section-key.

A simple result, so now let's refine that result until we have workable
data-sets.
