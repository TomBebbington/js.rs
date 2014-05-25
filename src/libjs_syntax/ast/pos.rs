#[deriving(Clone, Eq)]
/// A position in Javascript source code
pub struct Position {
	/// The column number
	pub column_number : uint,
	/// The line number
	pub line_number : uint
}
impl Position {
	/// Create a new position
	pub fn new(line_number: uint, column_number: uint) -> Position {
		Position {
			line_number: line_number,
			column_number: column_number
		}
	}
}