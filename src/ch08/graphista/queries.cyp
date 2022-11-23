// to load ratios:

load csv with headers from "https://raw.githubusercontent.com/logicalgraphs/crypto-n-rust/main/src/ch08/graphista/data/market.csv" as row
merge (c:Coin {name: row.buy})
merge (c1:Coin {name: row.sell})
merge (c)-[r:RATIO {multiplier: toFloat(row.ratio)}]->(c1)

// to load inverse ratios:

load csv with headers from "https://raw.githubusercontent.com/logicalgraphs/crypto-n-rust/main/src/ch08/graphista/data/market.csv" as row
with row where row.ratio <> "0"
merge (c:Coin {name: row.buy})
merge (c1:Coin {name: row.sell})
merge (c)<-[r:RATIO {multiplier: toFloat(row.inverse)}]-(c1)

// active order books:

match p=(c)-[r]->(c1) where r.multiplier > 0 return p

// non-reflexive paths:

match p=(n:Coin)-->(n1) where not (n1)-->(n) return p

// KUJI paths

// 1 step

match p=(n:Coin { name: "KUJI"})-[]->(n1:Coin)-->(n) where n1 <> n return p

// CSVible Downloadable tableable:

match p=(n:Coin { name: "KUJI"})-[f]->(a:Coin)-[t]->(n) where a <> n 
return f.multiplier,a.name,t.multiplier

// 2 step

match p=(n:Coin { name: "KUJI"})-[]->(a:Coin)-[]->(b)-->(n) 
where b <> n
return p

// and the cvsible:

match p=(n:Coin { name: "KUJI"})-[r1]->(a:Coin)-[r2]->(b)-[r3]->(n) 
where b <> n
return r1.multiplier,a.name,r2.multiplier,b.name,r3.multiplier

// 3 step

match p=(n:Coin { name: "KUJI"})-[]->(a:Coin)-[]->(b)-[]->(c)-->(n) 
where b <> n and c <> a
return p

// and the csvible:

match p=(n:Coin { name: "KUJI"})-[r1]->(a:Coin)-[r2]->(b)-[r3]->(c)-[r4]->(n) 
where b <> n and c <> a
return r1.multiplier,a.name,r2.multiplier,b.name,r3.multiplier,c.name,r4.multiplier

// and a query to extract just the node names for paths:

match p=(n:Coin { name: "OSMO"})-->(a:Coin)-->(b)-->(c)-->(d)-->(n) 
where b <> n and c <> a and d <> b and c <> n
return a.name,b.name,c.name,d.name

// ------------------------ pathing information

// 0 deep

match p=(n:Coin { name: "OSMO"})-->(a:Coin)-->(z)
where n = z 
return n.name,a.name,z.name

// 1 deep

match p=(n:Coin { name: "OSMO"})-->(a:Coin)-->(b)-->(z)
where n = z 
return n.name,a.name,b.name,z.name

// 2 deep

match p=(n:Coin { name: "ATOM"})-->(a:Coin)-->(b)-->(c)-->(z)
where n = z and b <> n and a <> c
return distinct n.name,a.name,b.name,c.name,z.name

// 3 deeo, n.b.: we do allow assets SOME repeat appearances here

match p=(n:Coin { name: "USK"})-->(a:Coin)-->(b)-->(c)-->(d)-->(z)
where n = z and b <> n and a <> c and b <> d and c <> z
return distinct n.name,a.name,b.name,c.name,d.name,z.name


