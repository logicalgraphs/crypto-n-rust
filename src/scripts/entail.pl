#!/usr/bin/perl

my @ans = `@ARGV`;
my @roff = @ans[-14..-1];
print "@roff";
