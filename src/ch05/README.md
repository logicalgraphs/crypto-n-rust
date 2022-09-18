# HOWTO

... cmc_prices

1. `crate new cmc_prices`

What I do is copy the Cargo template into the `cmc_prices`-project:

```SHELL
$ cd cmc_prices
$ cp ../cmc_prices-Cargo-manifest.toml ./Cargo.toml
```

... then I soft-link the following files into `cmc_prices/src/`:

```SHELL
$ cd src
$ ln -s ../../lib.rs ./lib.rs
$ ln -s ../../json_utils.rs ./json_utils.rs
$ ln -s ../../csv_utils.rs ./csv_utils.rs
$ ln -s ../../crypto_types.rs ./crypto_types.rs
$ ln -s ../../../ch03/utils.rs ./utils.rs
$ ln -s ../../cmc_prices.rs ./cmc_prices.rs
$ cd ../
```

Then you should be able to build and run your application:

```SHELL
$ cargo run ../cmc_listings_sample.json
```

... and see the following output:

```TXT
There are 2 coins.

date,cmc_id,rank,symbol,name,price
2021-03-21,1,1,BTC,Bitcoin,$57220.49
2021-03-21,1027,2,ETH,Ethereum,$1787.62
```

Doit-Toit!
