import Chalk

/// This struct represents the screen. It contains a few convenience functions
/// for drawing to the terminal screen.
///
/// - `clear()` clears the screen.
/// - `center(_:)` centers the provided string.
/// - `promptInt(_:)` shows a prompt that returns an integer from the input.
struct Screen {
	let esc = "\u{1b}["

	public func clear() {
		print(esc + "2J")
	}

	// TODO:
	/// Center the string lines and return a new centered string.
	public func center(_ input: String) -> String {
		// do nothing for the time being...
		return input
	}

	/// This function will present a prompt for an integer value.
	/// If the value provided is not an integer, the prompt will be presented
	/// again.
	public func promptInt(_ tag: String) -> Int {
		while true {
			print("\(tag, style: .italic) -> ", terminator: "")
			let input = readLine()

			if input == nil {
				continue
			}

			if let n = Int(input!) {
				return n
			}
		}
	}

	// TODO:
	/// Experimental feature that does not function, yet. Taken verbatim from
	/// the [_CLISpinner_](https://github.com/kiliankoe/CLISpinner) library by [Kilian Koeltzsch](https://github.com/kiliankoe).
	/// This function hides the cursor when the game is played.
	public func hideCursor(_ hide: Bool) {
		if hide {
			print(esc + "?25l")
		} else {
			print(esc + "?25h")
		}
	}

	public func openAlternativeScreenBuffer(_ open: Bool) {
		if open {
			print(esc + "?1049h")
			print(esc + "H")
		} else {
			print(esc + "?1049l")
		}
	}
}
