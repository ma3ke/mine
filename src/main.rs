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

/***
    //let args = CommandLine.arguments;
    println!("\u{1b}[?1049h");
    println!("\u{1b}[?25l");

    // here we decide what to do from the start screen, but we ignore this for now. We first just
    // try to get it up and running with the default field size.
    //
    /*

    // Check whether there are any arguments at all.
    // If there are some arguments, use the arguments as init paramaters for the
    // field.
    // If there are _no_ arguments, present the welcome screen with width, height,
    // and # mines prompt.
    if args.count < 2 {
        // Welcome screen

        /*
       let codeColor: UInt8 = 224;
       let welcome = "
    \("MINESWEEPER", color: .extended(27), style: [.bold, .italic])
    \("by Koen Westendorp\nkoenw.gitlab.io", color: .extended(39))

    \("press \("RETURN", color: .extended(codeColor), style: .bold) to start (with default values)", style: .italic)
    \("or press \("SPACE", color: .extended(codeColor), style: .bold) to provide custom field parameters", style: .italic)

    Did you know you can use
    \("mine", color: .extended(codeColor), style: .bold) \("height width mines", color: .extended(codeColor), style: .italic)
    Use \("mine", color: .extended(codeColor), style: .bold) \("--help", color: .extended(codeColor), style: .italic) to see all available commands.
    "
      */
    let welcome = "MINESWEEPER
        by Koen Westendorp";

        println!("\u{1b}[2J");
        //println!(String(repeating: "\n", count: 30));
      println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
        println!("\u{1b}[2J");
        println!(s.center(welcome), terminator: "");

        let input = GetKeyPress();
        match input {
         32 => {
            // SPACE
            // Field parameter prompt
            height = s.promptInt("height");
            width = s.promptInt("width");
            mines = s.promptInt("mines");
    }
         10 => {
            // RETURN / ENTER
            fallthrough
    }
            _ => {
                break
            }

        }
    } else {
        if let h = Int(args[1]) {
            height = h
        }
        if let w = Int(args[2]) {
            width = w
        }
        if let m = Int(args[3]) {
            mines = m
        }
    }
      */

    let s = Screen::new();
    let f = Field::new(height, width, mines);

    let pos = (0, 0); // (x, y)

    // inputstream and standardinput stuff. This must be dealt with quite differently.
    /*
    let stream = InputStream();
    let file = FileHandle.standardInput;
    */

    s.clear();
    println!("{}", f);

    let input = 0;
    let previous_input = 0;

    // TODO: Boy deal with that 10000 here...
    for _ in 0..10000 {
        // gameloop
        input = get_key_press();

        match input {
            104 | 68 => {
                // h — <
                f.translate_x(-1)
            }
            106 | 66 => {
                // j — v
                f.translate_y(1)
            }
            107 | 65 => {
                // k — ^
                f.translate_y(-1)
            }
            108 | 67 => {
                // l — >
                f.translate_x(1)
            }
            102 | 32 => {
                // f — SPACE
                f.flag(f.cursor_pos_x(), f.cursor_pos_y())
            }
            114 | 10 | 9 => {
                // r — RETURN — TAB
                f.reveal(f.cursor_pos_x(), f.cursor_pos_y());

                if previous_input == input {
                    f.reveal_from_cell(f.cursor_pos_x(), f.cursor_pos_y())
                }
            }
            81 => {
                // Q to quit
                break; //gameLoop
            }
            _ => {
                // print("none")
                break;
            }
        }

        s.clear();

        if f.has_won() == true {
            // win screen
            /*
                f.revealAll()
                print("\("YOU WON!!!", color: .extended(178), style: .bold)")
                print(f.gridString())
                print("\u{1b}[F\("press any key to exit", style: .italic)")
                let _ = GetKeyPress()
                break gameLoop
            } else if f.gameOver == false {
                // print("\(f.cursorPos) — [\(input)]")
                print("\("left \(mines - f.totalFlags)", color: .extended(238))")
                print(f.gridString())
            } else { // if gameOver is true
                f.revealAll()
                print("\("GAME OVER", color: .extended(75), style: .bold)")
                print(f.gridString())
                print("\u{1b}[F\("press any key to exit", style: .italic)", terminator: "")
                let _ = GetKeyPress()
                break gameLoop
            */

            // temp so the mech is intact but the printing is broken here. see above for printing
            f.reveal_all();
            println!("YOU WON!!!");
            println!("{}", f);
            println!("press any key to exit");
            let _ = get_key_press(); // get any key
            break; // gameLoop
        } else if f.gameOver == false {
            println!("left {}", mines - f.totalFlags);
            println!("{}", f);
        } else {
            // if gameOver is true
            f.reveal_all();
            println!("YOU WON!!!");
            println!("{}", f);
            println!("press any key to exit");
            let _ = get_key_press(); // get any key
            break; // gameLoop
        }

        previous_input = input
    }

    println!("Thanks for playing!");
    // s.openAlternativeScreenBuffer(false)
    println!("\u{1b}[?25h");
    println!("\u{1b}[?1049l");
    // exit(0)
}
***/
