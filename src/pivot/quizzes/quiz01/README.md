# Pivot quiz 01

## Reading from a REST endpoint

Build a trade-recommender based upon pivots 
(more deets on pivots 
[here](https://logicalgraphs.blogspot.com/p/prism.html)) in Rust.

* quiz 01: read 
[this REST endpoint](https://raw.githubusercontent.com/logicalgraphs/crypto-n-rust/pivot-quiz-01/data-files/csv/pivots.csv)

That's it. You can print out the response if you'd like.

Anybody want to counterchallenge me with #Elixir?

* [answer](answer.md)

## FUN FACT!

Did you know that you – YES, YOU! – can make your git repository data a (GET) 
REST endpoint?

IT'S TRUE! HERE'S HOW IS ONE EASY STEP!

Your CSV, TSV, or, really, any data on github has a 'raw'-counterpart that can 
be queried, just like any GET REST endpoint.

> *Caveat!* These raw-representations are NOT updated instantly on a `git push` 
but there is a timed-delay of 5 minutes (300 seconds), so: if you do a git push 
and don't see the updates in the raw data, DON'T PANIC! This is not a bug, but 
a feature from github.
