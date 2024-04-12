# ont

Creates an ontology of your cryptocurrency portfolio. 

![Portfolio as ontology](imgs/portfolio.png)

Your portfolio is recorded in the form of the following:

![portfolio data](imgs/data.png)

`$ echo <portfolio TSV> | ont`

n.b.: Don't pass in the headers, just the portfolio data.

This repository also has [a script](cypher/cmc.cyph) that represents which 
tokens are recorded from [CMC](https://coinmarketcap.com/), which then renders
interesting graphs, such as the following:

![CMC coins in my portfolio](imgs/cmc.png)

## Queries

Now that I've captured my portfolio data as a graph, there are several 
interesting representations that fall out, such as:

* How much is each protocol worth? / TVL?
* How much do I have in each blockchain?
* Which tokens are in which blockchains?
* How much do I have in each token?

## TODO

Further representations are possible by overlaying (meta-)graphs onto this
ontology. 

For example, 'purpose' or 'strategy' can be an aspect of investing. Am I 
investing into this token (on this protocol) to HODL? arb? leverage? loan?

O! the possibilities!

## Video

I do a walkthrough of using `./ont` and navigating a knowledge-graph in my video
[Portfolio as Ontology](https://www.youtube.com/watch?v=Hw6o-pJ-EsE).
