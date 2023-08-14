use chardet::detect;
use nu_plugin::{EvaluatedCall, LabeledError};
use nu_protocol::Value;

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
