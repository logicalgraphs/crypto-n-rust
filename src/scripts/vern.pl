#!/usr/bin/perl

my $cd = "cd $ENV{CRYPTO_TOOLS}/vern";
my @cmd = split(' ', "cargo run $ARGV[0] $ARGV[1] $ARGV[2] $ENV{MARKET} $ENV{FIN_DIR}/some_synthetics.tsv $ENV{FIN_DIR}/all-isomorphic-paths.csv");
my $ans = `$cd; $ENV{RUST_SCRIPTS_DIR}/entail.pl @cmd`;
print "\n\n./vern, ./vern, my main man, ./vern!\n\n$ans";
