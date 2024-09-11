# Pivot quiz 19

## Merge `Table`s solution

How to solve this?

Let's take a piecewise-approach, from the top-down.

The first thing is we need a pair of tables (to merge) and we need a 
`merge()`-function, so let's do that, implementing them with ... get this: 
`panic!()` !?!??!??

![`main()` with `panic!()`](imgs/01a-main-with-panic.png)

![`table_utils::merge()` as `panic!()`](imgs/01b-table-utils-with-panic.png)

Yup, just like in the old Smalltalk days. 😎
