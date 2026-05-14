load csv with headers from "https://raw.githubusercontent.com/logicalgraphs/crypto-n-rust/charts/src/ch08/charts/xform/data/trades.tsv" as row
FIELDTERMINATOR '\t'
merge (c:Coin {symbol: row.symbol, security: row.security, trade: row.net_gain})
merge (tt:TradeType {type: row.trade_type})
merge (a:Action {action: row.action})
merge (p:Portfolio {portfolio: row.portfolio})
merge (p)-[t:TRADE {trade_type: row.trade_type, action: row.action, executed: row.date_sold}]->(c)
merge (tt)-[y:KIND]->(c)
merge (a)-[:ACTION]->(c)
