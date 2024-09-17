pub fn hour_key_to_string(hour_key: i64) -> String {
    let mut result = hour_key.to_string();

    result.insert(4, '-');
    result.insert(7, '-');
    result.insert(10, 'T');
    result.insert(13, ':');
    result.push_str("00:00");

    result
}
