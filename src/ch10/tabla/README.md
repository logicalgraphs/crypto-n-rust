# tabla

Takes a copypasta from a SPREADSHEETSZORXEN! and converts those rows of data
into an HTML-table.

## e.g.

<code>
$ echo 'Token	amount	value $
USK	210.84	$210.63
LUNA	179.0488	$204.46
axlUSDC	47.642972	$47.64' | tabla
</code>

The output from the above would look like this:

<table border="1" width="75%" align="center"><tr bgcolor="cyan"><th><p>Token</p></th>
<th><p>amount</p></th>
<th><p>value $</p></th></tr>
<tr ><td align="left"><p>USK</p></td>
<td align="right"><p>210.84</p></td>
<td align="right"><p>$210.63</p></td></tr>
<tr ><td align="left"><p>LUNA</p></td>
<td align="right"><p>179.0488</p></td>
<td align="right"><p>$204.46</p></td></tr>
<tr ><td align="left"><p>axlUSDC</p></td>
<td align="right"><p>47.642972</p></td>
<td align="right"><p>$47.64</p></td></tr></table>

The. End.
