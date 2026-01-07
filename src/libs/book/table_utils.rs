use std::{
   clone::Clone,
   cmp::Eq,
   collections::{HashMap,HashSet},
   fmt::Display,
   hash::Hash
};

use crate::{
   compose,
   csv_utils::CsvWriter,
   err_utils::ErrStr,
   list_utils::{ht,tail},
   matrix_utils,
   matrix_utils::Matrix,
   string_utils::to_string,
   tuple_utils::{fst,snd,swap}
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

fn enum_headers<HEADER: Eq + Hash>(headers: Vec<HEADER>)
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
         tail(&hdr.split(separator).map(to_string).collect::<Vec<String>>());
      let (rows_, data) = rows_in_jest(rowf, df, body, separator)?;
      let cols_ = parse_headers(colf, &cols_str)?;
      Ok(Table { rows_, cols_, data })
   } else {
      Err("No table to ingest!".to_string())
   }
}

fn filter_map_or<T>(f: impl Fn(&str) -> ErrStr<T>, v: &Vec<String>)
      -> ErrStr<Vec<T>> {
   let mut ans: Vec<T> = Vec::new();
   for elt in v {
      let eh = f(elt)?;
      ans.push(eh);
   }
   Ok(ans)
}

fn parse_headers<HEADER: Eq + Hash>
   (headerf: impl Fn(&str) -> ErrStr<HEADER>,
    headers: &Vec<String>) -> ErrStr<HashMap<HEADER, usize>> {
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
      move |s| s.split(separator).map(to_string).collect()
   }
   let rows: Vec<Vec<String>> =
      lines.into_iter().map(&split_line(separator)).collect();
   let (mbs_hdrs, data): (Vec<Option<String>>, Vec<Vec<String>>) =
      rows.iter().map(|row| ht(&row)).unzip();
   let hdrs: Vec<String> = mbs_hdrs.into_iter().map(Option::unwrap).collect();
   let mut matrix: Matrix<DATUM> = Vec::new();
   for row in data {
      let r: Vec<DATUM> = filter_map_or(&df, &row)?;
      matrix.push(r);
   }
   let rows = parse_headers(rowf, &hdrs)?;
   Ok((rows, matrix))
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
