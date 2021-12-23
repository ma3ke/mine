use console::Style;
use std::fmt;

/// This struct represents the _Cell_ type. There are several variables
/// representing the cell's state.
#[derive(Clone)]
pub struct Cell {
    /// These values are populated in the field struct.
    mine: bool, // default = false
    pub(crate) neighbours: usize, // default = 0

    /// The _revealed_ and _flagged_ parameter change throughout the game,
    /// as the player flags and reveals cells. Note that _flagged_ can be
    /// toggled, whereas _revealed_ is only set to true.
    revealed: bool, // default = false

    flagged: bool, // default = false
}

impl Cell {
    pub fn new() -> Self {
        Self {
            mine: false,
            neighbours: 0,
            revealed: false,
            flagged: false,
        }
    }

    pub fn new_mine() -> Self {
        Self {
            mine: true,
            neighbours: 0,
            revealed: false,
            flagged: false,
        }
    }

    /// This function sets the reveal state of a cell to true. This mutation
    /// is one way: once a cell has been revealed, it is not be hidden
    /// afterwards.
    pub fn reveal(&mut self) {
        self.revealed = true
    }

    /// Returns `true` if the cell is a bomb.
    pub fn is_mine(&self) -> bool {
        self.mine
    }

    /// Returns the value of neighbours field of the cell.
    pub fn neighbours(&self) -> usize {
        self.neighbours
    }

    /// Returns `true` if the cell has been revealed.
    pub fn is_revealed(&self) -> bool {
        self.revealed
    }

    /// Returns `true` if the cell has been flagged.
    pub fn is_flagged(&self) -> bool {
        self.flagged
    }

    /// Toggles `flagged` state of a cell.
    pub fn toggle_flagged(&mut self) {
        self.flagged = !self.is_flagged()
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        /***
         * If the cell is revealed, show the underlying neighbours
         * count, or if the cell is actually a mine, show that it is a
         * mine. However, if the cell's _flagged_ parameter is set to
         * `true`, the flag will be shown, rather than its value.
         *
         * Note that all of the complicated conditionals implement
         * the coloring mechanism.
         *
         * On the coloring system:
         * - Mines are painted red and bold.
         * - Flags are painted red and bold-italic. Also, their color
         * is inversed.
         * - Unrevealed cells are left alone and are represented by ` .`.
         * - Revealed, empty cells are painted with a color depending
         * on their neigbour count. This makes them visually distinct.
         *
         * The coloring of the revealed cells is done by painting them
         * in the extended ascii color (a UInt8 value) that is derived
         * as follows:
         *		neighbours * 30
         *
         * Because, by definition, the neighbour count can never
         * exceed 8, the color integer will never exceed 8 * 30 = 240.
         * Therefore, `0 <= colorValue < 255 == u8::MAX`.
         *
         *
         * ## The logic
         *
         * (note: 'to place' is used as 'store representation as')
         *
         * if cell is in revealed state:
         *     if mine:
         *         place a bold, red 'M'
         *     else:
         *         place neighbours with color of neighbours*30 (over u8 range)
         *     if flagged:
         *         invert the placed cell
         * else if flagged:
         *     place an inverted, bold, italic, red 'F'
         * else:
         *     place '.'
         *
         * if cursor is on the cell, give the cell a yellow background color
         ***/
        let mut style = Style::new();
        let character: String;

        if self.is_revealed() {
            if self.is_mine() {
                // place a bold, red 'M'
                character = "M".to_string();
                style = style.red().bold();
            } else {
                // place neighbours with color of neighbours*30 (over u8 range)
                let nb = &self.neighbours();
                character = nb.to_string();
                style = style.color256(*nb as u8 * 30);
            }

            if self.is_flagged() {
                // invert the colors of the placed cell
                style = style.reverse();
            }
        } else if self.is_flagged() {
            // place an inverted, bold, italic, red 'F'
            character = "F".to_string();
            style = style.reverse().bold().italic().red();
        } else {
            // place '.'
            character = ".".to_string();
        }

        let styled_character = style.apply_to([" ", &character].concat());

        write!(f, "{}", styled_character)
    }
}

impl Cell {
    pub fn apply_cursor_styling(&self) -> String {
        Style::new()
            .on_yellow()
            .apply_to(format!("{}", self))
            .to_string()
    }
}
