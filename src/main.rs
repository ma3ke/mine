#[cfg(feature = "gui")]
use crate::mode::gui::gui;
#[cfg(feature = "tui")]
use crate::mode::tui::tui;

use structopt::StructOpt;

mod cell;
mod field;
mod mode;

/// Terminal interface for Mine
#[cfg(feature = "tui")]
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
#[cfg(feature = "gui")]
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
    #[cfg(feature = "tui")]
    Tui(Tui),
    #[cfg(feature = "gui")]
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
        #[cfg(feature = "tui")]
        Command::Tui(opt) => tui(opt),
        #[cfg(feature = "gui")]
        Command::Gui(opt) => gui(opt),
    }
}
