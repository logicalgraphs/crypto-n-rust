# types

A set of types used throughout the book. All these types are 
[csv-able](../csv_utils.rs).

```mermaid
classDiagram

	class CsvHeader~T~{
		<<Interface>>
		+header() String
	}
	class CsvWriter~T~{
		<<Interface>>
		+as_csv() String
		+ncols() usize
	}
        class Value~T~{
		<<Interface>>
                +value() T
        }
	class Tag~T~{
		-String tag
		-T value
                +untag() Pair~String, T~
	}
	class Dyad~T~{
		-StringPair pair
		-T value
		+unpair() Pair~StringPair, T~
	}
	class Index~T~{
		-usize ix
		-T value
		+mk_idx_offset(usize offset)
	}
        class Stamp~T~{
		-NaiveDate stamp
		-T value
		+date() NaiveDate
	}
	Tag ..|> CsvHeader
	Tag ..|> CsvWriter
	Tag ..|> Value
	Dyad ..|> CsvHeader
	Dyad ..|> CsvWriter
	Dyad ..|> Valu~
	Index ..|> CsvHeader
	Index ..|> CsvWriter
	Index ..|> Valu~
	Stamp ..|> CsvHeader
	Stamp ..|> CsvWriter
	Stamp ..|> Value
	link Value "values.rs"
	link CsvHeader "../csv_utils.rs"
	link CsvWriter "../csv_utils.rs"
	link Tag "tagged.rs"
	link Index "indexed.rs"
	link Dyad "dyadic.rs"
	link Stamp "stamped.rs"
```

* [Tag](tagged.rs)

> A tagged-type: a value that has a tag/associated-descriptor/key

* [Dyad](dyadic.rs)

> A tagged-type where the tag has a primary and secondary key.

* [Index](indexed.rs)

> A value with an `usize` index

* [Stamp](stamped.rs)

> A date-stamped value

* [Value](values.rs)

> Any type that carries a value, or: a comonad

