
use book::{
   file_utils::extract_date_and_body,
   list_utils::{head,split},
   utils::get_args
};

use data_entry::preprocess_with_sign;

fn usage() {
   println!("\n./benqi <file>");
   println!("\n\tConvert file of quotes and amounts to just amounts.\n");
}

fn main() {
   if let Some(file) = head(get_args()) {
      let (_, body) = extract_date_and_body(&file);
      split(body,"Borrowed".to_string()).iter_mut()
          .fold(1.0, preprocess_with_sign);
   } else {
      usage();
   }
}
