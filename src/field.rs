use crate::cell::Cell;
use rand::prelude::{thread_rng, SliceRandom};
use std::collections::HashMap;
use std::fmt;

/// The _Field_ class represents the play field made up of cells.
#[derive(Clone, PartialEq, Eq)]
pub struct Field {
    height: usize,
    width: usize,
    game_over: bool, // default = false
    cells: Vec<Cell>,
    cursor_pos: (usize, usize), // default = (x: 0, y: 0)
}

pub enum Edge {
    Left,
    Right,
    Up,
    Down,
}

impl Field {
    /// Initialises the array of cells with a (pseudo) random distribution of mines.
    pub fn new(height: usize, width: usize, mines: usize) -> Self {
        let mine_cells = vec![true; mines];
        let non_mine_cells = vec![false; height * width - mines];

        let mut rng = thread_rng();
        let mut is_mine_vec = [mine_cells, non_mine_cells].concat();
        is_mine_vec.shuffle(&mut rng);

        let cells = is_mine_vec
            .iter()
            .map(|m| match m {
                true => Cell::new_mine(),
                false => Cell::new(),
            })
            .collect();

        let mut f = Self {
            height: height.max(1),
            width: width.max(1),
            game_over: false,
            cursor_pos: (0, 0),
            cells,
        };
        // Call the `initializeNeighbours()` function to populate the cells in the cells array with
        // the proper neigbour count. The neigbour count represents the number of neigbouring
        // mines.
        f.initialize_neighbours();
        f
    }

    /// Returns the total number of flags in the field.
    pub fn total_flags(&self) -> usize {
        self.cells.iter().filter(|c| c.is_flagged()).count()
    }

    /// Returns the total number of mines in the field.
    pub fn total_mines(&self) -> usize {
        self.cells.iter().filter(|c| c.is_mine()).count()
    }

    /// Returns the number of mines in the field subtracted by the number of flags placed.
    pub fn mines_left(&self) -> isize {
        let mines = self.total_mines();
        (mines as isize) - (self.total_flags() as isize)
    }

    /// Returns the cell at a given position.
    pub fn get_cell(&self, x: usize, y: usize) -> Cell {
        let index = y * self.width + x;
        // TODO: Can we impl copy on this?
        self.cells[index].clone()
    }

    pub fn cells(&self) -> &Vec<Cell> {
        &self.cells
    }

    fn cells_mut(&mut self) -> &mut Vec<Cell> {
        &mut self.cells
    }

    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns y value of the cursor position in the field.
    pub fn cursor_pos_x(&self) -> usize {
        self.cursor_pos.0
    }

    /// Returns y value of the cursor position in the field.
    pub fn cursor_pos_y(&self) -> usize {
        self.cursor_pos.1
    }

    /// This function will reveal the cell at the specified position. If the
    /// cell does not have neighbours, it will also reveal its neighbours.
    ///
    /// Note that this means this is a recursive function depending on the cell
    /// that was called.
    ///
    /// If, however, the cell contains a mine, the field's `gameOver` variable
    /// will be set to `true`, signalling the end of the game.
    ///
    /// Because of this implementation, this function recreates the typical
    /// flood fill behaviour seen in many minesweepers.
    pub fn reveal(&mut self, x: usize, y: usize) {
        let index = y * self.width + x;
        if self.cells[index].is_flagged() {
            // If flagged, the cell cannot be revealed. That's the whole point kinda.
            return;
        }

        // Cell is not flagged. Reveal the cell.
        self.cells[index].reveal();

        // If the cell contains a mine... BOOOOM!!!! The game is over :(
        if self.cells[index].is_mine() {
            self.game_over = true
        }

        // If the current cell is not surrounded by any mines, the number of
        // neighbours is zero. In that case, reveal every neigbour.
        //
        // The result of this behaviour is that once a zero cell has been
        // revealed, the whole field of zeroes and its adjacent cells will be
        // revealed. Therefore this implements the typical flood fill
        // behaviour seen in minesweeper.
        if self.cells[index].neighbours() == 0 {
            for y_offset in -1..=1 {
                // Make sure the selected cell is within vertical bounds.
                if y as isize + y_offset < 0 || y as isize + y_offset >= self.height as isize {
                    continue;
                }

                for x_offset in -1..=1 {
                    // Make sure the selected cell is within horizontal bounds.
                    if x as isize + x_offset < 0 || x as isize + x_offset >= self.width as isize {
                        continue;
                    }

                    // If cell has already been revealed, do nothing.
                    // Else, reveal the selected cell. (Note that this
                    // will recursively call this function.)
                    if self
                        .get_cell(
                            (x as isize + x_offset) as usize,
                            (y as isize + y_offset) as usize,
                        )
                        .is_revealed()
                    {
                        continue;
                    } else {
                        let i =
                            index as isize + x_offset as isize + (y_offset * self.width as isize);
                        // Reveal the current cell
                        self.cells[i as usize].reveal();

                        // Recursively call this function on all neighbours of
                        // the initial cell.
                        self.reveal(
                            (x as isize + x_offset) as usize,
                            (y as isize + y_offset) as usize,
                        );
                    }
                }
            }
        }
    }

    /// This function reveals all cells in the field.
    pub fn reveal_all(&mut self) {
        for cell in self.cells_mut() {
            cell.reveal()
        }
    }

    /// Reveals the field from a given cell, expanding around cells where 0 neighbours are
    /// encountered.
    pub fn reveal_from_cell(&mut self, x: usize, y: usize) {
        // struct Pos: Hashable {
        //     var x: Int
        //     var y: Int
        // }
        type Pos = (usize, usize); // (x, y)

        // index - w - 1    index - w     index - w + 1
        // index - 1        .             index + 1
        // index + w - 1    index + w     index + w + 1
        let mut adjacent_cells: HashMap<Pos, Cell> = HashMap::new();

        let mut mines = 0;
        let mut flags = 0;
        for y_offset in -1..=1 {
            if y as isize + y_offset < 0 || y as isize + y_offset >= self.height as isize {
                continue;
            }

            for x_offset in -1..=1 {
                if x as isize + x_offset < 0 || x as isize + x_offset >= self.width as isize {
                    continue;
                }

                let cell = self.get_cell(
                    (x as isize + x_offset) as usize,
                    (y as isize + y_offset) as usize,
                );
                adjacent_cells.insert(
                    (
                        (x as isize + x_offset) as usize,
                        (y as isize + y_offset) as usize,
                    ),
                    cell.clone(),
                );

                if cell.is_mine() {
                    mines += 1
                }
                if cell.is_flagged() {
                    flags += 1
                }
            }
        }

        if mines == flags {
            for element in adjacent_cells.iter().filter(|e| !e.1.is_flagged()) {
                let p = element.0;
                self.reveal(p.0, p.1)
            }
        }
    }

    /// This function toggles the `flag` state of a given cell within a field.
    pub fn flag(&mut self, x: usize, y: usize) {
        let index = y * self.width + x;

        self.cells[index].toggle_flagged()
    }

    /// This function returns an integer value representing the number of
    /// neighbouring mines for a given _index_.
    fn count_neighbours(&self, index: usize) -> usize {
        // index - w - 1    index - w     index - w + 1
        // index - 1        .             index + 1
        // index + w - 1    index + w     index + w + 1
        let mut count = 0;
        let y: isize = (index / self.width) as isize;
        let x: isize = (index % self.width) as isize;

        for y_offset in -1..=1 {
            if y + y_offset < 0 || y + y_offset >= self.height as isize {
                continue;
            }

            for x_offset in -1..=1 {
                if x + x_offset < 0 || x + x_offset >= self.width as isize {
                    continue;
                }

                let cell = self.get_cell((x + x_offset) as usize, (y + y_offset) as usize);
                if cell.is_mine() {
                    count += 1
                }
            }
        }

        count
    }

    /// This function will iterate over every cell in the field and write the count of neighbouring
    /// mines to each cell. This initializes the cell array.
    fn initialize_neighbours(&mut self) {
        for index in 0..self.cells.len() {
            self.cells[index].neighbours = self.count_neighbours(index)
        }
    }

    /*
     * The translations
     */

    #[inline]
    /// Returns `true` when the position is legitimate within the boundaries of the field.
    fn valid_translation(&self, x: isize, y: isize) -> bool {
        let new_x = self.cursor_pos.0 as isize + x;
        let new_y = self.cursor_pos.1 as isize + y;

        let not_within_width = 0 > new_x || new_x >= self.width as isize;
        let not_within_height = 0 > new_y || new_y >= self.height as isize;

        !not_within_width && !not_within_height
    }

    #[inline]
    pub fn translate_x(&mut self, translation: isize) {
        if self.valid_translation(translation, 0) {
            self.cursor_pos.0 = (self.cursor_pos_x() as isize + translation) as usize
        }
    }

    #[inline]
    pub fn translate_y(&mut self, translation: isize) {
        if self.valid_translation(0, translation) {
            self.cursor_pos.1 = (self.cursor_pos_y() as isize + translation) as usize
        }
    }

    #[inline]
    pub fn move_cursor_to_edge(&mut self, edge: Edge) {
        match edge {
            Edge::Left => self.cursor_pos.0 = 0,
            Edge::Right => self.cursor_pos.0 = self.width - 1,
            Edge::Up => self.cursor_pos.1 = 0,
            Edge::Down => self.cursor_pos.1 = self.height - 1,
        }
    }

    /// This function returns `true` if the current field indicates that the player has won. This
    /// is determined by the following heuristic: _Is every non-mine cell revealed? If so, the
    /// player has won._
    pub fn has_won(&self) -> bool {
        for cell in &self.cells {
            // If a cell is a mine, skip it for checking whether it has been revealed.
            if cell.is_mine() {
                continue;
            }
            // If any non-mine cell is still not revealed, the game is not won.
            if !cell.is_revealed() {
                return false;
            }
        }

        true
    }

    /// Returns true if the game is over.
    pub fn is_game_over(&self) -> bool {
        // TODO: here we just read out the value as set during the reveal() function. However, is
        // it not better to check whether any mines have been revealed over the whole field? Sounds
        // more correct. But I cannot, on the other hand, think of any way the field can invalidly
        // have a false game_over value but also contain revealed mines. Hmm...
        self.game_over
    }
}

pub enum Action {
    CursorUp,
    CursorDown,
    CursorLeft,
    CursorRight,

    CursorToEdgeUp,
    CursorToEdgeDown,
    CursorToEdgeLeft,
    CursorToEdgeRight,

    Flag,
    Reveal,
    RevealAround,
}

impl Field {
    pub fn apply_action(&mut self, action: Action) {
        match action {
            Action::CursorUp => self.translate_y(-1),
            Action::CursorDown => self.translate_y(1),
            Action::CursorLeft => self.translate_x(-1),
            Action::CursorRight => self.translate_x(1),

            Action::CursorToEdgeUp => self.move_cursor_to_edge(Edge::Up),
            Action::CursorToEdgeDown => self.move_cursor_to_edge(Edge::Down),
            Action::CursorToEdgeLeft => self.move_cursor_to_edge(Edge::Left),
            Action::CursorToEdgeRight => self.move_cursor_to_edge(Edge::Right),

            Action::Flag => self.flag(self.cursor_pos_x(), self.cursor_pos_y()),
            Action::Reveal => {
                self.reveal(self.cursor_pos_x(), self.cursor_pos_y());
            }
            Action::RevealAround => self.reveal_from_cell(self.cursor_pos_x(), self.cursor_pos_y()),
        }
    }
}

// TODO: Move this to cell.rs?
pub enum CellState {
    Flagged,
    RevealedMine,
    Neighbours(usize),
    Hidden,
}

impl Cell {
    pub fn cell_state(&self) -> CellState {
        if self.is_flagged() {
            CellState::Flagged
        } else if self.is_revealed() {
            if self.is_mine() {
                CellState::RevealedMine
            } else {
                CellState::Neighbours(self.neighbours())
            }
        } else {
            CellState::Hidden
        }
    }
}

// TODO: Move to some game.rs?
pub enum GameState {
    // TODO: These names should be improved.
    Running,
    GameOver,
    Won,
}

impl Field {
    pub fn game_state(&self) -> GameState {
        if self.is_game_over() {
            GameState::GameOver
        } else if self.has_won() {
            GameState::Won
        } else {
            GameState::Running
        }
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        /***
         * The logic, as seen in the original Swift file:
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

        let mut rows: Vec<String> = Vec::new();
        for y in 0..self.height {
            let mut row: Vec<String> = Vec::new();
            for x in 0..self.width {
                let cell = self.get_cell(x, y);

                let cell_repr = if self.cursor_pos == (x, y) {
                    // if cursor is on the cell, give the cell a yellow background color
                    cell.apply_cursor_styling()
                } else {
                    format!("{}", cell)
                };

                row.push(cell_repr);
            }
            rows.push(row.concat());
        }

        write!(f, "{}", rows.join("\n"))
    }
}
