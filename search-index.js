var searchIndex = {};
searchIndex['js_syntax'] = {"items":[[0,"","js_syntax","This crate provides a Javascript parsing library with a parser,\na lexer, and Abstract Syntax Tree. The lexer started off based\noff the Kaleidocope OCaml tutorial, then it evolved off there.\n \nThe parser is based on my work on the [`hscript` project from \nHaxe](https://github.com/TomBebbington/hscript/blob/master/hscript/Parser.hx), but it's still\nvery premature and some important things are missing for now."],[0,"ast","","The Abstract Syntax Trees for Javascript tokens and expressions"],[0,"constant","js_syntax::ast","Constant AST"],[2,"Const","js_syntax::ast::constant","A Javascript constant"],[12,"CString","","A UTF-8 string, such as `\"Hello, world\"`",0],[12,"CRegExp","","A regular expression, such as `/where('s| is) [wW]ally/`",0],[12,"CNum","","A 64-bit floating-point number, such as `3.1415`",0],[12,"CInt","","A 32-bit integer, such as `42`",0],[12,"CBool","","A boolean, which is either `true` or `false` and is used to check if criteria are met",0],[12,"CNull","","The `null` value, which represents a non-existant value",0],[12,"CUndefined","","The `undefined` value, which represents a field or index that doesn't exist",0],[10,"eq","","",0],[10,"ne","","",0],[10,"clone","","",0],[10,"fmt","","",0],[0,"expr","js_syntax::ast","Expression AST"],[1,"Expr","js_syntax::ast::expr","A Javascript expression, including its position"],[11,"def","","The expression definition",1],[11,"start","","The starting position",1],[11,"end","","The ending position",1],[2,"ExprDef","","A Javascript expression"],[12,"BinOpExpr","","Run a operation between 2 expressions",2],[12,"UnaryOpExpr","","Run an operation on a value",2],[12,"ConstExpr","","Make a constant value",2],[12,"BlockExpr","","Run several expressions from top-to-bottom",2],[12,"LocalExpr","","Load a reference to a value",2],[12,"GetConstFieldExpr","","Gets the constant field of a value",2],[12,"GetFieldExpr","","Gets the field of a value",2],[12,"CallExpr","","Call a function with some values",2],[12,"WhileLoopExpr","","Repeatedly run an expression while the conditional expression resolves to true",2],[12,"IfExpr","","Check if a conditional expression is true and run an expression if it is and another expression if it isn't",2],[12,"SwitchExpr","","Run blocks whose cases match the expression",2],[12,"ObjectDeclExpr","","Create an object out of the binary tree given",2],[12,"ArrayDeclExpr","","Create an array with items inside",2],[12,"FunctionDeclExpr","","Create a function with the given name, arguments, and expression",2],[12,"ArrowFunctionDeclExpr","","Create an arrow function with the given arguments and expression",2],[12,"ConstructExpr","","Construct an object from the function and arguments given",2],[12,"ReturnExpr","","Return the expression from a function",2],[12,"ThrowExpr","","Throw a value",2],[12,"AssignExpr","","Assign an expression to a value",2],[12,"VarDeclExpr","","A variable declaration",2],[12,"TypeOfExpr","","Return a string representing the type of the given expression",2],[10,"eq","","",1],[10,"ne","","",1],[10,"clone","","",1],[10,"new","","Create a new expression with a starting and ending position",1],[10,"fmt","","",1],[10,"eq","","",2],[10,"ne","","",2],[10,"clone","","",2],[10,"get_precedence","","",2],[10,"fmt","","",2],[0,"op","js_syntax::ast","Operation AST"],[2,"NumOp","js_syntax::ast::op","A numeric operation between 2 values"],[12,"OpAdd","","`a + b` - Addition",3],[12,"OpSub","","`a - b` - Subtraction",3],[12,"OpDiv","","`a / b` - Division",3],[12,"OpMul","","`a * b` - Multiplication",3],[12,"OpMod","","`a % b` - Modulus",3],[2,"UnaryOp","","A unary operation on a single value"],[12,"UnaryIncrement","","`a++` - increment the value",4],[12,"UnaryDecrement","","`a--` - decrement the value",4],[12,"UnaryMinus","","`-a` - negate the value",4],[12,"UnaryPlus","","`+a` - convert to a number",4],[12,"UnaryNot","","`!a` - get the opposite of the boolean value",4],[2,"BitOp","","A bitwise operation between 2 values"],[12,"BitAnd","","`a & b` - Bitwise and",5],[12,"BitOr","","`a | b` - Bitwise or",5],[12,"BitXor","","`a ^ b` - Bitwise xor",5],[12,"BitShl","","`a << b` - Bit-shift leftwards",5],[12,"BitShr","","`a >> b` - Bit-shift rightrights",5],[2,"CompOp","","A comparitive operation between 2 values"],[12,"CompEqual","","`a == b` - Equality",6],[12,"CompNotEqual","","`a != b` - Unequality",6],[12,"CompStrictEqual","","`a === b` - Strict equality",6],[12,"CompStrictNotEqual","","`a !== b` - Strict unequality",6],[12,"CompGreaterThan","","`a > b` - If `a` is greater than `b`",6],[12,"CompGreaterThanOrEqual","","`a >= b` - If `a` is greater than or equal to `b`",6],[12,"CompLessThan","","`a < b` - If `a` is less than `b`",6],[12,"CompLessThanOrEqual","","`a <= b` - If `a` is less than or equal to `b`",6],[2,"LogOp","","A logical operation between 2 boolean values"],[12,"LogAnd","","`a && b` - Logical and",7],[12,"LogOr","","`a || b` - Logical or",7],[2,"BinOp","","A binary operation between 2 values"],[12,"BinNum","","Numeric operation",8],[12,"BinBit","","Bitwise operation",8],[12,"BinComp","","Comparitive operation",8],[12,"BinLog","","Logical operation",8],[6,"Operator","","Represents an operator"],[9,"get_precedence","","Get the precedence as an unsignes integer, where the lower it is, the more precedence/priority it has",9],[10,"eq","","",3],[10,"ne","","",3],[10,"clone","","",3],[10,"fmt","","",3],[10,"eq","","",4],[10,"ne","","",4],[10,"clone","","",4],[10,"fmt","","",4],[10,"eq","","",5],[10,"ne","","",5],[10,"clone","","",5],[10,"fmt","","",5],[10,"eq","","",6],[10,"ne","","",6],[10,"clone","","",6],[10,"fmt","","",6],[10,"eq","","",7],[10,"ne","","",7],[10,"clone","","",7],[10,"fmt","","",7],[10,"eq","","",8],[10,"ne","","",8],[10,"clone","","",8],[10,"get_precedence","","",8],[10,"fmt","","",8],[0,"pos","js_syntax::ast","Position AST"],[1,"Position","js_syntax::ast::pos","A position in Javascript source code"],[11,"column_number","","The column number",10],[11,"line_number","","The line number",10],[10,"eq","","",10],[10,"ne","","",10],[10,"clone","","",10],[10,"new","","Create a new position",10],[0,"token","js_syntax::ast","Token AST"],[1,"Token","js_syntax::ast::token","A single of token of Javascript code including its position"],[11,"data","","The token",11],[11,"pos","","The token's position",11],[2,"TokenData","","A single token of Javacript code - a single word, symbol or constant"],[12,"TString","","A string literal",12],[12,"TSemicolon","","A semicolon (;)",12],[12,"TColon","","A colon",12],[12,"TDot","","A dot / full stop",12],[12,"TEqual","","An equal sign",12],[12,"TComma","","A comma",12],[12,"TIdent","","An identity literal",12],[12,"TOpenParen","","An opening bracket",12],[12,"TCloseParen","","A closing bracket",12],[12,"TOpenBlock","","An opening curly bracket",12],[12,"TCloseBlock","","An closing curly bracket",12],[12,"TOpenArray","","An opening square bracket",12],[12,"TCloseArray","","A closing square bracket",12],[12,"TNumber","","A 64-bit floating-point number",12],[12,"TQuestion","","A question mark",12],[12,"TArrow","","An arrow",12],[12,"TBinOp","","An operation between 2 values",12],[12,"TUnaryOp","","A unary operation",12],[12,"TAssignOp","","An assign operation combined with something else",12],[12,"TComment","","A comment",12],[10,"clone","","",11],[10,"eq","","",11],[10,"ne","","",11],[10,"new","","Create a new detailed token from the token data, line number and column number",11],[10,"fmt","","",11],[10,"clone","","",12],[10,"eq","","",12],[10,"ne","","",12],[10,"fmt","","",12],[0,"lexer","js_syntax","The lexer, which transforms a string stream into a sequence of tokens"],[1,"Lexer","js_syntax::lexer","A Javascript lexer"],[11,"tokens","","The list of tokens generated so far",13],[11,"line_number","","The current line number in the script",13],[11,"column_number","","The current column number in the script",13],[11,"buffer","","The reader",13],[11,"peek_buffer","","The peeked character buffer",13],[2,"StringType","","The type of string used"],[12,"DoubleQuote","","`\"` - Double-quoted",14],[12,"SingleQuote","","`'` - Single-quoted",14],[2,"CommentType","","The type of comment used"],[12,"MultiLineComment","","`/*...*/` - A multi-line comment",15],[12,"SingleLineComment","","`//...` - A single-line comment",15],[2,"NumberType","","The type of number used"],[12,"DecimalNumber","","A decimal number, such as `3.1415`",16],[12,"HexadecimalNumber","","A hexadecimal number, such as `0xFF00FF`",16],[12,"OctalNumber","","An octal number, such as `011`",16],[10,"eq","","",14],[10,"ne","","",14],[10,"clone","","",14],[10,"eq","","",15],[10,"ne","","",15],[10,"clone","","",15],[10,"eq","","",16],[10,"ne","","",16],[10,"clone","","",16],[10,"new","","Creates a new lexer with empty buffers",13],[10,"lex_str","","Processes an input stream from a string into an array of tokens",13],[10,"lex","","Processes an input stream from the `buffer` into a vector of tokens",13],[0,"parser","js_syntax","The parser, which transforms a sequence of tokens into expressions"],[1,"Parser","js_syntax::parser","A Javascript parser"],[11,"tokens","","The tokens being input",17],[11,"pos","","The current position within the tokens",17],[2,"ParseError","","An error encountered during parsing an expression"],[12,"Expected","","When it expected a certain kind of token, but got another as part of something",18],[12,"ExpectedExpr","","When it expected a certain expression, but got another",18],[12,"AbruptEnd","","When there is an abrupt end to the parsing",18],[4,"ParseResult","",""],[4,"ParseStructResult","",""],[10,"clone","","",18],[10,"eq","","",18],[10,"ne","","",18],[10,"fmt","","",18],[10,"new","","Creates a new parser, using `tokens` as input",17],[10,"parse_all","","Parse all expressions in the token array",17],[10,"parse","","Parse a single expression",17]],"paths":[[2,"Const"],[1,"Expr"],[2,"ExprDef"],[2,"NumOp"],[2,"UnaryOp"],[2,"BitOp"],[2,"CompOp"],[2,"LogOp"],[2,"BinOp"],[6,"Operator"],[1,"Position"],[1,"Token"],[2,"TokenData"],[1,"Lexer"],[2,"StringType"],[2,"CommentType"],[2,"NumberType"],[1,"Parser"],[2,"ParseError"]]};
searchIndex['js'] = {"items":[[0,"","js","This crate provides a Javascript execution library with an\nJITCompiler and a Javascript standard library."],[0,"exec","","The interpreter"],[1,"Scope","js::exec","A variable scope"],[11,"this","","The value of `this` in the scope",0],[11,"vars","","The variables declared in the scope",0],[1,"JITCompiler","","A Javascript JIT compiler"],[11,"context","","The JIT Context",1],[11,"global","","An object representing the global object",1],[11,"scopes","","The scopes",1],[6,"Executor","","An execution engine"],[9,"new","","Make a new execution engine",2],[9,"set_global","","Set a global variable called `name` with the value `val`",2],[9,"get_global","","Resolve the global variable `name`",2],[9,"make_scope","","Create a new scope and return it",2],[9,"destroy_scope","","Destroy the current scope",2],[9,"compile","","Compile the expression",2],[9,"run","","Run an expression",2],[10,"scope","","Get the current scope",1],[10,"new","","",1],[10,"set_global","","",1],[10,"get_global","","",1],[10,"make_scope","","",1],[10,"destroy_scope","","",1],[10,"compile","","",1],[10,"run","","",1],[0,"stdlib","js","The standard Javascript library"],[0,"value","js::stdlib","Javascript values, utility methods and conversion between Javascript values and Rust values"],[1,"Value","js::stdlib::value","A Garbage-collected Javascript value as represented in the interpreter"],[11,"ptr","","The garbage-collected pointer",3],[2,"ValueData","","A Javascript value"],[12,"VNull","","`null` - A null value, for when a value doesn't exist",4],[12,"VUndefined","","`undefined` - An undefined value, for when a field or index doesn't exist",4],[12,"VBoolean","","`boolean` - A `true` / `false` value, for if a certain criteria is met",4],[12,"VString","","`String` - A UTF-8 string, such as `\"Hello, world\"`",4],[12,"VNumber","","`Number` - A 64-bit floating point number, such as `3.1415`",4],[12,"VInteger","","`Number` - A 32-bit integer, such as `42`",4],[12,"VObject","","`Object` - An object, such as `Math`, represented by a binary tree of string keys to Javascript values",4],[12,"VFunction","","`Function` - A runnable block of code, such as `Math.sqrt`, which can take some variables and return a useful value or act upon an object",4],[3,"from_value","","A utility function that just calls FromValue::from_value"],[3,"to_value","","A utility function that just calls ToValue::to_value"],[4,"ResultValue","","The result of a Javascript expression is represented like this so it can succeed (`Ok`) or fail (`Err`)"],[6,"ToValue","","Conversion to Javascript values from Rust values"],[9,"to_value","","Convert this value to a Rust value",5],[6,"FromValue","","Conversion to Rust values from Javascript values"],[9,"from_value","","Convert this value to a Javascript value",6],[10,"clone","","",3],[10,"clone","","",4],[10,"new_obj","","Returns a new empty object",3],[10,"is_object","","Returns true if the value is an object",3],[10,"is_undefined","","Returns true if the value is undefined",3],[10,"is_null","","Returns true if the value is null",3],[10,"is_null_or_undefined","","Returns true if the value is null or undefined",3],[10,"is_double","","Returns true if the value is a 64-bit floating-point number",3],[10,"is_string","","Returns true if the value is a string",3],[10,"is_true","","Returns true if the value is true",3],[10,"to_num","","Converts the value into a 64-bit floating point number",3],[10,"to_int","","Converts the value into a 32-bit integer",3],[10,"get_prop","","Resolve the property in the object",3],[10,"get_field","","Resolve the property in the object and get its value, or undefined if this is not an object or the field doesn't exist",3],[10,"get_field_slice","","Resolve the property in the object and get its value, or undefined if this is not an object or the field doesn't exist",3],[10,"set_field","","Set the field in the value",3],[10,"set_field_slice","","Set the field in the value",3],[10,"set_prop","","Set the property in the value",3],[10,"set_prop_slice","","Set the property in the value",3],[10,"from_json","","Convert from a JSON value to a JS value",3],[10,"get_type","","Get the type of the value",3],[10,"undefined","","Get the value for undefined",3],[10,"fmt","","",3],[10,"eq","","",3],[10,"to_json","","",3],[10,"add","","",3],[10,"sub","","",3],[10,"mul","","",3],[10,"div","","",3],[10,"rem","","",3],[10,"bitand","","",3],[10,"bitor","","",3],[10,"bitxor","","",3],[10,"shl","","",3],[10,"shr","","",3],[10,"not","","",3],[10,"neg","","",3],[10,"lt","","",3],[10,"le","","",3],[10,"gt","","",3],[10,"ge","","",3],[10,"to_value","std::string","",7],[10,"from_value","","",7],[10,"to_value","std::vec","",8],[10,"from_value","","",8],[10,"to_value","serialize::json","",9],[10,"from_value","","",9],[10,"to_value","core::option","",10],[10,"from_value","","",10],[0,"function","js::stdlib","The global `Function` object and function value representations"],[1,"Function","js::stdlib::function","A Javascript function"],[11,"object","","The fields associated with the function",11],[11,"repr","","This function's JIT representation",11],[11,"args","","The argument names of the function",11],[3,"_create","","Create a new `Function` object"],[3,"init","","Initialise the global object with the `Function` object"],[4,"FunctionData","",""],[10,"clone","","",11],[10,"new","","Make a new function",11],[10,"make","","Create a function from function data and arguments",11],[10,"call","","Call with some args",11],[0,"object","js::stdlib","The global `Object` object"],[1,"Property","js::stdlib::object","A Javascript property"],[11,"configurable","","If the type of this can be changed and this can be deleted",12],[11,"enumerable","","If the property shows up in enumeration of the object",12],[11,"writable","","If this property can be changed with an assignment",12],[11,"value","","The value associated with the property",12],[11,"get","","The function serving as getter",12],[11,"set","","The function serving as setter",12],[3,"make_object","","Create a new object"],[3,"get_proto_of","","Get the prototype of an object"],[3,"set_proto_of","","Set the prototype of an object"],[3,"define_prop","","Define a property in an object"],[3,"to_string","","To string"],[3,"has_own_prop","","Check if it has a property"],[3,"_create","","Create a new `Object` object"],[3,"init","","Initialise the `Object` object on the global object"],[4,"ObjectData","",""],[5,"PROTOTYPE","",""],[5,"INSTANCE_PROTOTYPE","",""],[10,"clone","","",12],[10,"new","","Make a new property with the given value",12],[10,"to_value","","",12],[10,"from_value","","",12],[0,"array","js::stdlib","The global `Array` object"],[3,"make_array","js::stdlib::array","Create a new array"],[3,"_create","","Create a new `Array` object"],[3,"init","","Initialise the global object with the `Array` object"],[0,"console","js::stdlib","The global `console` object"],[3,"log","js::stdlib::console","Print a javascript value to the standard output stream"],[3,"error","","Print a javascript value to the standard error stream"],[3,"_create","","Create a new `console` object"],[3,"init","","Initialise the global object with the `console` object"],[0,"math","js::stdlib","The global `Math` object"],[3,"abs","js::stdlib::math","Get the absolute value of a number"],[3,"acos","","Get the arccos of a number"],[3,"asin","","Get the arcsine of a number"],[3,"atan","","Get the arctangent of a number"],[3,"atan2","","Get the arctangent of a numbers"],[3,"cbrt","","Get the cubic root of a number"],[3,"ceil","","Get lowest integer above a number"],[3,"cos","","Get the cosine of a number"],[3,"exp","","Get the power to raise the natural logarithm to get the number"],[3,"floor","","Get the highest integer below a number"],[3,"log","","Get the natural logarithm of a number"],[3,"max","","Get the maximum of several numbers"],[3,"min","","Get the minimum of several numbers"],[3,"pow","","Raise a number to a power"],[3,"_random","","Generate a random floating-point number between 0 and 1"],[3,"round","","Round a number to the nearest integer"],[3,"sin","","Get the sine of a number"],[3,"sqrt","","Get the square root of a number"],[3,"tan","","Get the tangent of a number"],[3,"_create","","Create a new `Math` object"],[3,"init","","Initialise the `Math` object on the global object"],[0,"json","js::stdlib","The global `JSON` object"],[3,"parse","js::stdlib::json","Parse a JSON string into a Javascript object"],[3,"stringify","","Process a Javascript object into a JSON string"],[3,"_create","","Create a new `JSON` object"],[3,"init","","Initialise the global object with the `JSON` object"],[0,"number","js::stdlib","The global `Number` object with related functions and constants"],[3,"parse_float","js::stdlib::number","Parse a float into a value"],[3,"parse_int","","Parse an int into a value"],[3,"is_finite","","Check if a value when converted to a number is finite"],[3,"strict_is_finite","","Check if a number is finite"],[3,"is_nan","","Check if a value when converted to a number is equal to NaN"],[3,"strict_is_nan","","Check if a number is equal to NaN"],[3,"_create","","Create a new `Number` object"],[3,"init","","Initialise the parse functions and `Number` on the global object"],[0,"error","js::stdlib","The global `Error` object"],[3,"make_error","js::stdlib::error","Create a new error"],[3,"to_string","","Get the string representation of the error"],[3,"_create","","Create a new `Error` object"],[3,"init","","Initialise the global object with the `Error` object"],[0,"uri","js::stdlib","Contains global methods concerning URIs"],[3,"encode_uri","js::stdlib::uri","Encode a URI"],[3,"encode_uri_component","","Encode a URI component\nRust uses RFC 3986, but standard Javascript doesn't, this will need a fix"],[3,"decode_uri","","Decode a URI"],[3,"decode_uri_component","","Decode a URI component\nRust uses RFC 3986, but standard Javascript doesn't, this will need a fix"],[3,"init","","Initialise the URI functions on the global object"],[0,"string","js::stdlib","The global `String` object"],[3,"make_string","js::stdlib::string","Create new string"],[3,"get_string_length","","Get a string's length"],[3,"_create","","Create a new `String` object"],[3,"init","","Initialise the `String` object on the global object"],[10,"to_value","js::stdlib::object","",13],[10,"from_value","","",13]],"paths":[[1,"Scope"],[1,"JITCompiler"],[6,"Executor"],[1,"Value"],[2,"ValueData"],[6,"ToValue"],[6,"FromValue"],[1,"String"],[1,"Vec"],[2,"Json"],[2,"Option"],[1,"Function"],[1,"Property"],[4,"ObjectData"]]};

searchIndex['jit'] = {"items":[[0,"","jit","This crate wraps LibJIT"],[1,"Context","","Holds all of the functions you have built and compiled. There can be multuple, but normally there is only one."],[11,"_context","","",0],[1,"TypeKind","",""],[11,"bits","","",1],[1,"Type","","A Type of a value to JIT"],[11,"_type","","",2],[1,"Function","","A Function to JIT"],[11,"_context","","",3],[11,"_function","","",3],[1,"Value","","A Value that is being JITed"],[11,"_value","","",4],[1,"Label","","A label in the code that can be branched to in instructions"],[11,"_label","","",5],[1,"Types","","Holds type constructors"],[2,"ABI","","A platform's application binary interface"],[12,"CDECL","","The C application binary interface",6],[2,"CallFlags","","Call flags to a function"],[12,"JitCallNothrow","","When the function won't throw a value",7],[12,"JitCallNoReturn","","When the function won't return a value",7],[12,"JitCallTail","","When the function is tail-recursive",7],[5,"Invalid","",""],[5,"Void","",""],[5,"SByte","",""],[5,"UByte","",""],[5,"Short","",""],[5,"UShort","",""],[5,"Int","",""],[5,"UInt","",""],[5,"NInt","",""],[5,"NUInt","",""],[5,"Long","",""],[5,"ULong","",""],[5,"Float32","",""],[5,"Float64","",""],[5,"NFloat","",""],[5,"MaxPrimitive","",""],[5,"Struct","",""],[5,"Union","",""],[5,"Signature","",""],[5,"Pointer","",""],[5,"FirstTagged","",""],[6,"Compilable","","A type that can be compiled into a LibJIT representation"],[9,"compile","","Get a JIT representation of this value",8],[10,"new","","Create a new JIT Context",0],[10,"build_start","","Lock down the context to prevent multiple threads from using the builder at a time",0],[10,"build_end","","Unlock the context from this thread",0],[10,"create_function","","Create a function in the context with the type signature given",0],[10,"drop","","",0],[10,"clone","","",1],[10,"eq","","",1],[10,"ne","","",1],[10,"empty","","Returns an empty set of flags.",1],[10,"all","","Returns the set containing all flags.",1],[10,"bits","","Returns the raw value of the flags currently stored.",1],[10,"from_bits","","Convert from underlying bit representation, unless that\nrepresentation contains bits that do not correspond to a flag.",1],[10,"from_bits_truncate","","Convert from underlying bit representation, dropping any bits\nthat do not correspond to flags.",1],[10,"is_empty","","Returns `true` if no flags are currently stored.",1],[10,"is_all","","Returns `true` if all flags are currently set.",1],[10,"intersects","","Returns `true` if there are flags common to both `self` and `other`.",1],[10,"contains","","Returns `true` all of the flags in `other` are contained within `self`.",1],[10,"insert","","Inserts the specified flags in-place.",1],[10,"remove","","Removes the specified flags in-place.",1],[10,"bitor","","Returns the union of the two sets of flags.",1],[10,"bitand","","Returns the intersection between the two sets of flags.",1],[10,"sub","","Returns the set difference of the two sets of flags.",1],[10,"not","","Returns the complement of this set of flags.",1],[10,"drop","","",2],[10,"create_signature","","Create a function signature, with the given ABI, return type and parameters",2],[10,"create_struct","","Create a struct type with the given field types",2],[10,"create_union","","Create a union type with the given field types",2],[10,"create_pointer","","Create a pointer type with the given pointee type",2],[10,"get_size","","Work out the size of this type",2],[10,"get_kind","","Get the kind of this type",2],[10,"clone","","",3],[10,"drop","","",3],[10,"get_context","","Get the context this function was made i",3],[10,"set_optimization_level","","Set the optimization level of the function, where the bigger the level, the more effort should be spent optimising",3],[10,"set_recompilable","","Make this funcition a candidate for recompilation",3],[10,"dump","","Dump the function onto stdout",3],[10,"compile","","Compile the function",3],[10,"get_param","","Get a parameter of the function as a JIT Value",3],[10,"insn_uses_catcher","","Notify libjit that this function has a catch block in it so it can prepare",3],[10,"insn_throw","","Throw an exception from the function with the value given",3],[10,"insn_return","","Return from the function with the value given",3],[10,"insn_default_return","","Return from the function",3],[10,"insn_mul","","Make an instruction that multiplies the values",3],[10,"insn_add","","Make an instruction that adds the values",3],[10,"insn_sub","","Make an instruction that subtracts the second value from the first",3],[10,"insn_div","","Make an instruction that divides the first number by the second",3],[10,"insn_leq","","Make an instruction that checks if the first value is lower than or equal to the second",3],[10,"insn_geq","","Make an instruction that checks if the first value is greater than or equal to the second",3],[10,"insn_lt","","Make an instruction that checks if the first value is lower than the second",3],[10,"insn_gt","","Make an instruction that checks if the first value is greater than the second",3],[10,"insn_eq","","Make an instruction that checks if the values are equal",3],[10,"insn_neq","","Make an instruction that checks if the values are not equal",3],[10,"insn_and","","Make an instruction that performs a bitwise and on the two values",3],[10,"insn_or","","Make an instruction that performs a bitwise or on the two values",3],[10,"insn_xor","","Make an instruction that performs a bitwise xor on the two values",3],[10,"insn_not","","Make an instruction that performs a bitwise not on the two values",3],[10,"insn_neg","","Make an instruction that performs a bitwise negate on the value",3],[10,"insn_dup","","Make an instruction that duplicates the value given",3],[10,"insn_store","","Make an instruction that stores a value at a destination value",3],[10,"insn_store_relative","","Make an instruction that stores a value a certain offset away from a destination value",3],[10,"insn_set_label","","Make an instruction that sets a label",3],[10,"insn_branch","","Make an instruction that branches to a certain label",3],[10,"insn_branch_if","","Make an instruction that branches to a certain label if the value is true",3],[10,"insn_branch_if_not","","Make an instruction that branches to a certain label if the value is false",3],[10,"insn_jump_table","","Make an instruction that branches to a label in the table",3],[10,"insn_call_indirect","","Make an instruction that calls a function that has the signature given with some arguments",3],[10,"insn_call_native0","","Make an instruction that calls a Rust function that has the signature given with no arguments and expects a return value",3],[10,"insn_call_native1","","Make an instruction that calls a Rust function that has the signature given with a single argument and expects a return value",3],[10,"insn_call_native2","","Make an instruction that calls a Rust function that has the signature given with two arguments and expects a return value",3],[10,"insn_call_native3","","Make an instruction that calls a Rust function that has the signature given with three arguments and expects a return value",3],[10,"insn_call_native4","","Make an instruction that calls a Rust function that has the signature given with four arguments and expects a return value",3],[10,"insn_alloca","","Make an instruction that allocates some space",3],[10,"apply","","Apply a function to some arguments and set the retval to the return value",3],[10,"execute","","Execute a function and with some arguments",3],[10,"closure","","Turn this function into a closure",3],[10,"create_value","","Create a new value with the given type",3],[10,"clone","","",4],[10,"get_type","","Get the type of the value",4],[10,"eq","","",5],[10,"ne","","",5],[10,"hash","","",5],[10,"new","","Create a new label",5],[10,"get_void","","Void type",9],[10,"get_int","","Integer type",9],[10,"get_uint","","Unsigned integer type",9],[10,"get_long","","Long integer type",9],[10,"get_ulong","","Unsigned long integer type",9],[10,"get_float32","","32-bit floating point type",9],[10,"get_float64","","64-bit floating point type",9],[10,"get_float","","Default floating point type",9],[10,"get_void_ptr","","A void pointer, which can represent any kind of pointer",9],[10,"get_char","","Character type",9],[10,"get_cstring","","C String type",9],[10,"get_bool","","Boolean type",9],[10,"compile","std::string","",10]],"paths":[[1,"Context"],[1,"TypeKind"],[1,"Type"],[1,"Function"],[1,"Value"],[1,"Label"],[2,"ABI"],[2,"CallFlags"],[6,"Compilable"],[1,"Types"],[1,"String"]]};

initSearch(searchIndex);
