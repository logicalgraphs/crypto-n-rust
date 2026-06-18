use std::{
   clone::Clone,
   cmp::Eq,
   collections::{HashMap,HashSet},
   fmt::Display,
   hash::Hash
};

use succ::succ;

use super::{
   compose,
   csv_utils::CsvWriter,
   err_utils::ErrStr,
   list_utils::{ht,tail,filter_map_or},
   matrix_utils,
   matrix_utils::Matrix,
   string_utils::{str2strf,s},
   tuple_utils::{fst,snd,swap,duplicate}
};

// a Table is a matrix indexed by hashed values, so we can have, e.g.:
// Table<NaiveDate, String, f32> which maps to Matrix<T> with row headers as 
// dates and column headers as strings.

// I accomplish by the mapping, e.g.: 2024-03-10 -> 0, 2024-03-11 -> 1, ... 
// for rows and ADA -> 0, ATOM -> 1, AVAX -> 2, ... for columns.

// (the mappings can be any <T:Hash+Eq>-typed values)

// ----- STRUCTURE -----------------------------------------------------------

#[derive(Debug,Clone)]
pub struct Table<ROW, COL, T> {
   rows_: HashMap<ROW, usize>,
   cols_: HashMap<COL, usize>,
   pub data: Matrix<T>
}

impl <ROW: Display + Clone,COL: Display + Clone,DATUM: Display> CsvWriter
      for Table<ROW,COL,DATUM> {
   fn ncols(&self) -> usize { self.cols_.len() + 1 }
   fn as_csv(&self) -> String {
      let cols = display_cols(&self.cols_);
      let rows = sort_headers(&self.rows_);
      fn fmt_row<ROW: Display, T: Display>((header, row): (&ROW, &Vec<T>))
            -> String {
         format!("{},{}", header, vec_to_vec(&row).join(","))
      }
      let data: Vec<String> = rows.iter()
                                  .zip(self.data.iter())
                                  .map(fmt_row)
                                  .collect();
      format!("{}\n{}", cols, data.join("\n"))
   }
}

fn vec_to_vec<T: Display>(line: &Vec<T>) -> Vec<String> {
   line.into_iter().map(|s| format!("{}", s)).collect()
}

fn display_cols<COL: Display + Clone>(cols: &HashMap<COL,usize>) -> String {
   format!(",{}", vec_to_vec(&sort_headers(cols)).join(","))
}

// ----- META-INFORMATION ---------------------------------------------------

pub fn enum_headers<HEADER: Eq + Hash>(headers: Vec<HEADER>)
      -> HashMap<HEADER, usize> {
   headers.into_iter().enumerate().map(swap).collect()
}

fn sort_headers<HEADER: Clone>(hdrs: &HashMap<HEADER,usize>) -> Vec<HEADER> {
   let mut hdrs1: Vec<(&HEADER,&usize)> = hdrs.into_iter().collect();
   hdrs1.sort_by_key(|k| k.1);
   hdrs1.into_iter().map(compose!(Clone::clone)(fst)).collect()
}

pub fn rows<ROW: Clone,COL,T>(table: &Table<ROW,COL,T>) -> Vec<ROW> {
   sort_headers(&table.rows_)
}

pub fn cols<ROW,COL: Clone,T>(table: &Table<ROW,COL,T>) -> Vec<COL> {
   sort_headers(&table.cols_)
}

// ----- CONSTRUCTORS ------------------------------------------------------

pub fn mk_table<ROW: Eq + Hash, COL: Eq + Hash, T>(r: Vec<ROW>, c: Vec<COL>,
      data: Matrix<T>) -> Table<ROW, COL, T> {
   let rows = enum_headers(r);
   let cols = enum_headers(c);
   Table { rows_: rows, cols_: cols, data }
}

// Generates a table from a Vec<(COL, T)>-instance.
// The row-'headers' (all one of them) is obviously a String-type.

pub fn from_vec<ROW: Clone + Eq + Hash, COL: Eq + Hash, T>
      (row: &ROW, cols: Vec<(COL, T)>) -> Table<ROW, COL, T> {
   let rows = vec![row.clone()];
   let (cols, v) = cols.into_iter().unzip();
   let data = matrix_utils::from_vec(v);
   mk_table(rows, cols, data)
}

// The above function has the order embedded into the vector-of-pairs.
// The below function is where ordering is unimportant, so we order by
// the column-headers

pub fn from_map<ROW: Clone + Eq + Hash, COL: Clone + Eq + Ord + Hash, T: Clone>
      (row: &ROW, cols: &HashMap<COL, T>) -> Table<ROW, COL, T> {
   let mut data: Vec<(COL, T)> = Vec::new();
   for (k, v) in cols { data.push((k.clone(), v.clone())); }
   data.sort_by_key(|k| k.0.clone());
   from_vec(row, data)
}

pub fn ingest<ROW: Eq + Hash,COL: Eq + Hash,DATUM>
   (rowf: impl Fn(&str) -> ErrStr<ROW>,
    colf: impl Fn(&str) -> ErrStr<COL>,
    df:   impl Fn(&str) -> ErrStr<DATUM>,
    lines: &Vec<String>, separator: &str) -> ErrStr<Table<ROW,COL,DATUM>> {

   // the first row is the column-headers:
   let (header, body) = ht(lines);
   if let Some(hdr) = header {
      let cols_str: Vec<String> =
         tail(&hdr.split(separator).map(s).collect::<Vec<String>>());
      let (rows_, data) = rows_in_jest(rowf, df, body, separator)?;
      let cols_ = parse_headers(str2strf(&colf), cols_str)?;
      Ok(Table { rows_, cols_, data })
   } else {
      Err(s("No table to ingest!"))
   }
}

fn parse_headers<HEADER: Eq + Hash>
   (headerf: impl Fn(String) -> ErrStr<HEADER>,
    headers: Vec<String>) -> ErrStr<HashMap<HEADER, usize>> {
   let hdrs = filter_map_or(&headerf, headers)?;
   Ok(enum_headers(hdrs))
}

// rows_in_jest is a funny function ... GEDDIT? ;)

fn rows_in_jest<ROW: Eq + Hash, DATUM>
   (rowf: impl Fn(&str) -> ErrStr<ROW>,
    df:   impl Fn(&str) -> ErrStr<DATUM>,
    lines: Vec<String>, separator: &str)
      -> ErrStr<(HashMap<ROW, usize>, Matrix<DATUM>)> {
   fn split_line(separator: &str) -> impl Fn(String) -> Vec<String> + '_ {
      move |r| r.split(separator).map(s).collect()
   }
   let rows: Vec<Vec<String>> =
      lines.into_iter()
           .filter(|x| !x.is_empty())
           .map(&split_line(separator))
           .collect();
   let (mbs_hdrs, data): (Vec<Option<String>>, Vec<Vec<String>>) =
      rows.iter().map(|row| ht(&row)).unzip();
   let hdrs: Vec<String> = mbs_hdrs.into_iter().map(Option::unwrap).collect();
   let mut matrix: Matrix<DATUM> = Vec::new();
   for row in data {
      let r: Vec<DATUM> = filter_map_or(str2strf(&df), row)?;
      matrix.push(r);
   }
   let rows = parse_headers(str2strf(&rowf), hdrs)?;
   Ok((rows, matrix))
}

// ----- SPARSE MATRIX -------------------------------------------------------

// from a list of disparate HashMaps, ingest a table with columns with no
// values defaulting

pub fn sparse_matrix<COL: Clone + Eq + Hash, DATUM: Clone + Default>
      (maps: &[HashMap<COL, DATUM>]) -> Table<usize, COL, DATUM> {
   let mut headers: HashSet<COL> = HashSet::new();
   maps.iter().for_each(|map| headers.extend(map.keys().cloned()));
   let c0: Vec<(COL, usize)> = 
      headers.into_iter().enumerate().map(|(x, k)| (k, x+1)).collect();
   let c1: Vec<COL> = c0.clone().into_iter().map(fst).collect();
   let cols_: HashMap<COL, usize> = c0.into_iter().collect();
   let rows_: HashMap<usize, usize> =
      (0.. maps.len()).into_iter().map(compose!(duplicate)(succ)).collect();
   let data: Matrix<DATUM> =
      maps.iter()
          .map(|row| c1.iter()
               .map(|k| row.get(k).unwrap_or(&DATUM::default()).clone())
               .collect())
          .collect();
   Table { rows_, cols_, data }
}

// ----- VIEWS ----------------------------------------------------------------

fn c_ix<ROW, COL: Eq + Hash, DATUM>(table: &Table<ROW, COL, DATUM>, cix: &COL)
      -> Option<usize> {
   table.cols_.get(cix).cloned()
}

fn r_ix<ROW: Eq + Hash, COL, DATUM>(table: &Table<ROW, COL, DATUM>, rix: &ROW)
      -> Option<usize> {
   table.rows_.get(rix).cloned()
}

pub fn col<ROW, COL: Eq + Hash, DATUM: Clone>(table: &Table<ROW, COL, DATUM>,
                                              cix: &COL) -> Option<Vec<DATUM>> {
   c_ix(table, cix).and_then(|c| Some(matrix_utils::col(&table.data, c)))
}

pub fn row<ROW: Eq + Hash, COL, DATUM: Clone>(table: &Table<ROW, COL, DATUM>,
                                              rix: &ROW) -> Option<Vec<DATUM>> {
   r_ix(table, rix).and_then(|r| Some(table.data[r].clone()))
}

pub fn hashed_row<ROW: Eq+Hash+Clone, COL: Eq+Hash+Clone, DATUM: Clone>
      (table: &Table<ROW, COL, DATUM>, rix: &ROW)
      -> Option<HashMap<COL, DATUM>> {
   row(table, rix).and_then(|r| {
      let paired: HashMap<COL, DATUM> =
        cols(table).into_iter().zip(r.into_iter()).collect();
      Some(paired)
   })
}

pub fn hashed_rows<ROW: Eq+Hash+Clone, COL: Eq+Hash+Clone, DATUM: Clone>
      (table: &Table<ROW, COL, DATUM>) -> Vec<HashMap<COL, DATUM>> {
   rows(table).iter().filter_map(|rix| hashed_row(table, rix)).collect()
}

pub fn val<ROW: Eq + Hash, COL: Eq + Hash, DATUM: Clone>
         (table: &Table<ROW, COL, DATUM>, rix: &ROW, cix: &COL)
      -> Option<DATUM> {
   row(table, rix).and_then(|row| c_ix(table, cix)
                  .and_then(|ix| row.get(ix).cloned()))
}

pub fn row_filter<ROW: Clone + Eq + Hash, COL: Clone + Eq + Hash, DATA: Clone>
      (f: impl Fn(&ROW) -> bool, table: &Table<ROW, COL, DATA>)
      -> Table<ROW, COL, DATA> {
   // for the new table, 
   // 1. the columns are the columns, so that's no biggie
   let cols = sort_headers(&table.cols_);

   // 2. we filter the rows
   let mut rs = table.rows_.clone();
   rs.retain(|k,_v| f(k));

   // 3. now we filter the data by rows
   let row_ixen: HashSet<usize> = rs.values().map(|v| *v).collect();
   let rows = sort_headers(&rs);
   let data: Matrix<DATA> =
      table.data.clone()
                .into_iter()
                .enumerate()
                .filter(|(ix,_row)| row_ixen.contains(ix))
                .map(snd)
                .collect();

   mk_table(rows, cols, data)

   // now a call to col() on this new table will be properly filtered.
}

// ----- TRANSPOSE ------------------------------------------------------------

// transpose() is simply matrix_utils::transpose() with our table's rows and
// columns swapped ... right?

pub fn transpose<ROW: Clone + Eq + Hash, COL: Clone + Eq + Hash, DATUM: Clone>
      (table: &Table<ROW, COL, DATUM>) -> Table<COL, ROW, DATUM> {
   let new_rows = cols(&table);
   let new_cols = rows(&table);
   let new_dater = matrix_utils::transpose(&table.data);
   mk_table(new_rows, new_cols, new_dater)
}

// ----- MERGE ----------------------------------------------------------------

// merging two tables is a rather complicated affair? Nah.

// after some (very) heavy lifting, I simply added val(Table, ROW, COL)
// this reduced the problem with a nested for-loop, looking up the values in
// both tables to populate the newly merged table.

type Headers<HEADER> = HashMap<HEADER, usize>;

// merges headers of the old tables

fn new_headers<HEADER: Eq + Hash + Ord + Clone>
      (h1: &Headers<HEADER>, h2: &Headers<HEADER>) -> Vec<HEADER> {
   let keys1: HashSet<HEADER> = h1.keys().cloned().collect();
   let keys2: HashSet<HEADER> = h2.keys().cloned().collect();
   let mut sorted_headers: Vec<HEADER> = keys1.union(&keys2).cloned().collect();
   sorted_headers.sort();
   sorted_headers
}

// merge function that allows tuning of defaults and spews debug info on request

pub fn merge_with_default_d<ROW: Clone + Eq + Hash + Ord + Display, 
               COL: Clone + Eq + Hash + Ord + Display,
               DATUM: Clone + Display + 'static>
      (source: &Table<ROW, COL, DATUM>, adjoin: &Table<ROW, COL, DATUM>,
       default: impl Fn(String) -> ErrStr<DATUM> + 'static, debug: bool)
         -> ErrStr<Table<ROW, COL, DATUM>> {

   let sorted_rows = new_headers(&source.rows_, &adjoin.rows_);
   let sorted_cols = new_headers(&source.cols_, &adjoin.cols_);

   let mut new_mat = Vec::new();

   // now that we have the new headers (rows and cols), let's build the
   // new matrix for our table

   for row_hdr in &sorted_rows {
      let mut cols = Vec::new();

      // REWRITE! ... *sheesh*
      // 1. get the val, or nought, from adjoin table for the filtered row
      // 2. if nought, get the val from source table for the filtered row
      // 3. if nought, throw error

      for col_hdr in &sorted_cols {

         let err_msg =
            format!("Unable to find value at indices [{row_hdr}, {col_hdr}]");
         let cell: DATUM =
            val(&adjoin, &row_hdr, &col_hdr)
               .or(val(&source, &row_hdr, &col_hdr))
               .ok_or(err_msg)
               .or_else(&default)?;

         if debug { println!("Processed {col_hdr} for {row_hdr}: {cell}"); }
         cols.push(cell);
      }
      new_mat.push(cols);
   }
   Ok(mk_table(sorted_rows, sorted_cols, new_mat))
}

// standard merge-implementation: fail on unknown values (no defaulted values)

pub fn merge<ROW: Clone + Eq + Hash + Ord + Display,
             COL: Clone + Eq + Hash + Ord + Display,
             DATUM: Clone + Display + 'static>
      (source: &Table<ROW, COL, DATUM>, adjoin: &Table<ROW, COL, DATUM>)
        -> ErrStr<Table<ROW, COL, DATUM>> {
   merge_with_default_d(source, adjoin, err_out(), false)
}

pub fn err_out<'a, DATUM>() -> impl Fn(String) -> ErrStr<DATUM> + 'a {
   move |msg| Err(format!("Merge failed. Reason: {msg}"))
}

// reshapes merge-implementation by providing a default value on merge-lookup
// failure, call merge_with_default_d() when using this function-factory

pub fn default_f<'a, DATUM: Clone>(d: &'a DATUM)
      -> impl Fn(String) -> ErrStr<DATUM> + 'a {
   move |_msg| Ok(d.clone())
}

// ----- TESTS -------------------------------------------------------

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod functional_tests {
   use super::*;
   use paste::paste;

   use crate::{
      create_testing,
      parse_utils::{parse_id,parse_str,parse_int},
      string_utils::words,
      tuple_utils::first,
      utils::debug
   };

   create_testing!("table_utils");

   fn sample_tsv_table() -> String {
s("store	apples 	bananas	chips	durian	eggs	fish	guinness
1	29	27	33	28	18	44	7
2	14	38	15	2	46	5	12
3	15	5	28	44	13	39	17

4	38	48	16	22	47	8	37
5	5	27	2	30	42	18	14")
   }

   fn small_table_1() -> String {
s("store	chips	eggs	fish	guinness
3	22	11	27	9
4	16	47	8	37
5	2	42	18	14")
   }

   fn small_table_2() -> String {
s("store	chips	eggs	fish	guinness
1	44	18	44	7
2	15	46	5	12
3	28	13	39	17")
   }

   fn small_table_3() -> String {
s("store	apples	bananas	durian
1	29	27	18
2	14	38	2
4	38	48	22")
   }

   fn sparse_data_string(rows: &[HashMap<String, f32>]) -> String {
      rows.into_iter().map(debug).collect::<Vec<_>>().join("\n")
   }
   fn sample_sparse_table_data() -> Vec<HashMap<String, f32>> {
      let avalanche: HashMap<String, f32> =
         vec![("BTC", 0.96), ("sAVAX", 1402.5), ("ETH", 3.15), ("USDC", 9.8),
              ("AVAX", 32.8), ("UNDEAD", 457484.1), ("USDt", 100.0)]
              .into_iter().map(first(s)).collect();
      let binance: HashMap<String, f32> =
         vec![("USDt", 8943.2), ("DOGE", 52085.2), ("LTC", 79.5),
              ("LINK", 317.0), ("ETH", 1.8), ("BNB", 0.8)]
              .into_iter().map(first(s)).collect();
 
      let cardano: HashMap<String, f32> =
         vec![("ADA", 8185.7), ("SNEK", 508401.0), ("INDY", 1590.4),
              ("iUSD", 996.3), ("iBTC", 0.01), ("iETH", 0.7)]
              .into_iter().map(first(s)).collect();
      vec![avalanche, binance, cardano]
   }

   fn ingest_a_table(table: &str) -> ErrStr<Table<usize,String,i32>> {
      let lines: Vec<String> = table.split("\n").map(s).collect();
      ingest(parse_id, parse_str, parse_int, &lines, "\t")
   }

   fn ingest_table() -> ErrStr<Table<usize,String,i32>> {
      ingest_a_table(&sample_tsv_table())
   }

   run!("ingest", {
      let table = sample_tsv_table();
      println!("\tthe input table is:\n\n{table}");
      let tab = ingest_table()?;
      println!("\n\tthe parsed table is:\n\n{}", tab.as_csv());
   });

   run!("sparse_matrix", {
      let data = sample_sparse_table_data();
      println!("Sparse data input: {}\n", sparse_data_string(&data));
      let table = sparse_matrix(&data);
      println!("Sparse table:\n{}\n", table.as_csv());
   });

   run!("sparse_stores", {
      let a = ingest_a_table(&small_table_2())?;
      println!("Table a:\n\n{}", a.as_csv());
      let b = ingest_a_table(&small_table_3())?;
      println!("Table b:\n\n{}", b.as_csv());
      let mut a1 = hashed_rows(&a);
      let mut b1 = hashed_rows(&b);
      a1.append(&mut b1);
      let sparse = sparse_matrix(&a1);
      println!("Combined (sparse) table:\n\n{}", sparse.as_csv());
   });


   run!("row_store_2", {
      let stores = ingest_table()?;
      println!("\tThe store table:\n{}\n", stores.as_csv());
      println!("\tThe second store data is:\n{:?}", row(&stores, &2));
   });

   run!("val_store_5_fish", {
      let stores = ingest_table()?;
      let ur_mom = val(&stores, &5, &s("fish")).unwrap();
      println!("\tThe number of fish at store 5 is: {ur_mom}");
   });

   run!("enum_headers", {
      let hdrs = enum_headers(words("ix item price amt"));
      println!("Headers of table: {hdrs:?}");
   });

   run!("merge", {
      let a = ingest_a_table(&small_table_1())?;
      let b = ingest_a_table(&small_table_2())?;
      println!("Table to merge (1):\n\n{}", a.as_csv());
      println!("Table to merge (2):\n\n{}", b.as_csv());
      let c = merge(&a, &b)?;
      println!("Merged table:\n\n{}", c.as_csv());
   });

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod tests {
   use super::*;
   use super::functional_tests::ingest_table;

   #[test] fn test_ingest() {
      let table = ingest_table();
      assert!(table.is_ok());
   }

   #[test] fn test_rows() -> ErrStr<()> {
      let table = ingest_table()?;
      assert_eq!(rows(&table).len(), 5);
      Ok(())
   }

   #[test] fn test_cols() -> ErrStr<()> {
      let table = ingest_table()?;
      assert_eq!(cols(&table).len(), 7);
      Ok(())
   }

   fn val1(t: Table<usize, String, i32>, r: usize, c: &str) -> Option<i32> {
      val(&t, &r, &s(&c))
   }

   #[test] fn fail_val_row_col() -> ErrStr<()> {
      let table = ingest_table()?;
      assert!(val1(table, 7, "hamburger").is_none());
      Ok(())
   }

   #[test] fn fail_val_row() -> ErrStr<()> {
      let table = ingest_table()?;
      assert!(val1(table, 12, "bananas").is_none());
      Ok(())
   }

   #[test] fn fail_val_col() -> ErrStr<()> {
      let table = ingest_table()?;
      assert!(val1(table, 3, "apple pie").is_none());
      Ok(())
   }

   #[test] fn test_val() -> ErrStr<()> {
      let table = ingest_table()?;
      assert_eq!(val1(table, 2, "chips"), Some(15));
      Ok(())
   }

   #[test] fn test_row_filter() -> ErrStr<()> {
      let table = ingest_table()?;
      let filtered_table = row_filter(|x| *x > 3, &table);
      assert_eq!(2, filtered_table.data.len());
      Ok(())
   }
}
}
