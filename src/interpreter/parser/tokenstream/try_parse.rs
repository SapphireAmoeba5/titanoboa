use crate::interpreter::types::Type;

pub fn try_parse_number(token: &str) -> Option<Type> {
    if let Some(uinteger) = try_parse_hex(token) {
        return Some(uinteger);
    } else if let Some(uinteger) = try_parse_binary(token) {
        return Some(uinteger);
    } else if let Some(int) = try_parse_integer(token) {
        return Some(int);
    } else if let Some(float) = try_parse_float(token) {
        return Some(float);
    }
    None
}

fn try_parse_hex(token: &str) -> Option<Type> {
    if token.starts_with("0x") || token.starts_with("0X") {
        if let Ok(uinteger) = u64::from_str_radix(&token[2..], 16) {
            return Some(Type::Uint(uinteger));
        }
    }
    None
}

fn try_parse_binary(token: &str) -> Option<Type> {
    if token.starts_with("0b") || token.starts_with("0B") {
        if let Ok(uinteger) = u64::from_str_radix(&token[2..], 2) {
            return Some(Type::Uint(uinteger));
        }
    }
    None
}

fn try_parse_integer(token: &str) -> Option<Type> {
    if token.ends_with("u") {
        match token[..token.len() - 1].parse::<u64>() {
            Ok(uint) => return Some(Type::Uint(uint)),
            Err(_) => {}
        }
    } else {
        match token.parse::<i64>() {
            Ok(int) => return Some(Type::Int(int)),
            Err(_) => return None,
        }
    }
    None
}

fn try_parse_float(token: &str) -> Option<Type> {
    if token.ends_with("f") {
        match token[..token.len() - 1].parse::<f64>() {
            Ok(float) => return Some(Type::Float(float)),
            Err(_) => {}
        }
    } else {
        match token.parse::<f64>() {
            Ok(float) => return Some(Type::Float(float)),
            Err(_) => return None,
        }
    }
    None
}
