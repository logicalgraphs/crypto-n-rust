#!/bin/sh

echo -e "Blockaverse tokens, $LE_DATE

$($RUST_BOOK/src/scripts/tsv.pl $HOME/Downloads/export.csv)" | voronoi $COLORS
