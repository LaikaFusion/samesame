use std::io;
use std::env;

extern crate getopts;
use getopts::Options;

mod english_confusables;

fn print_usage(program: &str, opts: Options) {
    let use_text = format!("Usage: {} -i=IN_FILE [options]\n       \
                           {} [options] [TEXT]", program, program);
    print!("{}", opts.usage(&use_text));
}

fn main() {
  let mut input = String::new();
  let mut output = String::new();

  let args: Vec<String> = env::args().collect();
  println!("{:?}", args);

  let mut opts = Options::new();
  opts.optflag("d", "discrete", "use discrete mode, avoiding obvious homographs");
  opts.optopt("i", "", "set input file name", "IN_FILE");
  opts.optopt("o", "", "set output file name", "OUT_FILE");
  opts.optflag("v", "verbose", "use verbose mode");
  opts.optflag("h", "help", "print help menu");

  let opt_matches = match opts.parse(&args[1..]) {
    Ok(m) => { m }
    Err(f) => { panic!(f.to_string()) }
  };

  if opt_matches.opt_present("h") {
    print_usage(&args[0].clone(), opts);
    return;
  }
  match io::stdin().read_line(&mut input) {
    Ok(_n) => {
        output = english_confusables::map(input); 
        print!("{}", output);
    }
    Err(error) => println!("error: {}", error),

  }
}
