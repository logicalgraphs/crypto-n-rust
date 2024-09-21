use book::{
   // csv_utils::parse_csv,
   err_utils::ErrStr,
   file_utils::extract_date_and_body,
   utils::get_args,
};

fn usage() -> ErrStr<()> {
   println!("./answer21 <portfolio>
	Parses <portfolio>, extracting tokens and amounts
");
   Err("Must include <portfolio> file!".to_string())
}

fn main() -> ErrStr<()> {
   let args = get_args();
   if let Some(file) = args.first() {
      let (date, lines) = extract_date_and_body(&file)?;
      println!("File {file}, dated {date}, has {} items.", lines.len());
      Ok(())
   } else {
      usage()
   }
}
