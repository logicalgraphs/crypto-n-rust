# Pivot quiz 23 answer

## `./dawn`

The solution for `./dawn` involved introducing two new types to the 
`swerve`-library: 
[`PricedAsset`](../../swerve/types.rs#L476-L481) and 
[`TradeCall`](../../swerve/types.rs#L476-L481).

Then we [construct the `TradeCall`-value](../../swerve/types.rs#L502-L535) from 
the [pivot-table data](../../../data-files/csv/pivots.csv) and the 
recommendation for today from the `TradeRoute`. We know how much of each asset 
available to trade from the `Pools` that we extract from our `assets.csv`-file.

![Trade calls](imgs/swap-amounts.png)
