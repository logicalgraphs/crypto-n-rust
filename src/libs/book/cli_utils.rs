use clap::{ ArgMatches, Command };

use colored::*;
use figlet_rs::FIGfont;

/// puts the banner into clap struct command-structure
pub fn add_banner(cmd: Command) -> ArgMatches {
   let custom_banner = generate_banner(&cmd);
   cmd.about(custom_banner.clone()).long_about(custom_banner).get_matches()
}

/// generates a banner for dapps, accessed through --help option in the shell
// called: generate_banner(&Args::command())
pub fn generate_banner(cmd: &Command) -> String {
   let app_name = cmd.get_name().to_string();
   let version = cmd.render_version();
   generate_banner_from_strs(&app_name, &version)
}

pub fn generate_banner_from_strs(app_name: &str, version: &str) -> String {
   // 1. Initialize a built-in font template style
   let font = FIGfont::standard().unwrap();

   // 2. Convert your text string into a structured banner object
   let text_block = font.convert(app_name).unwrap().to_string();

   // Build your custom multi-color gradient path
   let grad = colorgrad::CustomGradient::new()
                       // Purple -> Pink -> Cyan
       .html_colors(&["#8A2BE2", "#FF007F", "#00F5FF"])
       .build()
       .unwrap();

   let lines: Vec<&str> =
      text_block.lines().filter(|l| !l.is_empty()).collect();
   let total_lines = lines.len() as f64;

   // 3. Initialize a clean String buffer to accumulate the results
   let mut output_buffer = String::new();

    for (i, line) in lines.iter().enumerate() {
        let j = i as f64;
        // Calculate the ratio position for the gradient color step
        let t = if total_lines > 1.0 { j / (total_lines - 1.0) } else { 0.0 };
        let rgba = grad.at(t).to_lrgba_u8();
        
        // 4. Color the line and format it cleanly straight 
        //    into the buffer string
        let colorized_line = line.truecolor(rgba.0, rgba.1, rgba.2).bold();
        output_buffer.push_str(&format!("{}\n", colorized_line));
    }
   format!("{}\n\n{}", output_buffer, version)
}

// ----- TESTS -------------------------------------------------------

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod functional_tests {
   use super::*;
   use paste::paste;

   use crate::{ create_testing, err_utils::ErrStr };

   create_testing!("cli_utils");

   run!("generate_banner_from_strs", {
      let banner = generate_banner_from_strs("graphs", "graphs 1.0.1");
      println!("{banner}");
   });
}
