use std::io::stdin;

pub fn lines_from_stream() -> Vec<String> {
   let mut lines: Vec<String> = Vec::new();
   loop {
      let mut butter = String::new();
      stdin().read_line(&mut butter).expect("EOF");
      if butter == "" { break; }
      lines.push(butter.trim().to_string());
   }
   lines
}

#[cfg(not(tarpaulin_include))]
pub mod functional_tests {
   use super::*;
   use crate::err_utils::{ErrStr,err_or};

   use std::io::Write;
   use std::process::{Command, Stdio};

   async fn run_stdio() -> ErrStr<usize> {
      println!("\nstream_utils::stdio functional test\n");
      let mut child =
        err_or(Command::new("reversi") // Example command that reverses input
                       .stdin(Stdio::piped())
                       .stdout(Stdio::piped())
                       .spawn(),
               "Unable to spawn child process")?;

      // Take the stdin handle and write data to it
      if let Some(mut stdin) = child.stdin.take() {
          err_or(stdin.write_all(b"hello world\n"),
                 "Unable to write to stdio")?;
      } // Handle drops here, 
        // closing the pipe so the child knows input is finished

      let output =
         err_or(child.wait_with_output(),
                "Unable to wait on the child process")?;
      println!("\tResult: {}", String::from_utf8_lossy(&output.stdout));
      println!("\nstream_utils::stdio:...ok");
      Ok(1)
   }

   pub async fn runoff() -> ErrStr<usize> {
      println!("\nstream_utils functional tests\n");
      let a = run_stdio().await?;
      Ok(a)
   }
}

