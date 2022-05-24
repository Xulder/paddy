use clap::Parser;

mod cmd;

use cmd::Commands;

// TODO: QOL features and code refractor needed.
// TODO: Make this whole thing more Unix and parse arguments and subcommands.
// TODO: Allow batch of spritesheets. Look into parallelising with `rayon`.
/// Simple tool for doing common operations on sprites and spritesheets
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct PaddyCli {
    #[clap(subcommand)]
    command: Commands,
}

fn main() {
    let cli = PaddyCli::parse();

    match &cli.command {
        Commands::Pad {
            width,
            height,
            padding,
            sheet_path,
            output_path,
        } => cmd::pad_spritesheet(width, height, padding, sheet_path, output_path),
    }
}
