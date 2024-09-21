//! This program render a svg file and display result on a window.

use hala_graphics::{svg::render_svg, Viewport};

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Svg file path
    path: String,

    /// The display viewport dimension.
    #[arg(short, long, value_parser = clap::value_parser!(Viewport), default_value = "(1920,1080)")]
    viewport: Viewport,
}

fn main() {
    let cli = Cli::parse();
    pretty_env_logger::init_timed();
    render_svg(None, cli.path, cli.viewport).unwrap();
}
