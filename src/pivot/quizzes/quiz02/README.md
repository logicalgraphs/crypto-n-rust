#Rust pop-quiz 02

Now that you can 
[read a REST endpoint](../quiz01/answer01.rs), shunt that functionality into 
its own module, `read_rest.rs`, that has the following function:


```Rust
read_pivots() -> ErrStr<Vec<String>>
```

![Run-off from calling `read_pivots()`](imgs/roff.jpg)

We'll be using this as a library function in follow-on quizzes. The library's 
name?

`swerve`.
