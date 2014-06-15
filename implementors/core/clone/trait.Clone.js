(function() {var implementors = {};
implementors['js.rs'] = ["impl <a class='trait' href='http://doc.rust-lang.org/core/clone/trait.Clone.html' title='core::clone::Clone'>Clone</a> for <a class='type' href='js.rs/ast/constant/type.Const.html' title='js.rs::ast::constant::Const'>Const</a>","impl <a class='trait' href='http://doc.rust-lang.org/core/clone/trait.Clone.html' title='core::clone::Clone'>Clone</a> for <a class='struct' href='js.rs/ast/expr/struct.Expr.html' title='js.rs::ast::expr::Expr'>Expr</a>","impl <a class='trait' href='http://doc.rust-lang.org/core/clone/trait.Clone.html' title='core::clone::Clone'>Clone</a> for <a class='type' href='js.rs/ast/expr/type.ExprDef.html' title='js.rs::ast::expr::ExprDef'>ExprDef</a>","impl <a class='trait' href='http://doc.rust-lang.org/core/clone/trait.Clone.html' title='core::clone::Clone'>Clone</a> for <a class='type' href='js.rs/ast/op/type.NumOp.html' title='js.rs::ast::op::NumOp'>NumOp</a>","impl <a class='trait' href='http://doc.rust-lang.org/core/clone/trait.Clone.html' title='core::clone::Clone'>Clone</a> for <a class='type' href='js.rs/ast/op/type.UnaryOp.html' title='js.rs::ast::op::UnaryOp'>UnaryOp</a>","impl <a class='trait' href='http://doc.rust-lang.org/core/clone/trait.Clone.html' title='core::clone::Clone'>Clone</a> for <a class='type' href='js.rs/ast/op/type.BitOp.html' title='js.rs::ast::op::BitOp'>BitOp</a>","impl <a class='trait' href='http://doc.rust-lang.org/core/clone/trait.Clone.html' title='core::clone::Clone'>Clone</a> for <a class='type' href='js.rs/ast/op/type.CompOp.html' title='js.rs::ast::op::CompOp'>CompOp</a>","impl <a class='trait' href='http://doc.rust-lang.org/core/clone/trait.Clone.html' title='core::clone::Clone'>Clone</a> for <a class='type' href='js.rs/ast/op/type.LogOp.html' title='js.rs::ast::op::LogOp'>LogOp</a>","impl <a class='trait' href='http://doc.rust-lang.org/core/clone/trait.Clone.html' title='core::clone::Clone'>Clone</a> for <a class='type' href='js.rs/ast/op/type.BinOp.html' title='js.rs::ast::op::BinOp'>BinOp</a>","impl <a class='trait' href='http://doc.rust-lang.org/core/clone/trait.Clone.html' title='core::clone::Clone'>Clone</a> for <a class='type' href='js.rs/ast/keyword/type.Keyword.html' title='js.rs::ast::keyword::Keyword'>Keyword</a>","impl <a class='trait' href='http://doc.rust-lang.org/core/clone/trait.Clone.html' title='core::clone::Clone'>Clone</a> for <a class='type' href='js.rs/ast/punc/type.Punctuator.html' title='js.rs::ast::punc::Punctuator'>Punctuator</a>","impl <a class='trait' href='http://doc.rust-lang.org/core/clone/trait.Clone.html' title='core::clone::Clone'>Clone</a> for <a class='struct' href='js.rs/ast/pos/struct.Position.html' title='js.rs::ast::pos::Position'>Position</a>","impl <a class='trait' href='http://doc.rust-lang.org/core/clone/trait.Clone.html' title='core::clone::Clone'>Clone</a> for <a class='struct' href='js.rs/ast/token/struct.Token.html' title='js.rs::ast::token::Token'>Token</a>","impl <a class='trait' href='http://doc.rust-lang.org/core/clone/trait.Clone.html' title='core::clone::Clone'>Clone</a> for <a class='type' href='js.rs/ast/token/type.TokenData.html' title='js.rs::ast::token::TokenData'>TokenData</a>","impl <a class='trait' href='http://doc.rust-lang.org/core/clone/trait.Clone.html' title='core::clone::Clone'>Clone</a> for <a class='type' href='js.rs/ast/types/type.Type.html' title='js.rs::ast::types::Type'>Type</a>","impl <a class='trait' href='http://doc.rust-lang.org/core/clone/trait.Clone.html' title='core::clone::Clone'>Clone</a> for <a class='type' href='js.rs/parser/type.ParseError.html' title='js.rs::parser::ParseError'>ParseError</a>",];
implementors['syntax'] = ["impl <a class='trait' href='http://doc.rust-lang.org/core/clone/trait.Clone.html' title='core::clone::Clone'>Clone</a> for <a class='type' href='syntax/ast/constant/type.Const.html' title='syntax::ast::constant::Const'>Const</a>","impl <a class='trait' href='http://doc.rust-lang.org/core/clone/trait.Clone.html' title='core::clone::Clone'>Clone</a> for <a class='struct' href='syntax/ast/expr/struct.Expr.html' title='syntax::ast::expr::Expr'>Expr</a>","impl <a class='trait' href='http://doc.rust-lang.org/core/clone/trait.Clone.html' title='core::clone::Clone'>Clone</a> for <a class='type' href='syntax/ast/expr/type.ExprDef.html' title='syntax::ast::expr::ExprDef'>ExprDef</a>","impl <a class='trait' href='http://doc.rust-lang.org/core/clone/trait.Clone.html' title='core::clone::Clone'>Clone</a> for <a class='type' href='syntax/ast/op/type.NumOp.html' title='syntax::ast::op::NumOp'>NumOp</a>","impl <a class='trait' href='http://doc.rust-lang.org/core/clone/trait.Clone.html' title='core::clone::Clone'>Clone</a> for <a class='type' href='syntax/ast/op/type.UnaryOp.html' title='syntax::ast::op::UnaryOp'>UnaryOp</a>","impl <a class='trait' href='http://doc.rust-lang.org/core/clone/trait.Clone.html' title='core::clone::Clone'>Clone</a> for <a class='type' href='syntax/ast/op/type.BitOp.html' title='syntax::ast::op::BitOp'>BitOp</a>","impl <a class='trait' href='http://doc.rust-lang.org/core/clone/trait.Clone.html' title='core::clone::Clone'>Clone</a> for <a class='type' href='syntax/ast/op/type.CompOp.html' title='syntax::ast::op::CompOp'>CompOp</a>","impl <a class='trait' href='http://doc.rust-lang.org/core/clone/trait.Clone.html' title='core::clone::Clone'>Clone</a> for <a class='type' href='syntax/ast/op/type.LogOp.html' title='syntax::ast::op::LogOp'>LogOp</a>","impl <a class='trait' href='http://doc.rust-lang.org/core/clone/trait.Clone.html' title='core::clone::Clone'>Clone</a> for <a class='type' href='syntax/ast/op/type.BinOp.html' title='syntax::ast::op::BinOp'>BinOp</a>","impl <a class='trait' href='http://doc.rust-lang.org/core/clone/trait.Clone.html' title='core::clone::Clone'>Clone</a> for <a class='type' href='syntax/ast/keyword/type.Keyword.html' title='syntax::ast::keyword::Keyword'>Keyword</a>","impl <a class='trait' href='http://doc.rust-lang.org/core/clone/trait.Clone.html' title='core::clone::Clone'>Clone</a> for <a class='type' href='syntax/ast/punc/type.Punctuator.html' title='syntax::ast::punc::Punctuator'>Punctuator</a>","impl <a class='trait' href='http://doc.rust-lang.org/core/clone/trait.Clone.html' title='core::clone::Clone'>Clone</a> for <a class='struct' href='syntax/ast/pos/struct.Position.html' title='syntax::ast::pos::Position'>Position</a>","impl <a class='trait' href='http://doc.rust-lang.org/core/clone/trait.Clone.html' title='core::clone::Clone'>Clone</a> for <a class='struct' href='syntax/ast/token/struct.Token.html' title='syntax::ast::token::Token'>Token</a>","impl <a class='trait' href='http://doc.rust-lang.org/core/clone/trait.Clone.html' title='core::clone::Clone'>Clone</a> for <a class='type' href='syntax/ast/token/type.TokenData.html' title='syntax::ast::token::TokenData'>TokenData</a>","impl <a class='trait' href='http://doc.rust-lang.org/core/clone/trait.Clone.html' title='core::clone::Clone'>Clone</a> for <a class='type' href='syntax/ast/types/type.Type.html' title='syntax::ast::types::Type'>Type</a>","impl <a class='trait' href='http://doc.rust-lang.org/core/clone/trait.Clone.html' title='core::clone::Clone'>Clone</a> for <a class='type' href='syntax/parser/type.ParseError.html' title='syntax::parser::ParseError'>ParseError</a>",];
implementors['js_syntax'] = ["impl <a class='trait' href='http://doc.rust-lang.org/core/clone/trait.Clone.html' title='core::clone::Clone'>Clone</a> for <a class='type' href='js_syntax/ast/constant/type.Const.html' title='js_syntax::ast::constant::Const'>Const</a>","impl <a class='trait' href='http://doc.rust-lang.org/core/clone/trait.Clone.html' title='core::clone::Clone'>Clone</a> for <a class='struct' href='js_syntax/ast/expr/struct.Expr.html' title='js_syntax::ast::expr::Expr'>Expr</a>","impl <a class='trait' href='http://doc.rust-lang.org/core/clone/trait.Clone.html' title='core::clone::Clone'>Clone</a> for <a class='type' href='js_syntax/ast/expr/type.ExprDef.html' title='js_syntax::ast::expr::ExprDef'>ExprDef</a>","impl <a class='trait' href='http://doc.rust-lang.org/core/clone/trait.Clone.html' title='core::clone::Clone'>Clone</a> for <a class='type' href='js_syntax/ast/op/type.NumOp.html' title='js_syntax::ast::op::NumOp'>NumOp</a>","impl <a class='trait' href='http://doc.rust-lang.org/core/clone/trait.Clone.html' title='core::clone::Clone'>Clone</a> for <a class='type' href='js_syntax/ast/op/type.UnaryOp.html' title='js_syntax::ast::op::UnaryOp'>UnaryOp</a>","impl <a class='trait' href='http://doc.rust-lang.org/core/clone/trait.Clone.html' title='core::clone::Clone'>Clone</a> for <a class='type' href='js_syntax/ast/op/type.BitOp.html' title='js_syntax::ast::op::BitOp'>BitOp</a>","impl <a class='trait' href='http://doc.rust-lang.org/core/clone/trait.Clone.html' title='core::clone::Clone'>Clone</a> for <a class='type' href='js_syntax/ast/op/type.CompOp.html' title='js_syntax::ast::op::CompOp'>CompOp</a>","impl <a class='trait' href='http://doc.rust-lang.org/core/clone/trait.Clone.html' title='core::clone::Clone'>Clone</a> for <a class='type' href='js_syntax/ast/op/type.LogOp.html' title='js_syntax::ast::op::LogOp'>LogOp</a>","impl <a class='trait' href='http://doc.rust-lang.org/core/clone/trait.Clone.html' title='core::clone::Clone'>Clone</a> for <a class='type' href='js_syntax/ast/op/type.BinOp.html' title='js_syntax::ast::op::BinOp'>BinOp</a>","impl <a class='trait' href='http://doc.rust-lang.org/core/clone/trait.Clone.html' title='core::clone::Clone'>Clone</a> for <a class='type' href='js_syntax/ast/keyword/type.Keyword.html' title='js_syntax::ast::keyword::Keyword'>Keyword</a>","impl <a class='trait' href='http://doc.rust-lang.org/core/clone/trait.Clone.html' title='core::clone::Clone'>Clone</a> for <a class='type' href='js_syntax/ast/punc/type.Punctuator.html' title='js_syntax::ast::punc::Punctuator'>Punctuator</a>","impl <a class='trait' href='http://doc.rust-lang.org/core/clone/trait.Clone.html' title='core::clone::Clone'>Clone</a> for <a class='struct' href='js_syntax/ast/pos/struct.Position.html' title='js_syntax::ast::pos::Position'>Position</a>","impl <a class='trait' href='http://doc.rust-lang.org/core/clone/trait.Clone.html' title='core::clone::Clone'>Clone</a> for <a class='struct' href='js_syntax/ast/token/struct.Token.html' title='js_syntax::ast::token::Token'>Token</a>","impl <a class='trait' href='http://doc.rust-lang.org/core/clone/trait.Clone.html' title='core::clone::Clone'>Clone</a> for <a class='type' href='js_syntax/ast/token/type.TokenData.html' title='js_syntax::ast::token::TokenData'>TokenData</a>","impl <a class='trait' href='http://doc.rust-lang.org/core/clone/trait.Clone.html' title='core::clone::Clone'>Clone</a> for <a class='type' href='js_syntax/ast/types/type.Type.html' title='js_syntax::ast::types::Type'>Type</a>","impl <a class='trait' href='http://doc.rust-lang.org/core/clone/trait.Clone.html' title='core::clone::Clone'>Clone</a> for <a class='type' href='js_syntax/parser/type.ParseError.html' title='js_syntax::parser::ParseError'>ParseError</a>",];
implementors['front'] = ["impl <a class='trait' href='http://doc.rust-lang.org/core/clone/trait.Clone.html' title='core::clone::Clone'>Clone</a> for <a class='struct' href='front/stdlib/value/struct.Value.html' title='front::stdlib::value::Value'>Value</a>","impl <a class='trait' href='http://doc.rust-lang.org/core/clone/trait.Clone.html' title='core::clone::Clone'>Clone</a> for <a class='type' href='front/stdlib/value/type.ValueData.html' title='front::stdlib::value::ValueData'>ValueData</a>","impl <a class='trait' href='http://doc.rust-lang.org/core/clone/trait.Clone.html' title='core::clone::Clone'>Clone</a> for <a class='struct' href='front/stdlib/function/struct.Function.html' title='front::stdlib::function::Function'>Function</a>","impl <a class='trait' href='http://doc.rust-lang.org/core/clone/trait.Clone.html' title='core::clone::Clone'>Clone</a> for <a class='struct' href='front/stdlib/object/struct.Property.html' title='front::stdlib::object::Property'>Property</a>",];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        
})()
