# book

> (odd title for a library, innit?)

## a set of common utilities around core functionalities

### [matrix_utils](matrix_utils.rs)

`Matrix<T>` is aliased to `Vec<Vec<T>>` with an ingest function: 
`from_lines` and a (coding-)efficient way to view columns: `col`.

### [table_utils](table_utils.rs)

We present the `Table`-type, an enhancement of the `Matrix`-type in that both
columns and rows have their own respective types and are indexed(-into) with
such.
