#!/usr/bin/perl

my $cd = "cd $ENV{CRYPTO_TOOLS}/vern";
my @cmd = split(' ', "cargo run $ARGV[0] $ARGV[1] $ARGV[2] $ENV{FIN_DIR}/market.lsv $ENV{FIN_DIR}/some_synthetics.tsv $ENV{FIN_DIR}/order_book_volumes.json $ENV{FIN_DIR}/all-isomorphic-paths.csv");
my $ans = `$cd; entail @cmd`;
print "\n\n./vern, ./vern, my main man, ./vern!\n\n$ans";
