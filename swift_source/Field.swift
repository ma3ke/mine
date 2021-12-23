import Foundation
import Chalk

/// The _Field_ class represents the play field made up of cells.
public class Field {
	var height: Int
	var width: Int

	var gameOver: Bool = false
	var totalFlags: Int {
		get {
			cells.reduce(0) { prev, cell in
				prev + (cell.flagged ? 1 : 0)
			}
		}
	}

	var cells: [Cell]
	var cursorPos: (x: Int, y: Int) = (0, 0)

	public func getCell(x: Int, y: Int) -> Cell {
		let index = y * self.width + x
		let cell = cells[index]
		return cell
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
	public func reveal(x: Int, y: Int) {
		let index = y * self.width + x
		if cells[index].flagged == true {
			return
		}

		/// Reveal the cell.
		cells[index].reveal()

		/// If the cell contains a mine... BOOOOM!!!! The game is over :(
		if cells[index].mine  {
			self.gameOver = true
		}

		/// If the current cell is not surrounded by any mines, the number of
		/// neighbours is zero. In that case, reveal every neigbour.
		///
		/// The result of this behaviour is that once a zero cell has been
		/// revealed, the whole field of zeroes and its adjacent cells will be
		/// revealed. Therefore this implements the typical flood fill
		/// behaviour seen in minesweeper.
		if cells[index].neighbours == 0 {
			for yOffset in -1...1 {
				/// Make sure the selected cell is within vertical bounds.
				if y + yOffset < 0 || y + yOffset >= self.height {
					continue
				}

				for xOffset in -1...1 {
					/// Make sure the selected cell is within horizontal bounds.
					if x + xOffset < 0 || x + xOffset >= self.width {
						continue
					}

					/// If cell has already been revealed, do nothing.
					/// Else, reveal the selected cell. (Note that this
					/// will recursively call this function.)
					if getCell(x: x + xOffset, y: y + yOffset).revealed {
						continue
					} else {
						let i = index + xOffset + (yOffset * self.width)
						/// Reveal the current cell
						cells[i].reveal()

						/// Recursively call this function on all neighbours of
						/// the initial cell.
						reveal(x: x + xOffset, y: y + yOffset)
					}
				}
			}
		}

	}

	/// This function reveals all cells in the field.
	public func revealAll() {
		for (index, _) in cells.enumerated() {
			cells[index].reveal()
		}
	}

	public func revealFromCell(x: Int, y: Int) {
		struct Pos: Hashable {
			var x: Int
			var y: Int
		}

		// index - w - 1    index - w     index - w + 1
		// index - 1        .             index + 1
		// index + w - 1    index + w     index + w + 1
		var adjacentCells = [Pos: Cell]()

		var mines = 0
		var flags = 0
		for yOffset in -1...1 {
			if y + yOffset < 0 || y + yOffset >= self.height {
				continue
			}

			for xOffset in -1...1 {
				if x + xOffset < 0 || x + xOffset >= self.width {
					continue
				}

				let cell = getCell(x: x + xOffset, y: y + yOffset)
				adjacentCells[Pos(x: x + xOffset, y: y + yOffset)] = cell

				if cell.mine {
					mines += 1
				}
				if cell.flagged {
					flags += 1
				}
			}
		}

		if mines == flags {
			for element in adjacentCells where element.value.flagged == false {
				let p = element.key
				reveal(x: p.x, y: p.y)
			}
		}
	}

	/// This function toggles the `flag` state of a given cell within a field.
	public func flag(x: Int, y: Int) {
		let index = y * self.width + x

		cells[index].flagged.toggle()
	}

	public init(height: Int, width: Int, mines: Int) {
		self.height = height
		self.width = width

		/// Initialises the array of cells with a (pseudo) random distribution
		/// of mines.
		let mineArray: [Bool] = Array(repeating: true, count: mines)
		let noMineArray = Array(repeating: false, count: height * width - mines)
		var cellArray = [Cell]()
		for isMine in (mineArray + noMineArray).shuffled() {
			let newCell = Cell(mine: isMine, revealed: false)
			cellArray.append(newCell)
		}
		self.cells = cellArray

		/// Call the `initializeNeighbours()` function to populate the cells in
		/// the cells array with the proper neigbour count. The neigbour count
		/// represents the number of neigbouring mines.
		self.initializeNeighbours()
	}
}

extension Field {
	/// This function returns an integer value representing the number of
	/// neighbouring mines for a given _index_.
	func countNeighbours(atIndex index: Int) -> Int {
		// index - w - 1    index - w     index - w + 1
		// index - 1        .             index + 1
		// index + w - 1    index + w     index + w + 1
		var count = 0
		let y = (index / self.width)
		let x = index % self.width

		for yOffset in -1...1 {
			if y + yOffset < 0 || y + yOffset >= self.height {
				continue
			}

			for xOffset in -1...1 {
				if x + xOffset < 0 || x + xOffset >= self.width {
					continue
				}

				let cell = getCell(x: x + xOffset, y: y + yOffset)
				if cell.mine {
					count += 1
				}
			}
		}

		return count
	}

	/// This function will iterate over every cell in the field and write the
	/// count of neighbouring mines to each cell.
	/// This initializes the cell array.
	public func initializeNeighbours() {
		for index in 0..<cells.count {
			cells[index].neighbours = countNeighbours(atIndex: index)
		}
	}
}

extension Field {
	/// This function returns a string representing the current field.
	public func gridString() -> String {
		var rows = ""
		for y in 0..<height {
			var row = ""
			for x in 0..<width {
				let cell = getCell(x: x, y: y)
				var char: String

				/// If the cell is revealed, show the underlying neighbours
				/// count, or if the cell is actually a mine, show that it is a
				/// mine. However, if the cell's _flagged_ parameter is set to
				/// `true`, the flag will be shown, rather than its value.
				///
				/// Note that all of the complicated conditionals implement
				/// the coloring mechanism.
				///
				/// On the coloring system:
				/// - Mines are painted red and bold.
				/// - Flags are painted red and bold-italic. Also, their color
				/// is inversed.
				/// - Unrevealed cells are left alone and are represented by ` .`.
				/// - Revealed, empty cells are painted with a color depending
				/// on their neigbour count. This makes them visually distinct.
				///
				/// The coloring of the revealed cells is done by painting them
				/// in the extended ascii color (a UInt8 value) that is derived
				/// as follows:
				///		neighbours * 30
				///
				/// Because, by definition, the neighbour count can never
				/// exceed 8, the color integer will never exceed 8 * 30 = 240.
				/// Therefore, `0 <= colorValue < 255 == UInt8.max`.
				///
				if cell.revealed {
					// TODO: make this code more clear. Currently, it's pure spaghetti!!!
					char = cell.mine ? "\(" M", color: .red, style: .bold)" : "\(" \(cell.neighbours)", color: .extended(UInt8(cell.neighbours) * 30))"
					if cell.flagged == true {
						char = "\(char, style: .inverse)"
					}
				} else if cell.flagged {
					char = "\(" F", color: .red, style: [.bold, .italic, .inverse])"
				} else {
					char = " ."
				}

				if x == cursorPos.x && y == cursorPos.y {
					// give background color to the cursor
					char = "\(char, background: .yellow)"
				}

				row.append(char)
			}

			rows.append(row + "\n")
		}

		return rows
	}
}

extension Field {
	/// Returns `true` when the position is legitimate within the boundaries of the field.
	func validTranslation(x: Int, y: Int) -> Bool {
		let newX = self.cursorPos.x + x
		let newY = self.cursorPos.y + y

		if 0 > newX || newX >= width {
			return false
		} else if 0 > newY || newY >= height {
			return false
		}

		return true
	}

	public func translate(x translation: Int) {
		if validTranslation(x: translation, y: 0) {
			self.cursorPos.x += translation
		}
	}

	public func translate(y translation: Int) {
		if validTranslation(x: 0, y: translation) {
			self.cursorPos.y += translation
		}
	}
}

extension Field {
	/// This function returns `true` if the current field indicates that the
	/// player has won. This is determined by the following heuristic:
	/// _Is every non-mine cell revealed? If so, the player has won._
	public func hasWon() -> Bool {
		for cell in cells {
			if cell.mine == true {
				continue
			}
			if cell.revealed == false {
				return false
			}
		}

		return true
	}
}
