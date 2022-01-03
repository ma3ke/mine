use crate::mode::{gui::gui, tui::tui};
use structopt::StructOpt;

mod cell;
mod field;
mod mode;

/// Terminal interface for Mine
#[derive(StructOpt)]
pub struct Tui {
    /// Field width.
    #[structopt(short, long, default_value = "9")]
    width: usize,

    /// Field height.
    #[structopt(short, long, default_value = "9")]
    height: usize,

    /// The number of mines to be placed in the field.
    #[structopt(short, long, default_value = "10")]
    mines: usize,
}

/// Graphical interface for Mine
#[derive(StructOpt)]
pub struct Gui {
    /// Field width.
    #[structopt(short, long, default_value = "9")]
    width: u32,

    /// Field height.
    #[structopt(short, long, default_value = "9")]
    height: u32,

    /// The number of mines to be placed in the field.
    #[structopt(short, long, default_value = "10")]
    mines: u32,
}

/// Mine: a minesweeper game for the terminal and gui.
#[derive(StructOpt)]
enum Command {
    Tui(Tui),
    Gui(Gui),
}

#[derive(StructOpt)]
#[structopt(name = "Mine", author = "Koen Westendorp")]
struct Opt {
    #[structopt(subcommand)]
    command: Command,
}

fn main() -> Result<(), std::io::Error> {
    let opt = Opt::from_args();

    match opt.command {
        Command::Tui(opt) => tui(opt),
        Command::Gui(opt) => gui(opt),
    }
}
