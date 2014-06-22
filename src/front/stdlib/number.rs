use stdlib::value::{Value, ResultValue, VNumber, VInteger, to_value, from_value};
use stdlib::function::Function;
use std::f64::{NAN, MAX_VALUE, MIN_VALUE, INFINITY, NEG_INFINITY, EPSILON};
/// Parse a float into a value
pub fn parse_float(args:Vec<Value>, _:Value, _:Value, _:Value) -> ResultValue {
    let parsed = from_str::<f64>(from_value::<String>(*args.get(0)).unwrap().as_slice());
    return Ok(to_value(match parsed {
        Some(v) => v,
        None => NAN
    }));
}
/// Parse an int into a value
pub fn parse_int(args:Vec<Value>, _:Value, _:Value, _:Value) -> ResultValue {
    let parsed = from_str::<i32>(from_value::<String>(*args.get(0)).unwrap().as_slice());
    return Ok(match parsed {
        Some(v) => to_value(v),
        None => to_value(NAN)
    });
}
/// Check if a value when converted to a number is finite
pub fn is_finite(args:Vec<Value>, _:Value, _:Value, _:Value) -> ResultValue {
    Ok(to_value(if args.len() == 0 {
        false
    } else {
        from_value::<f64>(*args.get(0)).unwrap().is_finite()
    }))
}
/// Check if a number is finite
pub fn strict_is_finite(args:Vec<Value>, _:Value, _:Value, _:Value) -> ResultValue {
    Ok(to_value(if args.len() == 0 {
        false
    } else {
        let num = args.get(0);
        match **num {
            VNumber(v) => v.is_finite(),
            VInteger(_) => true, // integers can't be infinite
            _ => false
        }
    }))
}
/// Check if a value when converted to a number is equal to NaN
pub fn is_nan(args:Vec<Value>, _:Value, _:Value, _:Value) -> ResultValue {
    Ok(to_value(if args.len() == 0 {
        false
    } else {
        from_value::<f64>(*args.get(0)).unwrap().is_nan()
    }))
}
/// Check if a number is equal to NaN
pub fn strict_is_nan(args:Vec<Value>, _:Value, _:Value, _:Value) -> ResultValue {
    Ok(to_value(if args.len() == 0 {
        false
    } else {
        let num = args.get(0);
        match **num {
            VNumber(v) => v.is_nan(),
            _ => false
        }
    }))
}
/// Create a new `Number` object
pub fn _create(global:Value) -> Value {
    let number = Value::new_obj(Some(global));
    number.set_field("NaN", to_value(NAN));
    number.set_field("MAX_VALUE", to_value(MAX_VALUE));
    number.set_field("MIN_VALUE", to_value(MIN_VALUE));
    number.set_field("POSITIVE_INFINITY", to_value(INFINITY));
    number.set_field("NEGATIVE_INFINITY", to_value(NEG_INFINITY));
    number.set_field("EPSILON", to_value(EPSILON));
    number.set_field("parseFloat", Function::make(parse_float, ["string"]));
    number.set_field("parseInt", Function::make(parse_int, ["string"]));
    number.set_field("isFinite", Function::make(strict_is_finite, ["num"]));
    number.set_field("isNaN", Function::make(strict_is_nan, ["num"]));
    number
}
/// Initialise the parse functions and `Number` on the global object
pub fn init(global:Value) {
    global.set_field("NaN", to_value(NAN));
    global.set_field("Infinity", to_value(INFINITY));
    global.set_field("parseFloat", Function::make(parse_float, ["string"]));
    global.set_field("parseInt", Function::make(parse_int, ["string"]));
    global.set_field("isFinite", Function::make(is_finite, ["number"]));
    global.set_field("isNaN", Function::make(is_nan, ["num"]));
    global.set_field("Number", _create(global));
}