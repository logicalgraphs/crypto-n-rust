# Benqi daily positions

Reports the positions on all assets (supply and borrow) on the Benqi 
marketplace.

## Setup

1. [BUIDL `stamp`](../../ch04/)
2. copypasta your [Benqi marketplace positions](https://app.benqi.fi/overview)
3. `$ echo '*multiline benqi marketplace*' | stamp > data/benqi_positions.lsv`

![Benqi Overview](imgs/benqi-marketplace.png)

[Here](benqi/2024-03-02.lsv) is a sample of the copypasta'd Benqi assets.

## Run

1. `$ cargo run benqi/2024-03-02.lsv`

You should see a report looking something like this:

![My supplies and borrows](imgs/supplies-borrows.png)

which you can copypasta onto your portfolio spreadsheetszorxen, that you 
have establish from anon to accept these positions and suchlike.
