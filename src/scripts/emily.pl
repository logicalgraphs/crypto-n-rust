#!/usr/bin/perl

my $cd = "cd $ENV{CRYPTO_TOOLS}/intimate";
my $run = "cargo run $ARGV[0] $ENV{FIN_DIR}/market.lsv $ARGV[1]";
my $ans = `$cd; $run`;

print "\n\nFor ./emily, wherever I may find her.\n\n$ans";
