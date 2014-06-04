use ast::expr::*;
use ast::types::*;
use ast::constant::*;
use ast::op::*;
use std::iter::FromIterator;

/// Type an expression
pub fn resolve_type(expr:&Expr) -> Box<Type> {
	match expr.def {
		ConstExpr(CInt(_)) =>
			box IntegerType,
		ConstExpr(CNum(_)) =>
			box NumberType,
		ConstExpr(CRegExp(_, _, _)) =>
			box NativeObjectType,
		ConstExpr(CBool(_)) =>
			box BooleanType,
		ConstExpr(CNull) =>
			box NullType,
		ConstExpr(CUndefined) =>
			box UndefinedType,
		ConstExpr(CString(_)) =>
			box StringType,
		ObjectDeclExpr(_) =>
			box ObjectType,
		ArrayDeclExpr(_) =>
			box ObjectType,
		BinOpExpr(BinNum(OpAdd), ref a, ref b) => {
			box match (*resolve_type(*a), *resolve_type(*b)) {
				(StringType, _) | (_, StringType) =>
					StringType,
				(IntegerType, IntegerType) =>
					IntegerType,
				_ => NumberType
			}
		},
		BinOpExpr(BinNum(_), _, _) =>
			box NumberType,
		BinOpExpr(BinBit(_), _, _) =>
			box IntegerType,
		BinOpExpr(BinComp(_), _, _) =>
			box BooleanType,
		BinOpExpr(BinLog(_), _, _) =>
			box BooleanType,
		UnaryOpExpr(UnaryNot, _) =>
			box BooleanType,
		UnaryOpExpr(UnaryPlus, ref inner) | UnaryOpExpr(UnaryMinus, ref inner) =>
			resolve_type(*inner),
		UnaryOpExpr(_, _) =>
			box NumberType,
		BlockExpr(ref exprs) =>
			resolve_type(exprs.get(exprs.len() - 1)),
		LocalExpr(_) =>
			box AnyType,
		GetConstFieldExpr(_, _) =>
			box AnyType,
		GetFieldExpr(_, _) =>
			box AnyType,
		CallExpr(_, _) =>
			box AnyType,
		WhileLoopExpr(_, _) =>
			box UndefinedType,
		IfExpr(_, ref if_expr, Some(ref else_expr)) => {
			let if_type = resolve_type(*if_expr);
			let else_type = resolve_type(*else_expr);
			if if_type == else_type {
				if_type
			} else {
				box AnyOfType(vec!(*if_type, *else_type))
			}
		},
		IfExpr(_, ref if_expr, None) => {
			let if_type = *resolve_type(*if_expr);
			let else_type = UndefinedType;
			box if if_type == else_type {
				if_type
			} else {
				AnyOfType(vec!(if_type, else_type))
			}
		},
		SwitchExpr(_, ref matches, None) => {
			box AnyOfType(FromIterator::from_iter(matches.iter().map(|&(_, ref block)| *resolve_type(block.get(block.len() - 1)))))
		},
		SwitchExpr(_, ref matches, Some(ref def)) => {
			let mut types : Vec<Type> = FromIterator::from_iter(matches.iter().map(|&(_, ref block)| *resolve_type(block.get(block.len() - 1))));
			types.push(*resolve_type(*def));
			box AnyOfType(types)
		},
		FunctionDeclExpr(_, _, _) | ArrowFunctionDeclExpr(_, _) =>
			box FunctionType,
		ConstructExpr(_, _) =>
			box ObjectType,
		ReturnExpr(_) =>
			box UndefinedType,
		ThrowExpr(_) =>
			box UndefinedType,
		AssignExpr(_, ref what) =>
			resolve_type(*what),
		VarDeclExpr(_) =>
			box UndefinedType,
		TypeOfExpr(_) =>
			box StringType
	}
}