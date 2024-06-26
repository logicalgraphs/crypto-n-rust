# Pivot quiz 11 answer

## serializing ratios as JSON

The majority of the work was (re-)activating the 
[json_utils](../../../libs/book/json_utils.rs) so that serialization of Rust
structures become, _not JSON_, but JavaScript objects.

> (There is a distinction.)

![Ratios as JavaScript objects](imgs/JS-ratio.objects.png)

We then embed these data directly into whichever chart-renderer you choose.
