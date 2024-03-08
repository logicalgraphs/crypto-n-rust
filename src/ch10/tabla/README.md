# tabla

Takes a copypasta from a SPREADSHEETSZORXEN! and converts those rows of data
into an HTML-table.

## e.g.

<code>
$ echo 'Token	amount	value $
USK	210.84	$210.63
LUNA	179.0488	$204.46
axlUSDC	47.642972	$47.64' | tabla --totals 2
</code>

The output from the above would look like this:

![Table with a total-row](imgs/table-total.png)

n.b.: the `--totals`-parameter (with following arguments) is optional.
n.b.b.: The `--totals`-columns are zero-indexed-based, e.g. `2` indicates the
third row; /capice?/

The. End.
