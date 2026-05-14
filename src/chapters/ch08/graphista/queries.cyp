// to load ratios:

load csv with headers from "https://raw.githubusercontent.com/logicalgraphs/crypto-n-rust/main/src/ch08/graphista/data/market-graph.csv" as row
merge (c:Coin {name: row.buy})
merge (c1:Coin {name: row.sell})
merge (c)-[r:RATIO {multiplier: toFloat(row.ratio)}]->(c1)

// to load inverse ratios:

load csv with headers from "https://raw.githubusercontent.com/logicalgraphs/crypto-n-rust/main/src/ch08/graphista/data/market-graph.csv" as row
with row where row.ratio <> "0"
merge (c:Coin {name: row.buy})
merge (c1:Coin {name: row.sell})
merge (c)<-[r:RATIO {multiplier: toFloat(row.inverse)}]-(c1)

// remove ATOM/OSMO/ATOM paths

match (n:Coin { name: 'ATOM'})-[r]-(n1:Coin { name: 'OSMO'}) delete r

// and

match (n:Coin)
where n.name in ['SCRT', 'STARS']
detach delete n

// active order books:

match p=(c)-[r]->(c1) where r.multiplier > 0 return p

// non-reflexive paths:

match p=(n:Coin)-->(n1) where not (n1)-->(n) return p

// ------------------------ pathing information

// 0 deep

match p=(n)-->(a:Coin)-->(z)
where n = z 
return distinct n.name,a.name,z.name

// 1 deep

match p=(n)-->(a:Coin)-->(b)-->(z)
where n = z 
return distinct n.name,a.name,b.name,z.name

// 2 deep

match p=(n)-->(a:Coin)-->(b)-->(c)-->(z)
where n = z and b <> n and a <> c
return distinct n.name,a.name,b.name,c.name,z.name

// 3 deep, n.b.: we do allow assets SOME repeat appearances here

match p=(n)-->(a:Coin)-->(b)-->(c)-->(d)-->(z)
where n = z and b <> n and a <> c and b <> d and c <> z
return distinct n.name,a.name,b.name,c.name,d.name,z.name
