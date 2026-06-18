use clap::Parser;
use std::fs;
use typst_syntax::parse;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    source: String,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let source_code = fs::read_to_string(args.source)?;
    let ast = parse(&source_code);
    println!("{:#?}", ast);

    Ok(())
}
