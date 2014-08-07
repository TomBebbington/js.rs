use syntax::ast::expr::*;
use syntax::ast::types::*;
use syntax::ast::constant::*;
use syntax::ast::op::*;

/// Type an expression
pub fn resolve_type(expr:&Expr) -> Type {
    match expr.def {
        ConstExpr(CInt(_)) =>
            IntegerType,
        ConstExpr(CNum(_)) =>
            NumberType,
        ConstExpr(CRegExp(_, _, _)) =>
            NativeObjectType,
        ConstExpr(CBool(_)) =>
            BooleanType,
        ConstExpr(CNull) =>
            NullType,
        ConstExpr(CUndefined) =>
            UndefinedType,
        ConstExpr(CString(_)) =>
            StringType,
        ObjectDeclExpr(_) =>
            ObjectType,
        ArrayDeclExpr(_) =>
            ObjectType,
        BinOpExpr(BinNum(OpAdd), box ref a, box ref b) => {
            match (resolve_type(a), resolve_type(b)) {
                (StringType, _) | (_, StringType) =>
                    StringType,
                (IntegerType, IntegerType) =>
                    IntegerType,
                _ => NumberType
            }
        },
        BinOpExpr(BinNum(_), _, _) =>
            NumberType,
        BinOpExpr(BinBit(_), _, _) =>
            IntegerType,
        BinOpExpr(BinComp(_), _, _) =>
            BooleanType,
        BinOpExpr(BinLog(_), _, _) =>
            BooleanType,
        UnaryOpExpr(UnaryNot, _) =>
            BooleanType,
        UnaryOpExpr(UnaryPlus, box ref inner) | UnaryOpExpr(UnaryMinus, box ref inner) =>
            resolve_type(inner),
        UnaryOpExpr(_, _) =>
            NumberType,
        BlockExpr(ref exprs) =>
            resolve_type(&exprs[exprs.len() - 1]),
        LocalExpr(_) =>
            AnyType,
        GetConstFieldExpr(_, _) =>
            AnyType,
        GetFieldExpr(_, _) =>
            AnyType,
        CallExpr(_, _) =>
            AnyType,
        WhileLoopExpr(_, _) =>
            UndefinedType,
        IfExpr(_, box ref if_expr, Some(box ref else_expr)) => {
            let if_type = resolve_type(if_expr);
            let else_type = resolve_type(else_expr);
            if if_type == else_type {
                if_type
            } else {
                AnyOfType(vec!(if_type, else_type))
            }
        },
        IfExpr(_, box ref if_expr, None) => {
            let if_type = resolve_type(if_expr);
            let else_type = UndefinedType;
            if if_type == else_type {
                if_type
            } else {
                AnyOfType(vec!(if_type, else_type))
            }
        },
        SwitchExpr(_, ref matches, None) => {
            AnyOfType(matches.iter().map(|&(_, ref block)| resolve_type(&block[block.len() - 1])).collect())
        },
        SwitchExpr(_, ref matches, Some(box ref def)) => {
            let mut types : Vec<Type> = matches.iter().map(|&(_, ref block)| resolve_type(&block[block.len() - 1])).collect();
            types.push(resolve_type(def));
            AnyOfType(types)
        },
        FunctionDeclExpr(_, _, _) | ArrowFunctionDeclExpr(_, _) =>
            FunctionType,
        ConstructExpr(_, _) =>
            ObjectType,
        ReturnExpr(_) =>
            UndefinedType,
        ThrowExpr(_) =>
            UndefinedType,
        AssignExpr(_, box ref what) =>
            resolve_type(what),
        VarDeclExpr(_) =>
            UndefinedType,
        TypeOfExpr(_) =>
            StringType
    }
}