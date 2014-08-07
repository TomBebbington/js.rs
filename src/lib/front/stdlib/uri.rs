use front::stdlib::value::{Value, ResultValue, to_value};
use front::stdlib::function::Function;
use url::{DEFAULT_ENCODE_SET, utf8_percent_encode, lossy_utf8_percent_decode};

/// Encode a URI
pub fn encode_uri(args:Vec<Value>, _:Value, _:Value, _:Value) -> ResultValue {
    let arg = args[0];
    Ok(to_value(utf8_percent_encode(arg.to_string().as_slice(), DEFAULT_ENCODE_SET)))
}
/// Encode a URI component
pub fn encode_uri_component(args:Vec<Value>, _:Value, _:Value, _:Value) -> ResultValue {
    let arg = args[0];
    Ok(to_value(utf8_percent_encode(arg.to_string().as_slice(), DEFAULT_ENCODE_SET)))
}
/// Decode a URI
pub fn decode_uri(args:Vec<Value>, _:Value, _:Value, _:Value) -> ResultValue {
    let arg = args[0];
    Ok(to_value(lossy_utf8_percent_decode(arg.to_string().as_bytes())))
}
/// Decode a URI component
pub fn decode_uri_component(args:Vec<Value>, _:Value, _:Value, _:Value) -> ResultValue {
    let arg = args[0];
    Ok(to_value(lossy_utf8_percent_decode(arg.to_string().as_bytes())))
}
/// Initialise the URI functions on the global object
pub fn init(global:Value) {
    js_extend!(global, {
        "encodeURI": Function::make(encode_uri, ["uri"]),
        "encodeURIComponent": Function::make(encode_uri_component, ["uri_comp"]),
        "decodeURI": Function::make(decode_uri, ["uri"]),
        "decodeURIComponent": Function::make(decode_uri_component, ["uri_comp"])
    });
}