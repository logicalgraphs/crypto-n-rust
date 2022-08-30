# Chapter 3: Modules

In this chapter we're going to be talking about using code with modules. We're
going to be creating a module, `utils.rs`, that has two functions what we'll
be able to use in the programs we write (as opposed to writing the same 
functionality for each program).

So, let's get started and just write the thing, and see how far we make it.
After, we'll review the code of the module we've created.

From what you've learned in (chapter 2)[02-summer.md], ...

Create a module `utils.rs` that exports 2 functions:

* `get_args()` that gets the command line arguments 
* `get_nums()` that gets the arguments as numbers.

Then:

Rewrite (`summer.rs`)[src/ch02/summer.rs], using `get_nums()` from the
`utils.rs`-module you've created.

Answers *(remember, don't peek until you've tried to solve this yourself)*:

* (`utils.rs`)[src/ch03/utils.rs]
* (`winter.rs`)[src/ch03/winter.rs]

## Discussion
