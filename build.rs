use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;

const NUM_DAYS: usize = 8;

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("commands.rs");
    let mut f = File::create(&dest_path).unwrap();

    writeln!(f, "#[derive(Debug, StructOpt)]")?;
    writeln!(f, "enum Opts {{")?;

    for i in 1..=NUM_DAYS {
        writeln!(f, "    Day{:02}(day{:02}::Day{:02}),", i, i, i)?;
    }

    writeln!(f, "}}")?;
    writeln!(f, "")?;

    writeln!(f, "pub fn parse_args() -> Box<dyn Command> {{")?;
    writeln!(f, "    match Opts::from_args() {{")?;

    for i in 1..=NUM_DAYS {
        writeln!(
            f,
            "        Opts::Day{:02}(command) => Box::new(command),",
            i
        )?;
    }

    writeln!(f, "    }}")?;
    writeln!(f, "}}")?;

    Ok(())
}
