pub fn subscribe(channel: &str, symbols: &Vec<String>) -> Vec<u8> {
    let symbols: String = symbols
        .iter()
        .map(|sym| format!(r#""{sym}""#))
        .collect::<Vec<String>>()
        .join(", ");
    let message = format!(
        r#"{{"method": "subscribe", "params": {{ "channel": "{channel}", "symbol": [{symbols}]}}}}"#
    );
    return message.as_bytes().to_vec();
}
