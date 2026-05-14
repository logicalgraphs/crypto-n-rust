use std::io::{Read, stdin};
use super::{ err_utils::{ErrStr,err_or}, string_utils::s };

pub fn lines_from_stdin() -> ErrStr<Vec<String>> {
   let stdin = stdin();
   lines_from_stream(stdin.lock())
}

fn lines_from_stream<R: Read>(mut io: R) -> ErrStr<Vec<String>> {
   let mut butter = String::new();
   let _ = err_or(io.read_to_string(&mut butter), "Cannot seive stdin")?;
   if butter.is_empty() {
      Err(s("Nothing on input stream"))
   } else {
      Ok(butter.split("\n").map(s).collect())
   }
}

// ----- TESTS -------------------------------------------------------

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
pub mod functional_tests {
   use super::*;

   use std::{ io::{Write,Cursor}, process::{Command, Stdio, Child, Output} };

   use paste::paste;
   use crate::{ create_testing, utils::now };

   create_testing!("stream_utils",
                   "Running commands against the input stream.");

   async fn input(cmd: &str) -> ErrStr<Child> {
      let mut child =
        err_or(Command::new(cmd) // Example command that reverses input
                       .stdin(Stdio::piped())
                       .stdout(Stdio::piped())
                       .spawn(),
               "Unable to spawn child process")?;

      // Take the stdin handle and write data to it
      if let Some(mut stdin) = child.stdin.take() {
          err_or(stdin.write_all(b"I have eaten
the plums
that were in
the icebox

and which
you were probably
saving
for breakfast

Forgive me
they were delicious
so sweet
and so cold"),
                 "Unable to write to stdin")?;
      } // Handle drops here,
        // closing the pipe so the child knows input is finished
      Ok(child)
   }

   async fn cmd(c: &str) -> ErrStr<Output> {
      let child = input(c).await?;
      let output =
         err_or(child.wait_with_output(),
                "Unable to wait on the child process")?;
      Ok(output)
   }

   async fn cmd_out(c: &str) -> ErrStr<String> {
      let outie = cmd(c).await?;
      let reader = Cursor::new(outie.stdout);
      let lines = lines_from_stream(reader)?;
      let ans = lines.join("\n");
      println!("The output stream from command '{c}':\n\n{ans}\n");
      Ok(ans)
   }

   async fn poem() -> ErrStr<String> {
      cmd_out("cat").await
   }

/*
         kid.write_all(b"1
2
3
4
Can I have a little more?

5
6
7,8,9,10...
I love you!").expect("The Beatles");
*/

   run!("lines_from_stream",
        " (A poem I read by William Carlos Williams)",
      now(async {
         let p = poem().await?;
         println!("\tpoem line count: {}\n", p.len());
         Ok::<(), String>(())
   }));

   #[cfg(test)]
   mod tests {
      use super::*;

      #[tokio::test]
      async fn test_lines_from_stream_ok() {
         let p = poem().await;
         assert!(p.is_ok());
      }

      #[test]
      fn fail_lines_from_stream_empty_stream() {
         let empty_reader = Cursor::new([]);
         let lines = lines_from_stream(empty_reader);
         assert!(lines.is_err());
      }

      #[tokio::test]
      async fn test_lines_from_stream() -> ErrStr<()> {
         let p = poem().await?;
         let lines: Vec<String> = p.lines().map(s).collect();
         assert_eq!(14, lines.len());
         Ok(())
      }
   }
}

