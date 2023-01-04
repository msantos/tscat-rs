use clap::Parser;
use std::io;
use std::io::Write;

use chrono::prelude::*;

/// timestamp stdin to stdout/stderr
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// stdout=1, stderr=2, both=3
    #[clap(short, long, default_value_t = 1)]
    output: u32,

    /// timestamp format (see strftime(3))
    #[clap(short, long, default_value = "%F%T%z")]
    format: String,

    /// behaviour if write buffer is full
    #[clap(short = 'W', long = "write-error", default_value = "block")]
    write_error: String,

    /// verbose mode
    #[clap(short, long)]
    verbose: bool,

    /// label
    label: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    run(&args)
}

fn run(args: &Args) -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = io::stdout();
    let mut stderr = io::stderr();
    let stdin = io::stdin();

    let mut buf = String::new();

    loop {
        buf.clear();
        let _len = match stdin.read_line(&mut buf) {
            Ok(0) => return Ok(()),
            Ok(n) => n,
            Err(err) => return Err(Box::new(err)),
        };

        let local: DateTime<Local> = Local::now();
        let mut ts = local.format(&args.format).to_string();

        ts.push(' ');
        match &args.label {
            Some(label) => {
                ts.push_str(label);
                ts.push(' ');
            }
            None => (),
        }
        ts.push_str(&buf);

        if args.output & 1 != 0 {
            stdout.write_all(ts.as_bytes())?;
            stdout.flush()?;
        }

        if args.output & 2 != 0 {
            stderr.write_all(ts.as_bytes())?;
            stderr.flush()?;
        }
    }
}
