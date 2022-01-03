use crate::field::{Edge, Field};
use console::{style, Key, Term};
use std::io::Write;
use structopt::StructOpt;

mod cell;
mod field;

/// _Mine_ by Koen Westendorp
#[derive(StructOpt)]
struct Opt {
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

fn main() -> Result<(), std::io::Error> {
    let opt = Opt::from_args();

    // Initialize some sensible default values.
    let width = opt.width;
    let height = opt.height;
    let mines = opt.mines;
    let mut f = Field::new(height, width, mines);

    let mut term = Term::stdout();
    term.set_title("mine");
    term.hide_cursor()?;
    // Close the alternative screen buffer again.
    print!("\u{1b}[?1049h");
    term.clear_screen()?;
    term.write_fmt(format_args!("\n{}", f))?;

    let mut previous_input = Key::Unknown;

    // gameloop
    //TODO: Why did I need this 'gameloop label again?
    'gameloop: loop {
        let input = term.read_key()?;

        // TODO: Let's just use characters, now, not this integer mess.
        match input {
            // Basic movement
            Key::Char('h') | Key::ArrowLeft => {
                // h — <
                f.translate_x(-1)
            }
            Key::Char('j') | Key::ArrowDown => {
                // j — v
                f.translate_y(1)
            }
            Key::Char('k') | Key::ArrowUp => {
                // k — ^
                f.translate_y(-1)
            }
            Key::Char('l') | Key::ArrowRight => {
                // l — >
                f.translate_x(1)
            }

            // Movements to edges
            Key::Char('H') | Key::Char('0') => {
                // H — <<
                f.move_cursor_to_edge(Edge::Left)
            }
            Key::Char('L') | Key::Char('$') => {
                // L — >>
                f.move_cursor_to_edge(Edge::Right)
            }
            Key::Char('G') => {
                // G — vv
                f.move_cursor_to_edge(Edge::Down)
            }
            Key::Char('g') => {
                // g — ^^
                f.move_cursor_to_edge(Edge::Up)
            }

            // Flag selected cell
            Key::Char('f') | Key::Char(' ') => {
                // f — SPACE
                f.flag(f.cursor_pos_x(), f.cursor_pos_y())
            }

            // Reveal selected cell
            Key::Char('r') | Key::Enter | Key::Tab => {
                // r — RETURN — TAB
                f.reveal(f.cursor_pos_x(), f.cursor_pos_y());

                // If the previous input and the current input are the same, when the cell is
                // attempted to be revealed, this is considered a double press. In that case, the
                // neighbouring cells are to be revealed too, when possible.
                if previous_input == input {
                    f.reveal_from_cell(f.cursor_pos_x(), f.cursor_pos_y())
                }
            }

            // Quit
            Key::Char('q') => {
                // q to quit
                break 'gameloop;
            }

            // Otherwise, do nothing
            _ => {}
        }

        term.clear_screen()?;

        if f.has_won() {
            // Win screen
            f.reveal_all();
            println!("{}", style("YOU WON!!!").color256(178).bold());
            term.write_fmt(format_args!("{}\n", f))?;
            println!("{}", style("press any key to exit").italic());
            let _ = term.read_char(); // get any key
            term.clear_screen()?;
            break 'gameloop;
        } else if f.is_game_over() {
            // Game over screen
            f.reveal_all();
            println!("{}", style("GAME OVER").color256(75).bold());
            term.write_fmt(format_args!("{}\n", f))?;
            println!("{}", style("press any key to exit").italic());
            let _ = term.read_char(); // get any key
            term.clear_screen()?;
            break 'gameloop;
        } else {
            // The game is not over, neither has it been won. Show the number of mines left, and
            // the total number of flags. Continue the game.
            println!(
                "{}",
                style(format!(
                    "{} out of {} mines left",
                    mines as isize - f.total_flags() as isize,
                    mines
                ))
                .color256(238)
            );
            term.write_fmt(format_args!("{}", f))?;
        }

        previous_input = input;
    }

    term.show_cursor()?;
    // Close the alternative screen buffer again.
    print!("\u{1b}[?1049l");

    Ok(())
}
