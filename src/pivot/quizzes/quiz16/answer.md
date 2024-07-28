# Pivot Rust pop-quiz

## Last updated-date

The solution presented itself because of two things:

* I already have functionality 
[to read in the pivots-table](https://github.com/logicalgraphs/crypto-n-rust/blob/pivot-quiz-16/src/pivot/swerve/snarf.rs#L37-L49)
(I did have to liberate that functionality from the `snarf_emas()`-function,
... but that function was doing too much by itself, anyway).
* `NaiveDate` implements `Sub` (returning a `TimeDelta`), so I simply apply
`num_days()` to that result.

The answer – voilà – manifests itself!

![Most recent update on `$PIVOTS`](imgs/most-recent-update.png)
