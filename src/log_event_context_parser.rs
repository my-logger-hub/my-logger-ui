use crate::LogEventContextApiModel;

enum SeparatorType {
    SingleChar,
    Word,
}

#[derive(Debug, Clone, Copy)]
enum ParseMode {
    AwaitingForKey,
    ParsingKey(usize),
    AwaitingForValue,
    ParsingValue(usize),
    AwaitingForSeparatorStart,
    AwaitingForSeparatorEnd,
}

pub fn parse_key_value_from_string(src: &str) -> Vec<LogEventContextApiModel> {
    let mut mode = ParseMode::AwaitingForKey;

    let mut key = None;

    let mut result = Vec::new();

    let bytes = src.as_bytes();

    for (no, b) in bytes.iter().enumerate() {
        match mode {
            ParseMode::AwaitingForKey => {
                if is_key_started(*b) {
                    mode = ParseMode::ParsingKey(no);
                }
            }
            ParseMode::ParsingKey(start_from) => {
                if is_key_finished(*b) {
                    key = Some(into_string(&bytes[start_from..no]));
                    mode = ParseMode::AwaitingForValue
                }
            }
            ParseMode::AwaitingForValue => {
                if is_value_started(*b) {
                    mode = ParseMode::ParsingValue(no);
                }
            }
            ParseMode::ParsingValue(start_from) => {
                if is_value_finished(*b) {
                    let value = into_string(&bytes[start_from..no]);
                    let key = key.take().unwrap();

                    result.push(LogEventContextApiModel { key, value });

                    if let Some(separator_type) = is_separator_started(*b) {
                        match separator_type {
                            SeparatorType::SingleChar => {
                                mode = ParseMode::AwaitingForKey;
                            }
                            SeparatorType::Word => {
                                mode = ParseMode::AwaitingForSeparatorStart;
                            }
                        }
                    } else {
                        mode = ParseMode::AwaitingForSeparatorStart
                    }
                }
            }
            ParseMode::AwaitingForSeparatorStart => {
                if let Some(separator_type) = is_separator_started(*b) {
                    match separator_type {
                        SeparatorType::SingleChar => {
                            mode = ParseMode::AwaitingForKey;
                        }
                        SeparatorType::Word => {
                            mode = ParseMode::AwaitingForSeparatorEnd;
                        }
                    }
                }
            }
            ParseMode::AwaitingForSeparatorEnd => {
                if is_separator_ended(*b) {
                    mode = ParseMode::AwaitingForKey;
                }
            }
        }
    }

    match mode {
        ParseMode::AwaitingForKey => {}
        ParseMode::ParsingKey(_) => {}
        ParseMode::AwaitingForValue => {}
        ParseMode::ParsingValue(start_from) => {
            let value = into_string(&bytes[start_from..bytes.len()]);
            let key = key.take().unwrap();

            result.push(LogEventContextApiModel { key, value });
        }
        ParseMode::AwaitingForSeparatorStart => {}
        ParseMode::AwaitingForSeparatorEnd => {}
    }

    result
}

fn is_key_started(b: u8) -> bool {
    if b.is_ascii_alphabetic() {
        return true;
    }

    if b == b'"' {
        return true;
    }
    if b == b'\'' {
        return true;
    }

    return false;
}

fn is_key_finished(b: u8) -> bool {
    if b == b'"' {
        return true;
    }
    if b == b'\'' {
        return true;
    }

    if b == b':' || b == b'=' {
        return true;
    }

    !b.is_ascii_digit() && !b.is_ascii_alphabetic() && b != b'_'
}

fn is_value_started(b: u8) -> bool {
    if b.is_ascii_alphabetic() {
        return true;
    }

    if b == b'"' {
        return true;
    }
    if b == b'\'' {
        return true;
    }

    return false;
}

fn is_value_finished(b: u8) -> bool {
    if b == b'"' {
        return true;
    }
    if b == b'\'' {
        return true;
    }

    b <= 32
}

fn is_separator_started(b: u8) -> Option<SeparatorType> {
    if b == b',' {
        return Some(SeparatorType::SingleChar);
    }
    if b == b';' {
        return Some(SeparatorType::SingleChar);
    }
    if b == b':' {
        return Some(SeparatorType::SingleChar);
    }
    if b.is_ascii_alphabetic() {
        return Some(SeparatorType::Word);
    }

    None
}

fn is_separator_ended(b: u8) -> bool {
    b == b' '
}

fn into_string(src: &[u8]) -> String {
    if src[0] == b'"' {
        return std::str::from_utf8(&src[1..src.len()]).unwrap().to_string();
    }

    if src[0] == b'\'' {
        return std::str::from_utf8(&src[1..src.len()]).unwrap().to_string();
    }

    std::str::from_utf8(src).unwrap().to_string()
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_single_key_value_several_scenarios() {
        let result = super::parse_key_value_from_string("key=value");
        assert_eq!(1, result.len());

        assert_eq!("key", result.get(0).unwrap().key.as_str());
        assert_eq!("value", result.get(0).unwrap().value.as_str());

        let result = super::parse_key_value_from_string("key =    value");
        assert_eq!(1, result.len());

        assert_eq!("key", result.get(0).unwrap().key.as_str());
        assert_eq!("value", result.get(0).unwrap().value.as_str());

        let result = super::parse_key_value_from_string("key='value'");
        assert_eq!(1, result.len());

        assert_eq!("key", result.get(0).unwrap().key.as_str());
        assert_eq!("value", result.get(0).unwrap().value.as_str());

        let result = super::parse_key_value_from_string("'key'='value'");
        assert_eq!(1, result.len());

        assert_eq!("key", result.get(0).unwrap().key.as_str());
        assert_eq!("value", result.get(0).unwrap().value.as_str());
    }

    #[test]
    fn test_two_key_values_with_one_char_separator() {
        let result = super::parse_key_value_from_string("key0=value0,key1=value1");
        assert_eq!(2, result.len());

        assert_eq!("key0", result.get(0).unwrap().key.as_str());
        assert_eq!("value0", result.get(0).unwrap().value.as_str());

        assert_eq!("key1", result.get(1).unwrap().key.as_str());
        assert_eq!("value1", result.get(1).unwrap().value.as_str());

        let result = super::parse_key_value_from_string("key0=value0;key1=value1");
        assert_eq!(2, result.len());

        assert_eq!("key0", result.get(0).unwrap().key.as_str());
        assert_eq!("value0", result.get(0).unwrap().value.as_str());

        assert_eq!("key1", result.get(1).unwrap().key.as_str());
        assert_eq!("value1", result.get(1).unwrap().value.as_str());
    }

    #[test]
    fn test_two_key_values_with_one_char_separator_and_spaces() {
        let result = super::parse_key_value_from_string("key0=value0  ,   key1=value1");
        assert_eq!(2, result.len());

        assert_eq!("key0", result.get(0).unwrap().key.as_str());
        assert_eq!("value0", result.get(0).unwrap().value.as_str());

        assert_eq!("key1", result.get(1).unwrap().key.as_str());
        assert_eq!("value1", result.get(1).unwrap().value.as_str());

        let result = super::parse_key_value_from_string("key0  = 'value0'  ;   key1=\"value1\"");
        assert_eq!(2, result.len());

        assert_eq!("key0", result.get(0).unwrap().key.as_str());
        assert_eq!("value0", result.get(0).unwrap().value.as_str());

        assert_eq!("key1", result.get(1).unwrap().key.as_str());
        assert_eq!("value1", result.get(1).unwrap().value.as_str());
    }

    #[test]
    fn test_two_key_values_with_a_separator_as_word() {
        let result = super::parse_key_value_from_string("key0=value0  and  key1=value1");
        assert_eq!(2, result.len());

        assert_eq!("key0", result.get(0).unwrap().key.as_str());
        assert_eq!("value0", result.get(0).unwrap().value.as_str());

        assert_eq!("key1", result.get(1).unwrap().key.as_str());
        assert_eq!("value1", result.get(1).unwrap().value.as_str());

        let result = super::parse_key_value_from_string("key0  = 'value0' AND  key1=\"value1\"");
        assert_eq!(2, result.len());

        assert_eq!("key0", result.get(0).unwrap().key.as_str());
        assert_eq!("value0", result.get(0).unwrap().value.as_str());

        assert_eq!("key1", result.get(1).unwrap().key.as_str());
        assert_eq!("value1", result.get(1).unwrap().value.as_str());
    }
}
