use chardet::detect;
use nu_plugin::{EvaluatedCall, LabeledError};
use nu_protocol::{Span, Value};

pub fn detect_encoding_name(input: Span, bytes: &[u8]) -> Result<String, LabeledError> {
    let (encoding, _, _) = detect(bytes);
    if encoding.is_empty() {
        return Err(LabeledError {
            label: "Input contains unknown encoding".into(),
            msg: "try giving a encode name".into(),
            span: Some(input),
        });
    }
    match encoding.to_uppercase().as_str() {
        // these encodings are not supported by encoding_rs
        "UTF-32BE"
        | "UTF-32LE"
        | "X-ISO-100646-UCS-4-2143"
        | "X-ISO-10646-UCS-4-3412"
        | "X-EUC-TW"
        | "IBM855" => Err(LabeledError {
            label: "Unsupported encoding".into(),
            msg: format!("input is detected as {}", encoding),
            span: Some(input),
        }),
        // chardet returns non-standard names for these encodings
        "MACCYRILLIC" => Ok("x-mac-cyrillic".into()),
        // windows-31j extends shift_jis with some special characters
        "CP932" => Ok("windows-31j".into()),
        "CP949" => Ok("windows-949".into()),
        _ => Ok(encoding),
    }
}

pub fn detect_charset(call: &EvaluatedCall, input: &Value) -> Result<Value, LabeledError> {
    let res = detect(input.as_binary()?);
    Ok(Value::Record {
        cols: vec!["charset".into(), "language".into(), "confidence".into()],
        vals: vec![
            Value::String {
                val: res.0,
                span: call.head,
            },
            Value::String {
                val: res.2,
                span: call.head,
            },
            Value::Float {
                val: res.1 as f64,
                span: call.head,
            },
        ],
        span: call.head,
    })
}
