use std::fmt::{Formatter, Result, Show};

#[deriving(Clone, Eq)]
/// A Javascript constant
pub enum Const {
	/// A UTF-8 string, such as `"Hello, world"`
	CString(String),
	/// A regular expression, such as `/where('s| is) [wW]ally/`
	CRegExp(String, bool, bool),
	/// A 64-bit floating-point number, such as `3.1415`
	CNum(f64),
	/// A 32-bit integer, such as `42`
	CInt(i32),
	/// A boolean, which is either `true` or `false` and is used to check if criteria are met
	CBool(bool),
	/// The `null` value, which represents a non-existant value
	CNull,
	/// The `undefined` value, which represents a field or index that doesn't exist
	CUndefined
}
impl Show for Const {
	fn fmt(&self, f: &mut Formatter) -> Result {
		return match *self {
			CString(ref st) => write!(f, "\"{}\"", st),
			CRegExp(ref reg, _, _) => write!(f, "~/{}/", reg),
			CNum(num) => write!(f, "{}", num),
			CInt(num) => write!(f, "{}", num),
			CBool(v) => write!(f, "{}", v),
			CNull => write!(f, "null"),
			CUndefined => write!(f, "undefined")
		}
	}
}
