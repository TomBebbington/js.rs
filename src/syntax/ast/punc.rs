use std::fmt::{Formatter, Result, Show};
#[deriving(PartialEq, Clone)]
/// Punctuation
pub enum Punctuator {
    /// `{`
    POpenBlock,
    /// `}`
    PCloseBlock,
    /// `(`
    POpenParen,
    /// `)`
    PCloseParen,
    /// `[`
    POpenBracket,
    /// `]`
    PCloseBracket,
    /// `.`
    PDot,
    /// `;`
    PSemicolon,
    /// `,`
    PComma,
    /// `<`
    PLessThan,
    /// `>`
    PGreaterThan,
    /// `<=`
    PLessThanOrEq,
    /// `>=`
    PGreaterThanOrEq,
    /// `==`
    PEq,
    /// `!=`
    PNotEq,
    /// `===`
    PStrictEq,
    /// `!==`
    PStrictNotEq,
    /// `+` 
    PAdd,
    /// `-`
    PSub,
    /// `*`
    PMul,
    /// `/`
    PDiv,
    /// `%`
    PMod,
    /// `++`
    PInc,
    /// `--`
    PDec,
    /// `<<`
    PLeftSh,
    /// `>>`
    PRightSh,
    /// `>>>`
    PURightSh,
    /// `&`
    PAnd,
    /// `|`
    POr,
    /// `^`
    PXor,
    /// `!`
    PNot,
    /// `~`
    PNeg,
    /// `&&`
    PBoolAnd,
    /// `||`
    PBoolOr,
    /// `?`
    PQuestion,
    /// `:`
    PColon,
    /// `=`
    PAssign,
    /// `+=`
    PAssignAdd,
    /// `-=`
    PAssignSub,
    /// `*=`
    PAssignMul,
    /// `/=`
    PAssignDiv,
    /// `%=`
    PAssignMod,
    /// `<<=`
    PAssignLeftSh,
    /// `>>=`
    PAssignRightSh,
    /// `>>>=`
    PAssignURightSh,
    /// `&=`
    PAssignAnd,
    /// `|=`
    PAssignOr,
    /// `^=`
    PAssignXor,
    /// `=>`
    PArrow
}
impl Show for Punctuator {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", match *self {
            POpenBlock => "{",
            PCloseBlock => "}",
            POpenParen => "(",
            PCloseParen => ")",
            POpenBracket => "[",
            PCloseBracket => "]",
            PDot => ".",
            PSemicolon => ";",
            PComma => ",",
            PLessThan => "<",
            PGreaterThan => ">",
            PLessThanOrEq => "<=",
            PGreaterThanOrEq => ">=",
            PEq => "==",
            PNotEq => "!=",
            PStrictEq => "===",
            PStrictNotEq => "!==",
            PAdd => "+",
            PSub => "-",
            PMul => "*",
            PDiv => "/",
            PMod => "%",
            PInc => "++",
            PDec => "--",
            PLeftSh => "<<",
            PRightSh => ">>",
            PURightSh => ">>>",
            PAnd => "&",
            POr => "|",
            PXor => "^",
            PNot => "!",
            PNeg => "~",
            PBoolAnd => "&&",
            PBoolOr => "||",
            PQuestion => "?",
            PColon => ":",
            PAssign => "=",
            PAssignAdd => "+=",
            PAssignSub => "-=",
            PAssignMul => "*=",
            PAssignDiv => "/=",
            PAssignMod => "%=",
            PAssignLeftSh => "<<=",
            PAssignRightSh => ">>=",
            PAssignURightSh => ">>>=",
            PAssignAnd => "&=",
            PAssignOr => "|=",
            PAssignXor => "^=",
            PArrow => "=>"
        })
    }
}