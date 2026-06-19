mod node_to_json;

use clap::Parser;
use std::fs;
use std::path::PathBuf;
use typst_syntax::parse;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    source: PathBuf,

    #[arg(long)]
    json: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let source_code = fs::read_to_string(args.source)?;
    let ast = parse(&source_code);

    if args.json {
        let value = node_to_json::node_to_json(&ast);
        println!("{}", serde_json::to_string_pretty(&value)?);
    } else {
        println!("{:#?}", ast);
    }

    Ok(())
}
