# HOWTO

... cmc_prices

1. I've already created the project with `crate new cmc_prices`

What I do is copy the Cargo template into the `cmc_prices`-project:

... then I use the book crate. The `Cargo.toml` is included in the repository.

Then you should be able to build and run your application:

```SHELL
$ cd cmc_prices; cargo run ../cmc_listings_sample.json
```

... and see the following output:

```TXT
There are 2 coins.

date,cmc_id,rank,symbol,name,price
2021-03-21,1,1,BTC,Bitcoin,$57220.49
2021-03-21,1027,2,ETH,Ethereum,$1787.62
```

Doit-Toit!
