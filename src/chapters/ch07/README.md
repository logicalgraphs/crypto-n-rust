# Pop-quiz: scanning data

So, okay, I've got this manual process going where I manually update my
market positions on [@BenqiFinance](https://app.benqi.fi/overview) after each
SUPPLY and BORROW.

That's getting tedious for me.

So, I've scraped the data from the screen, storing it 
[here](data/benqi_positions.lsv).

Write a Rust program that:

1. scans this file
2. lists each position by line

So, for example, it would list:

SUPPLIED

1.5774  
23,618.2155  
0.0494  
26,909.59  
0.2765  
4.0004  
300.9647  
19.3823  
0.0038  
826.6162  

BORROWED

0.6051  
0.0226  
6,687.91  
0.0884  
0.1579  
15,228.43  
4,469.81  
0.001  
1.0017  

Just that. Only that.

* [(basic) solution](data_entry.rs)

## BONUS

1. Remove the commas from the output numbers. I don't like commas.
2. Put negative number signs in front of the borrows.
3. remove the SUPPLY and BORROW headers.

So, the bonus output would be simply:

1.5774  
23618.2155  
0.0494  
26909.59  
0.2765  
4.0004  
300.9647  
19.3823  
0.0038  
826.6162  
-0.6051  
-0.0226  
-6,687.91  
-0.0884  
-0.1579  
-15228.43  
-4,469.81  
-0.001  
-1.0017  

Rock and roll. Rokk. und. Rollen!

* [(bonus) solution](bonus_data_entry.rs) with folds and higher-order functions
