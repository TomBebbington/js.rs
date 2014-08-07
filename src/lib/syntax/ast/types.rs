use std::fmt::{Formatter, Result, Show};
#[deriving(Clone, PartialEq)]
#[repr(u8)]
/// Represents a Javascript type at parsing/compilation
pub enum Type {
    /// `undefined`
    UndefinedType,
    /// `null`
    NullType,
    /// `boolean`
    BooleanType,
    /// `number`
    NumberType,
    /// `number`
    IntegerType,
    /// `string`
    StringType,
    /// A native object
    NativeObjectType,
    /// `function`
    FunctionType,
    /// `object`
    ObjectType,
    /// Any of these types
    AnyOfType(Vec<Type>),
    /// Any type
    AnyType
}
impl Type {
    /// Normalise the type
    pub fn normalise(&mut self) {
        *self = match self.clone() {
            AnyOfType(ref tys) if tys.is_empty() => UndefinedType,
            thing => thing
        }
    }
}
impl Show for Type {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            UndefinedType =>
                write!(f, "undefined"),
            NullType =>
                write!(f, "null"),
            BooleanType =>
                write!(f, "boolean"),
            NumberType | IntegerType =>
                write!(f, "number"),
            StringType =>
                write!(f, "string"),
            NativeObjectType =>
                write!(f, "native"),
            FunctionType =>
                write!(f, "function"),
            ObjectType =>
                write!(f, "object"),
            AnyOfType(ref tys) =>
                write!(f, "any of {}", tys),
            AnyType =>
                write!(f, "any"),
        }
    }
}