use std::collections::{HashSet,HashMap};

use book::{
   compose,
   matrix_utils::{Matrix,from_lines},
   stream_utils::lines_from_stream
};

use crypto::types::usd::USD;

fn step_0_clear() -> String {
   "match (n) detach delete n;".to_string()
}

fn step_1_create_portfolio() -> String {
   "create (p:Portfolio { name: 'Portfolio' });".to_string()
}

fn step_2_create_blockchain(b: String) -> String {
   format!("create (b:Blockchain {{ name: '{b}' }});")
}

// ----- The 'E' in 'ETL' ---------------------------------------------------

fn extract_blockchain(line: &Vec<String>) -> String {
   line[1].to_string()
}

fn extract_protocol_blockchain(line: &Vec<String>) -> (String, String) {
   (line[6].clone(), line[1].clone())
}

type BlockChains = HashMap<String, HashSet<String>>;

fn step_3_map_token_protocol(row: &Vec<String>, b: &mut BlockChains) -> String {
   let token = &row[0];
   let (protocol, blockchain) = extract_protocol_blockchain(&row);
   let value: USD
      = row[3].parse().expect(&format!("Could not parse USD {}", row[3]));
   let amount = value.amount;
   let prots = b.entry(blockchain.clone()).or_insert(HashSet::new());
   prots.insert(protocol.clone());
   format!("merge (p:Protocol {{ name: '{protocol}' }})
merge (t:Token {{ name: '{token}' }})
merge (t)-[:ON {{ value: {amount}, in: '{blockchain}' }}]->(p);")
}

fn protocol_mapper(blockchain: &str) -> impl Fn(String) -> String + '_ {
   move |protocol| format!("match (b:Blockchain {{ name: '{blockchain}' }})
match (p:Protocol {{ name: '{protocol}' }})
merge (p)-[:IN]->(b);")
}

fn step_4_map_protocols_blockchain(pear: (String, HashSet<String>)) -> String {
   let maps: Vec<String> = pear.1.into_iter()
                                 .map(protocol_mapper(&pear.0))
                                 .collect();
   maps.join("\n")
}

fn step_5_map_blockchains_portfolio() -> String {
   "match (p:Portfolio)
match (b:Blockchain)
merge (p)<-[:IN]-(b);".to_string()
}

fn print_str(s: String) { println!("{s}"); }

fn create_blockchains(matrix: &Matrix<String>) {
   let many_blockchains: Vec<String>
      = matrix.into_iter().map(extract_blockchain).collect();
   let blockchains: HashSet<String> = many_blockchains.into_iter().collect();
   blockchains.into_iter()
              .for_each(compose!(print_str)(step_2_create_blockchain));
}

fn map_protocols(matrix: &Matrix<String>) {
   let mut all_protocols: BlockChains = HashMap::new();
   for row in matrix {
      print_str(step_3_map_token_protocol(&row, &mut all_protocols));
   }
   all_protocols.into_iter()
                .for_each(compose!(print_str)(step_4_map_protocols_blockchain));
}

fn main() {
   let lines = lines_from_stream();
   let matrix: Matrix<String> = from_lines(&lines, "\t");

   print_str(step_0_clear());
   print_str(step_1_create_portfolio());

   create_blockchains(&matrix);
   map_protocols(&matrix);

   print_str(step_5_map_blockchains_portfolio());
}
