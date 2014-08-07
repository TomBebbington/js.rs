use syntax::ast::token::*;
use syntax::ast::expr::*;
use syntax::ast::constant::*;
use syntax::ast::op::*;
use syntax::ast::punc::*;
use syntax::ast::keyword::*;
use collections::treemap::TreeMap;
use std::fmt;
use std::vec::Vec;
macro_rules! mk (
    ($this:expr, $def:expr) => (
        Expr::new($def, try!($this.get_token($this.pos - 1)).pos, try!($this.get_token($this.pos - 1)).pos)
    );
    ($this:expr, $def:expr, $first:expr) => (
        Expr::new($def, $first.pos, try!($this.get_token($this.pos - 1)).pos)
    );
)
#[deriving(Clone, PartialEq)]
/// An error encountered during parsing an expression
pub enum ParseError {
    /// When it expected a certain kind of token, but got another as part of something
    Expected(Vec<TokenData>, Token, &'static str),
    /// When it expected a certain expression, but got another
    ExpectedExpr(&'static str, Expr),
    /// When it didn't expect this keyword
    UnexpectedKeyword(Keyword),
    /// When there is an abrupt end to the parsing
    AbruptEnd
}
impl fmt::Show for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Expected(ref wanted, ref got, ref routine) if wanted.len() == 0 => write!(f, "{}:{}: Expected expression for {}, got {}", got.pos.line_number, got.pos.column_number, routine, got.data),
            Expected(ref wanted, ref got, ref routine) => {
                try!(write!(f, "{}:{}: ", got.pos.line_number, got.pos.column_number));
                try!(write!(f, "Expected "));
                let last = wanted.last().unwrap();
                for wanted_token in wanted.iter() {
                    try!(write!(f, "'{}'{}", wanted_token, if wanted_token == last {""} else {", "}));
                }
                try!(write!(f, " for {}", routine));
                write!(f, " but got {}", got.data)
            },
            UnexpectedKeyword(ref key) => {
                write!(f, "Unexpected {}", key)
            }
            ExpectedExpr(ref wanted, ref got) => {
                write!(f, "Expected {}, but got {}", wanted, got)
            },
            AbruptEnd => {
                write!(f, "Abrupt end")
            }
        }
    }
}
pub type ParseResult = Result<Expr, ParseError>;
/// A Javascript parser
pub struct Parser {
    /// The tokens being input
    tokens: Vec<Token>,
    /// The current position within the tokens
    pos: uint
}
impl Parser {
    #[inline(always)]
    /// Creates a new parser, using `tokens` as input
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {tokens: tokens, pos: 0}
    }
    /// Parse all expressions in the token array
    pub fn parse_all(&mut self) -> ParseResult {
        let mut exprs = Vec::new();
        while self.pos < self.tokens.len() {
            let result = try!(self.parse());
            exprs.push(result);
        }
        Ok(mk!(self, BlockExpr(exprs)))
    }
    fn parse_struct(&mut self, keyword:Keyword) -> ParseResult {
        match keyword {
            KThrow => {
                let thrown = try!(self.parse());
                Ok(mk!(self, ThrowExpr(box thrown)))
            },
            KVar => {
                let mut vars = Vec::new();
                loop {
                    let name = match self.get_token(self.pos) {
                        Ok(Token { data: TIdentifier(ref name), ..}) => name.clone(),
                        Ok(tok) => return Err(Expected(vec!(TIdentifier("identifier".into_string())), tok, "var statement")),
                        Err(AbruptEnd) => break,
                        Err(e) => return Err(e)
                    };
                    self.pos += 1;
                    match self.get_token(self.pos) {
                        Ok(Token {data: TPunctuator(PAssign), ..}) => {
                            self.pos += 1;
                            let val = try!(self.parse());
                            vars.push((name, Some(val)));
                            match self.get_token(self.pos) {
                                Ok(Token {data: TPunctuator(PComma), ..}) => self.pos += 1,
                                _ => break
                            }
                        },
                        Ok(Token {data: TPunctuator(PComma), ..}) => {
                            self.pos += 1;
                            vars.push((name, None));
                        },
                        _ => {
                            vars.push((name, None));
                            break;
                        }
                    }
                }
                Ok(mk!(self, VarDeclExpr(vars)))
            },
            KReturn => Ok(mk!(self, ReturnExpr(Some(box try!(self.parse()).clone())))),
            KNew => {
                let call = try!(self.parse());
                match call.def {
                    CallExpr(ref func, ref args) => Ok(mk!(self, ConstructExpr(func.clone(), args.clone()))),
                    _ => Err(ExpectedExpr("constructor", call))
                }
            },
            KTypeOf => Ok(mk!(self, TypeOfExpr(box try!(self.parse())))),
            KIf => {
                try!(self.expect_punc(POpenParen, "if block"));
                let cond = try!(self.parse());
                try!(self.expect_punc(PCloseParen, "if block"));
                let expr = try!(self.parse());
                let next = self.get_token(self.pos + 1);
                Ok(mk!(self, IfExpr(box cond, box expr, if next.is_ok() && next.unwrap().data == TKeyword(KElse) {
                    self.pos += 2;
                    Some(box try!(self.parse()))
                } else {
                    None
                })))
            },
            KWhile => {
                try!(self.expect_punc(POpenParen, "while condition"));
                let cond = try!(self.parse());
                try!(self.expect_punc(PCloseParen, "while condition"));
                let expr = try!(self.parse());
                Ok(mk!(self, WhileLoopExpr(box cond, box expr)))
            },
            KSwitch => {
                try!(self.expect_punc(POpenParen, "switch value"));
                let value = self.parse();
                try!(self.expect_punc(PCloseParen, "switch value"));
                try!(self.expect_punc(POpenBlock, "switch block"));
                let mut cases = Vec::new();
                let mut default = None;
                while self.pos + 1 < self.tokens.len() {
                    let tok = try!(self.get_token(self.pos));
                    self.pos += 1;
                    match tok.data {
                        TKeyword(KCase) => {
                            let cond = self.parse();
                            let mut block = Vec::new();
                            try!(self.expect_punc(PColon, "switch case"));
                            loop {
                                match try!(self.get_token(self.pos)).data {
                                    TKeyword(KCase) | TKeyword(KDefault) => break,
                                    TPunctuator(PCloseBlock) => break,
                                    _ => block.push(try!(self.parse()))
                                }
                            }
                            cases.push((cond.unwrap(), block));
                        },
                        TKeyword(KDefault) => {
                            let mut block = Vec::new();
                            try!(self.expect_punc(PColon, "default switch case"));
                            loop {
                                match try!(self.get_token(self.pos)).data {
                                    TKeyword(KCase) | TKeyword(KDefault) => break,
                                    TPunctuator(PCloseBlock) => break,
                                    _ => block.push(try!(self.parse()))
                                }
                            }
                            default = Some(mk!(self, BlockExpr(block)));
                        },
                        TPunctuator(PCloseBlock) => break,
                        _ => return Err(Expected(vec!(TKeyword(KCase), TKeyword(KDefault), TPunctuator(PCloseBlock)), tok, "switch block"))
                    }
                }
                try!(self.expect_punc(PCloseBlock, "switch block"));
                Ok(mk!(self, SwitchExpr(box value.unwrap(), cases, match default {
                    Some(v) => Some(box v),
                    None => None
                })))
            },
            KFunction => {
                let tk = try!(self.get_token(self.pos));
                let name = match tk.data {
                    TIdentifier(ref name) => {
                        self.pos += 1;
                        Some(name.clone())
                    },
                    TPunctuator(POpenParen) => None,
                    _ => return Err(Expected(vec!(TIdentifier("identifier".into_string())), tk.clone(), "function name"))
                };
                try!(self.expect_punc(POpenParen, "function"));
                let mut args:Vec<String> = Vec::new();
                let mut tk = try!(self.get_token(self.pos));
                while tk.data != TPunctuator(PCloseParen) {
                    match tk.data {
                        TIdentifier(ref id) => args.push(id.clone()),
                        _ => return Err(Expected(vec!(TIdentifier("identifier".into_string())), tk.clone(), "function arguments"))
                    }
                    self.pos += 1;
                    if try!(self.get_token(self.pos)).data == TPunctuator(PComma) {
                        self.pos += 1;
                    }
                    tk = try!(self.get_token(self.pos));
                }
                self.pos += 1;
                let block = try!(self.parse());
                Ok(mk!(self, FunctionDeclExpr(name, args, box block)))
            },
            _ => Err(UnexpectedKeyword(keyword))
        }
    }
    /// Parse a single expression
    pub fn parse(&mut self) -> ParseResult {
        if self.pos > self.tokens.len() {
            return Err(AbruptEnd);
        }
        let token = try!(self.get_token(self.pos));
        self.pos += 1;
        let expr : Expr = match token.data {
            TPunctuator(PSemicolon) | TComment(_) if self.pos < self.tokens.len() => try!(self.parse()),
            TPunctuator(PSemicolon) | TComment(_) => mk!(self, ConstExpr(CUndefined)),
            TNumericLiteral(num) =>
                mk!(self, ConstExpr(CNum(num))),
            TNullLiteral =>
                mk!(self, ConstExpr(CNull)),
            TStringLiteral(text) =>
                mk!(self, ConstExpr(CString(text))),
            TBooleanLiteral(val) =>
                mk!(self, ConstExpr(CBool(val))),
            TIdentifier(ref s) if s.as_slice() == "undefined" =>
                mk!(self, ConstExpr(CUndefined)),
            TIdentifier(s) =>
                mk!(self, LocalExpr(s)),
            TKeyword(keyword) =>
                try!(self.parse_struct(keyword)),
            TPunctuator(POpenParen) => {
                match try!(self.get_token(self.pos)).data {
                    TPunctuator(PCloseParen) if try!(self.get_token(self.pos + 1)).data == TPunctuator(PArrow) => {
                        self.pos += 2;
                        let expr = try!(self.parse());
                        mk!(self, ArrowFunctionDeclExpr(Vec::new(), box expr), token)
                    },
                    _ => {
                        let next = try!(self.parse());
                        let next_tok = try!(self.get_token(self.pos));
                        self.pos += 1;
                        match next_tok.data {
                            TPunctuator(PCloseParen) => next,
                            TPunctuator(PComma) => { // at this point it's probably gonna be an arrow function
                                let mut args = vec!(match next.def {
                                    LocalExpr(name) => name,
                                    _ => "".into_string()
                                }, match try!(self.get_token(self.pos)).data {
                                    TIdentifier(ref id) => id.clone(),
                                    _ => "".into_string()
                                });
                                let mut expect_ident = true;
                                loop {
                                    self.pos += 1;
                                    let curr_tk = try!(self.get_token(self.pos));
                                    match curr_tk.data {
                                        TIdentifier(ref id) if expect_ident => {
                                            args.push(id.clone());
                                            expect_ident = false;
                                        },
                                        TPunctuator(PComma) => {
                                            expect_ident = true;
                                        },
                                        TPunctuator(PCloseParen) => {
                                            self.pos += 1;
                                            break;
                                        },
                                        _ if expect_ident => return Err(Expected(vec!(TIdentifier("identifier".into_string())), curr_tk, "arrow function")),
                                        _ => return Err(Expected(vec!(TPunctuator(PComma), TPunctuator(PCloseParen)), curr_tk, "arrow function"))
                                    }
                                }
                                try!(self.expect(TPunctuator(PArrow), "arrow function"));
                                let expr = try!(self.parse());
                                mk!(self, ArrowFunctionDeclExpr(args, box expr), token)
                            }
                            _ => return Err(Expected(vec!(TPunctuator(PCloseParen)), next_tok, "brackets"))
                        }
                    }
                }
            },
            TPunctuator(POpenBracket) => {
                let mut array : Vec<Expr> = Vec::new();
                let mut expect_comma_or_end = try!(self.get_token(self.pos)).data == TPunctuator(PCloseBracket);
                loop {
                    let token = try!(self.get_token(self.pos));
                    if token.data == TPunctuator(PCloseBracket) && expect_comma_or_end {
                        self.pos += 1;
                        break;
                    } else if token.data == TPunctuator(PComma) && expect_comma_or_end {
                        expect_comma_or_end = false;
                    } else if token.data == TPunctuator(PComma) && !expect_comma_or_end {
                        array.push(mk!(self, ConstExpr(CNull)));
                        expect_comma_or_end = false;
                    } else if expect_comma_or_end {
                        return Err(Expected(vec!(TPunctuator(PComma), TPunctuator(PCloseBracket)), token.clone(), "array declaration"));
                    } else {
                        let parsed = try!(self.parse());
                        self.pos -= 1;
                        array.push(parsed);
                        expect_comma_or_end = true;
                    }
                    self.pos += 1;
                }
                mk!(self, ArrayDeclExpr(array), token)
            },
            TPunctuator(POpenBlock) if try!(self.get_token(self.pos)).data == TPunctuator(PCloseBlock) => {
                self.pos += 1;
                mk!(self, ObjectDeclExpr(box TreeMap::new()), token)
            },
            TPunctuator(POpenBlock) if try!(self.get_token(self.pos + 1)).data == TPunctuator(PColon) => {
                let mut map = box TreeMap::new();
                while try!(self.get_token(self.pos - 1)).data == TPunctuator(PComma) || map.len() == 0 {
                    let tk = try!(self.get_token(self.pos));
                    let name = match tk.data {
                        TIdentifier(ref id) => id.clone(),
                        TStringLiteral(ref str) => str.clone(),
                        _ => return Err(Expected(vec!(TIdentifier("identifier".into_string()), TStringLiteral("string".into_string())), tk, "object declaration"))
                    };
                    self.pos += 1;
                    try!(self.expect(TPunctuator(PColon), "object declaration"));
                    let value = try!(self.parse());
                    map.insert(name, value);
                    self.pos += 1;
                }
                mk!(self, ObjectDeclExpr(map), token)
            },
            TPunctuator(POpenBlock) => {
                let mut exprs = Vec::new();
                loop {
                    if try!(self.get_token(self.pos)).data == TPunctuator(PCloseBlock) {
                        break;
                    } else {
                        exprs.push(try!(self.parse()));
                    }
                }
                self.pos += 1;
                mk!(self, BlockExpr(exprs), token)
            },
            TPunctuator(PSub) =>
                mk!(self, UnaryOpExpr(UnaryMinus, box try!(self.parse()))),
            TPunctuator(PAdd) =>
                mk!(self, UnaryOpExpr(UnaryPlus, box try!(self.parse()))),
            TPunctuator(PNot) =>
                mk!(self, UnaryOpExpr(UnaryNot, box try!(self.parse()))),
            TPunctuator(PInc) =>
                mk!(self, UnaryOpExpr(UnaryIncrementPre, box try!(self.parse()))),
            TPunctuator(PDec) =>
                mk!(self, UnaryOpExpr(UnaryDecrementPre, box try!(self.parse()))),
            _ => return Err(Expected(Vec::new(), token.clone(), "script"))
        };
        if self.pos >= self.tokens.len() {
            Ok(expr)
        } else {
            self.parse_next(expr)
        }
    }
    fn get_token(&self, pos:uint) -> Result<Token, ParseError> {
        if pos < self.tokens.len() {
            Ok(self.tokens.get(pos).clone())
        } else {
            Err(AbruptEnd)
        }
    }
    fn parse_next(&mut self, expr:Expr) -> ParseResult {
        let next = try!(self.get_token(self.pos));
        let mut carry_on = true;
        let mut result = expr.clone();
        match next.data {
            TPunctuator(PDot) => {
                self.pos += 1;
                let tk = try!(self.get_token(self.pos));
                match tk.data {
                    TIdentifier(ref s) => result = mk!(self, GetConstFieldExpr(box expr, s.to_string())),
                    _ => return Err(Expected(vec!(TIdentifier("identifier".into_string())), tk, "field access"))
                }
                self.pos += 1;
            },
            TPunctuator(POpenParen) => {
                let mut args = Vec::new();
                let mut expect_comma_or_end = try!(self.get_token(self.pos + 1)).data == TPunctuator(PCloseParen);
                loop {
                    self.pos += 1;
                    let token = try!(self.get_token(self.pos));
                    if token.data == TPunctuator(PCloseParen) && expect_comma_or_end {
                        self.pos += 1;
                        break;
                    } else if token.data == TPunctuator(PComma) && expect_comma_or_end {
                        expect_comma_or_end = false;
                    } else if expect_comma_or_end {
                        return Err(Expected(vec!(TPunctuator(PComma), TPunctuator(PCloseParen)), token, "function call arguments"));
                    } else {
                        let parsed = try!(self.parse());
                        self.pos -= 1;
                        args.push(parsed);
                        expect_comma_or_end = true;
                    }
                }
                result = mk!(self, CallExpr(box expr, args));
            },
            TPunctuator(PQuestion) => {
                self.pos += 1;
                let if_e = try!(self.parse());
                try!(self.expect(TPunctuator(PColon), "if expression"));
                let else_e = try!(self.parse());
                result = mk!(self, IfExpr(box expr, box if_e, Some(box else_e)));
            },
            TPunctuator(POpenBracket) => {
                self.pos += 1;
                let index = try!(self.parse());
                try!(self.expect(TPunctuator(PCloseBracket), "array index"));
                result = mk!(self, GetFieldExpr(box expr, box index));
            },
            TPunctuator(PSemicolon) | TComment(_) => {
                self.pos += 1;
            },
            TPunctuator(PAssign) => {
                self.pos += 1;
                let next = try!(self.parse());
                result = mk!(self, AssignExpr(box expr, box next));
            },
            TPunctuator(PArrow) => {
                self.pos += 1;
                let mut args = Vec::with_capacity(1);
                match result.def {
                    LocalExpr(name) => args.push(name),
                    _ => return Err(ExpectedExpr("identifier", result))
                }
                let next = try!(self.parse());
                result = mk!(self, ArrowFunctionDeclExpr(args, box next));
            },
            TPunctuator(PAdd) =>
                result = try!(self.binop(BinNum(OpAdd), expr)),
            TPunctuator(PSub) =>
                result = try!(self.binop(BinNum(OpSub), expr)),
            TPunctuator(PMul) =>
                result = try!(self.binop(BinNum(OpMul), expr)),
            TPunctuator(PDiv) =>
                result = try!(self.binop(BinNum(OpDiv), expr)),
            TPunctuator(PMod) =>
                result = try!(self.binop(BinNum(OpMod), expr)),
            TPunctuator(PBoolAnd) =>
                result = try!(self.binop(BinLog(LogAnd), expr)),
            TPunctuator(PBoolOr) =>
                result = try!(self.binop(BinLog(LogOr), expr)),
            TPunctuator(PAnd) =>
                result = try!(self.binop(BinBit(BitAnd), expr)),
            TPunctuator(POr) =>
                result = try!(self.binop(BinBit(BitOr), expr)),
            TPunctuator(PXor) =>
                result = try!(self.binop(BinBit(BitXor), expr)),
            TPunctuator(PLeftSh) =>
                result = try!(self.binop(BinBit(BitShl), expr)),
            TPunctuator(PRightSh) =>
                result = try!(self.binop(BinBit(BitShr), expr)),
            TPunctuator(PEq) =>
                result = try!(self.binop(BinComp(CompEqual), expr)),
            TPunctuator(PNotEq) =>
                result = try!(self.binop(BinComp(CompNotEqual), expr)),
            TPunctuator(PStrictEq) =>
                result = try!(self.binop(BinComp(CompStrictEqual), expr)),
            TPunctuator(PStrictNotEq) =>
                result = try!(self.binop(BinComp(CompStrictNotEqual), expr)),
            TPunctuator(PLessThan) =>
                result = try!(self.binop(BinComp(CompLessThan), expr)),
            TPunctuator(PLessThanOrEq) =>
                result = try!(self.binop(BinComp(CompLessThanOrEqual), expr)),
            TPunctuator(PGreaterThan) =>
                result = try!(self.binop(BinComp(CompGreaterThan), expr)),
            TPunctuator(PGreaterThanOrEq) =>
                result = try!(self.binop(BinComp(CompGreaterThanOrEqual), expr)),
            TPunctuator(PInc) =>
                result = mk!(self, UnaryOpExpr(UnaryIncrementPost, box try!(self.parse()))),
            TPunctuator(PDec) =>
                result = mk!(self, UnaryOpExpr(UnaryDecrementPost, box try!(self.parse()))),
            _ => carry_on = false
        };
        if carry_on && self.pos < self.tokens.len() {
            self.parse_next(result)
        } else {
            Ok(result)
        }
    }
    fn binop(&mut self, op:BinOp, orig:Expr) -> Result<Expr, ParseError> {
        let (precedence, assoc) = op.get_precedence_and_assoc();
        self.pos += 1;
        let next = try!(self.parse());
        Ok(match next.def {
            BinOpExpr(ref op2, ref a, ref b) => {
                let other_precedence = op2.get_precedence();
                if precedence < other_precedence || (precedence == other_precedence && !assoc) {
                    mk!(self, BinOpExpr(*op2, b.clone(), box mk!(self, BinOpExpr(op.clone(), box orig, a.clone()))))
                } else {
                    mk!(self, BinOpExpr(op, box orig, box next.clone()))
                }
            },
            _ => mk!(self, BinOpExpr(op, box orig, box next))
        })
    }
    /// Returns an error if the next symbol is not `tk`
    fn expect(&mut self, tk:TokenData, routine:&'static str) -> Result<(), ParseError> {
        self.pos += 1;
        let curr_tk = try!(self.get_token(self.pos - 1));
        if curr_tk.data != tk {
            Err(Expected(vec!(tk), curr_tk, routine))
        } else {
            Ok(())
        }
    }
    /// Returns an error if the next symbol is not the punctuator `p`
    #[inline(always)]
    fn expect_punc(&mut self, p:Punctuator, routine:&'static str) -> Result<(), ParseError> {
        self.expect(TPunctuator(p), routine)
    }
}