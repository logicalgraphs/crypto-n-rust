#!/usr/bin/perl

my $cd = "cd $ENV{CRYPTO_TOOLS}/charts/gantt";
my $run = "cargo run '$ARGV[0]'";
my $ans = `$cd; $run`;

print "$ans";
