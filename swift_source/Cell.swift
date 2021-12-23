/// This struct represents the _Cell_ type. There are several variables
/// representing the cell's state.
public struct Cell {

	/// These values are populated in the field struct.
	var mine: Bool = false
	var neighbours: Int = 0

	/// The _revealed_ and _flagged_ parameter change throughout the game,
	/// as the player flags and reveals cells. Note that _flagged_ can be
	/// toggled, whereas _revealed_ is only set to true.
	var revealed: Bool = false
	var flagged: Bool = false

	/// This function sets the reveal state of a cell to true. This mutation
	/// is one way: once a cell has been revealed, it is not be hidden
	/// afterwards.
	public mutating func reveal() {
		self.revealed = true
	}
}
