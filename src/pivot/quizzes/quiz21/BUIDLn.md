# Pivot quiz 21 answer

## Computing how much to trade for how much (applying δ)

First things first. We need to 
[totally overhaul file_utils.rs](../../libs/book/file_utils.rs) to use
`ErrStr` instead of what it did before: `panic!()`.
