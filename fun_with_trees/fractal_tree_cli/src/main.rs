use std::{fs, io::Write};
use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
   /// Seed for generating the tree
   #[arg(short, long, default_value = "Hello World")]
   seed: String,

   /// Name of target file
   #[arg(short, long, default_value = "tree.svg")]
   target_file_name: String,

   /// Depth of the tree
   #[arg(short = 'd', long, default_value_t = 13usize)]
   tree_depth: usize,

   /// Trunk length
   #[arg(short = 'l', long, default_value_t = 100f64)]
   trunk_length: f64,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let buffer = fractal_tree::tree_svg(args.seed, args.tree_depth, args.trunk_length);
    let mut file =
        fs::File::create(args.target_file_name)?;
    file.write_all(buffer.as_bytes())?;
    Ok(())
}
