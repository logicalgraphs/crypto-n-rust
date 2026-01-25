# book

> (odd title for a library, innit?)

## a set of common utilities around core functionalities

### [types](types)

### [json_utils](json_utils.rs)

Both for the deserialization and serialization of JSON data/JavaScript objects

### [matrix_utils](matrix_utils.rs)

`Matrix<T>` is aliased to `Vec<Vec<T>>` with an ingest function: 
`from_lines` and a (coding-)efficient way to view columns: `col`. 

Do you need to transpose a `Matrix`? No problem: `transpose` has got you 
covered.

### [table_utils](table_utils.rs)

We present the `Table`-type, an enhancement of the `Matrix`-type in that both
columns and rows have their own respective types and are indexed(-into) with
such.

The `Table`-type also has `cols` and has its own `row_filter`.

For example, let's say you have 
[token-price data](../../../data-files/csv/pivots.csv) extending beyond the last
100 days, but you only wish to do analysis on the last 10 days. To get the last
10 days of data, you would do something like this:

```Rust
use std::ops::Sub;
use chrono::{Days,NaiveDate};       // "0.4.38"
use book::table_utils::row_filter;

let days = Days::new(10);
let start = today.sub(days);

fn in_range(d: &NaiveDate) -> impl Fn(&NaiveDate) -> bool + '_ {
   |date| { date.ge(d) }
}
let domain = row_filter(in_range(start), &table);
```

## Revisions

* 1.02, 2026-01-25: Separated types to its own set of modules; added type-tests
* 1.01, 2026-01-07: Added first test (`ht_empty_list`) to 
[list_utils](list_utils.rs). It's all uphill, sry: sunshine and daffodils
from here!

