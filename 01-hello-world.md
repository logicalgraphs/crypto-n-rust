# Chapter 1: Hello, world!

So, when I said we aren't going to do "Hello, world!" Why are we here doing
"Hello, world!"?

Well, it's because I'm an onery son o' a gun. Get used to it.

Okay, (relenting), it's because the "Hello, world!"-app is, well: useless. We
need to say "Hello, Johny!" or "Hello, " ... whomever we're "hello"-ing to.

Why?

"Hello, world!" is output only, non-configurable. A useless program.

But "Hello, "... whomever is both input and output and configurable.

From "Hello, geophf!" you can go in just about any direction we want to go.

Neat-o.

*A note on pedagogy*

This chapter, and subsequent chapters, are split into two parts: part I 
introduces a concept (yes, like "Hello, world!"); part II addresses the quiz
preceeding the chapter and how I solve it as a *Rustacean*.

## Part I: "Hello, world!"

The Rust community [already 
provides](https://doc.rust-lang.org/rust-by-example/hello.html)  how to solve 
"Hello, world!" itself.

Go there, do that exercise, then come back when you've "Hello, world!"ed 
yourself in Rust.

But, how does one say "Hello, John!" or "Hello, Jane!" or "Hello, Jamal!"?

Aye. There's the rub!

I'm not interested in typing in: "geophf" in the shell in response to the 
prompt:

> `What is your name? _`

I don't know many people who are. This isn't a world where the `Animal`-game,
programmed in BASIC, requires interactive shell queries and responses. We're
not in that world anymore.

Thank goodness.

OKAY!

So, in this rendition of "Hello, world!" we'll use the arguments to the program
as the names of the people (or, ... well: vampires or space aliens, I'm not
speciesist) (that, actually, is a word now) that we greet.

So, our program

`$ ./greet john jane jamal`

responds with:

```TXT
Hello, john!
Hello, jane!
Hello, jamal!
```

So, you already know how to write the "Hello, world!" program.

Write the `greet`-program.

When you complete this program and get the above result, check your approach
against mine.

[greet source](src/ch01/greet.rs)

## Part II: Discussion

Q: May you kindly review this answer and show how it does what it does, el geophf?
A: yes. 😎

### Line 1:

```Rust
// be nice to somebody, or ... somebodies, you know?
```

Q: What is Line 1?
A: A comment

Q: What does Line 1 do?
A: nothing

Q: Why put comments in code when they do nothing?
A1: no reason, comments are oftentimes wildly out-of-date and inaccurate.
A2: because I'm NICE! 😎

### Line 3:

```Rust
use std::env;
```

Q: What does Line 3 do?
A: imports the `std::env`-library

Q: What for?
A: so we can use the env functions in our code (see Line 6)

### Line 5:

```Rust
fn main() {
```

Q: What does Line 5 do?
A: Creates a function, `main()`.

Q: What is `main()` for?
A: This is the function run when the program is compiled.

Q: Why?
A: I don't answer 'why'-questions.

### Line 6:

```Rust
let args: Vec<_> = env::args().collect();
```

Q: Whoa!
A: Don't panic. Let's break this down.

Q: What does `let` do?
A: Creates a variable.

Q: What does `Vec<_>` mean?
A: That is the type for the variable, `args`: a vector.

Q: What is the rest?
A: it collects the command-line arguments and assigns them to the variable, 
`args`.

### Line 7:

```Rust
let (_, names) = args.split_at(1);
```

Q: What the ...?
A: Be cool. This is a tuple-assignment.

Q: What does a tuple-assignment do?
A: Assigns two variables to two values at the same time.

Q: Neat!
A: Innit?

Q: What is the first variable: '`_`'?
A: That's called the "don't care"-variable.

Q: Why?
A: Remember: I don't answer 'why'-questions.

Q: Oh. Okay. What does the "don't care"-variable mean?
A: It means that I don't care what this particular value is: I'm not going to 
use it.

Q: What is that first value that you're discarding?
A: The program name (in this case: "greet").

Q: Oh. What is the second value?
A: The arguments to the program on the command line. So, if we typed:

`$ ./greet Mary Sue`

then
`_` would be `["greet"]` and
`names` would be `["Mary", "Sue"]`

### Line 9:

```Rust
match names.len() {
```

Q: I think I get it. Line 9 matches the number of names to what happens below, 
right?
A: Correctamundo! 🎉 There are lots of ways to match in Rust. They have a 
[whole section on matching](https://doc.rust-lang.org/book/ch06-02-match.html).

### Lines 10-11:

```Rust
0 => { println!("Whom?"); }
_ => {
```

Q: I get that line 10 matches a 0-length to printing "`Whom?`", but what does 
line 11 mean?
A: The `_`-match pattern is the "don't care" match. It matches anything.

Q: Ah. Okay, back to line 10: why does `println` have a "`!`" following it?
A: That means that `println` isn't a function but a macro.

Q: What's the difference between a function and a macro in Rust?
A: Fair question! For now we can treat them as the same thing, but there's 
[a whole section in the Rust 
manual](https://doc.rust-lang.org/book/ch19-06-macros.html) that discusses 
macros and their purposes. 

### Lines 12-14:

```Rust
for name in names {
  println!("Hello, {}!", name);
}
```

Q: So, ... this loops over all the names, right?
A: Yeup.

Q: What is the "`{}`" in 13 for?
A: It tells `println!` where the name is to be placed in the output.

Q: Ah, okay. That's pretty much it, right?
A: Yeup!

Congratulations: your first program! This calls for celebration.

HAPFEN KĀKĒ! 🎂

![](imgs/ch01/cake-finished.jpg)
