use std::fmt::{Formatter, Result, Show};
use std::from_str::FromStr;
#[deriving(Clone, PartialEq)]
/// A Javascript Keyword
pub enum Keyword {
    /// The `break` keyword
    KBreak,
    /// The `case` keyword
    KCase,
    /// The `catch` keyword
    KCatch,
    /// The `class` keyword, which is reserved for future use
    KClass,
    /// The `continue` keyword
    KContinue,
    /// The `debugger` keyword
    KDebugger,
    /// The `default` keyword
    KDefault,
    /// The `delete` keyword
    KDelete,
    /// The `do` keyword
    KDo,
    /// The `else` keyword
    KElse,
    /// The `enum` keyword
    KEnum,
    /// The `extends` keyword
    KExtends,
    /// The `finally` keyword
    KFinally,
    /// The `for` keyword
    KFor,
    /// The `function` keyword
    KFunction,
    /// The `if` keyword
    KIf,
    /// The `in` keyword
    KIn,
    /// The `instanceof` keyword
    KInstanceOf,
    /// The `import` keyword
    KImport,
    /// The `new` keyword
    KNew,
    /// The `return` keyword
    KReturn,
    /// The `super` keyword
    KSuper,
    /// The `switch` keyword
    KSwitch,
    /// The `this` keyword
    KThis,
    /// The `throw` keyword
    KThrow,
    /// The `try` keyword
    KTry,
    /// The `typeof` keyword
    KTypeOf,
    /// The `var` keyword
    KVar,
    /// The `void` keyword
    KVoid,
    /// The `while` keyword
    KWhile,
    /// The `with` keyword
    KWith
}
impl FromStr for Keyword {
    fn from_str(s: &str) -> Option<Keyword> {
        match s {
            "break" => Some(KBreak),
            "case" => Some(KCase),
            "catch" => Some(KCatch),
            "class" => Some(KClass),
            "continue" => Some(KContinue),
            "debugger" => Some(KDebugger),
            "default" => Some(KDefault),
            "delete" => Some(KDelete),
            "do" => Some(KDo),
            "else" => Some(KElse),
            "enum" => Some(KEnum),
            "extends" => Some(KExtends),
            "finally" => Some(KFinally),
            "for" => Some(KFor),
            "function" => Some(KFunction),
            "if" => Some(KIf),
            "in" => Some(KIn),
            "instanceof" => Some(KInstanceOf),
            "import" => Some(KImport),
            "new" => Some(KNew),
            "return" => Some(KReturn),
            "super" => Some(KSuper),
            "switch" => Some(KSwitch),
            "this" => Some(KThis),
            "throw" => Some(KThrow),
            "try" => Some(KTry),
            "typeof" => Some(KTypeOf),
            "var" => Some(KVar),
            "void" => Some(KVoid),
            "while" => Some(KWhile),
            "with" => Some(KWith),
            _ => None
        }
    }
}
impl Show for Keyword {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", match *self {
            KBreak => "break",
            KCase => "case",
            KCatch => "catch",
            KClass => "class",
            KContinue => "continue",
            KDebugger => "debugger",
            KDefault => "default",
            KDelete => "delete",
            KDo => "do",
            KElse => "else",
            KEnum => "enum",
            KExtends => "extends",
            KFinally => "finally",
            KFor => "for",
            KFunction => "function",
            KIf => "if",
            KIn => "in",
            KInstanceOf => "instanceof",
            KImport => "import",
            KNew => "new",
            KReturn => "return",
            KSuper => "super",
            KSwitch => "switch",
            KThis => "this",
            KThrow => "throw",
            KTry => "try",
            KTypeOf => "typeof",
            KVar => "var",
            KVoid => "void",
            KWhile => "while",
            KWith => "with"
        })
    }
}