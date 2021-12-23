import Foundation
import Chalk

/// Initialize some sensible default values.
var height = 10
var width = 10
var mines = 15

let args = CommandLine.arguments
print("\u{1b}[?1049h")
print("\u{1b}[?25l")
// s.openAlternativeScreenBuffer(true)

// s.hideCursor(true)
// defer {
	// s.hideCursor(false)
// }

/// Check whether there are any arguments at all.
/// If there are some arguments, use the arguments as init paramaters for the
/// field.
/// If there are _no_ arguments, present the welcome screen with width, height,
/// and # mines prompt.
if args.count < 2 {
	// Welcome screen

	let codeColor: UInt8 = 224
	let welcome = """

		\("MINESWEEPER", color: .extended(27), style: [.bold, .italic])
		\("by Koen Westendorp\nkoenw.gitlab.io", color: .extended(39))

		\("press \("RETURN", color: .extended(codeColor), style: .bold) to start (with default values)", style: .italic)
		\("or press \("SPACE", color: .extended(codeColor), style: .bold) to provide custom field parameters", style: .italic)

		Did you know you can use
		\("mine", color: .extended(codeColor), style: .bold) \("height width mines", color: .extended(codeColor), style: .italic)
		Use \("mine", color: .extended(codeColor), style: .bold) \("--help", color: .extended(codeColor), style: .italic) to see all available commands.
		"""

	print("\u{1b}[2J")
	// print("\u{1b}[2J")
	// print("\u{1b}[2J")
	print(String(repeating: "\n", count: 30))
	// print("\u{1b}[2J")
	print("\u{1b}[2J")
	print(s.center(welcome), terminator: "")

	let input = GetKeyPress()
	switch input {
	case 32:
		// SPACE
		// Field parameter prompt
		height = s.promptInt("height")
		width = s.promptInt("width")
		mines = s.promptInt("mines")

	case 10:
		// RETURN / ENTER
		fallthrough
	default:
		break
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

var s = Screen()
var f = Field(height: height, width: width, mines: mines)

var pos = (x: 0, y: 0)
let stream = InputStream()

let file = FileHandle.standardInput

s.clear()
print(f.gridString())

var input: Int = 0
var previousInput: Int = 0

gameLoop: for _ in 0..<10000 {
	input = GetKeyPress()

	switch input {
	case 104, 68:
		// h — <
		f.translate(x: -1)
	case 106, 66:
		// j — v
		f.translate(y: +1)
	case 107, 65:
		// k — ^
		f.translate(y: -1)
	case 108, 67:
		// l — >
		f.translate(x: +1)
	case 102, 32:
		// f — SPACE
		f.flag(x: f.cursorPos.x, y: f.cursorPos.y)
	case 114, 10, 9:
		// r — RETURN — TAB
		f.reveal(x: f.cursorPos.x, y: f.cursorPos.y)

		if previousInput == input {
			f.revealFromCell(x: f.cursorPos.x, y: f.cursorPos.y)
		}
	case 81:
		// Q to quit
		break gameLoop
	default:
		// print("none")
		break
	}

	// Clear Screen escape code
	s.clear()

	if f.hasWon() {
		// win screen
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
	}

	previousInput = input
}

print("Thanks for playing!")
// s.openAlternativeScreenBuffer(false)
print("\u{1b}[?25h", terminator: "")
print("\u{1b}[?1049l", terminator: "")
exit(0)

