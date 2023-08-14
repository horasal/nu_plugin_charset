use charset::detect_charset;
use nu_plugin::{serve_plugin, EvaluatedCall, JsonSerializer, LabeledError, Plugin};
use nu_protocol::{Category, PluginExample, PluginSignature, Span, SyntaxShape, Type, Value};

mod charset;

struct Charset;

impl Charset {
    fn new() -> Self {
        Self {}
    }
}

impl Plugin for Charset {
    fn signature(&self) -> Vec<PluginSignature> {
        vec![
            PluginSignature::build("charset")
            .usage("detect charset of input string")
            .category(Category::Strings)
            .input_output_types(vec![
                (Type::Binary, Type::Record(vec![
                ("charset".into(), Type::String),
                ("language".into(),Type::String),
                ("confidence".into(), Type::Float)])), 
                (Type::String, Type::Record(vec![
                ("charset".into(), Type::String),
                ("language".into(),Type::String),
                ("confidence".into(), Type::Float)])), 
            ])
            .plugin_examples(vec![
                PluginExample {
                    description: "detect charet of input string".into(),
                    example: "open --raw shift_jis.txt | charset".into(),
                    result: Some(Value::Record {
                        cols: vec!["charset".into(), "language".into(), "confidence".into()], 
                        vals: vec![
                            Value::String { val: "SHIFT_JIS".into(), span: Span::test_data() },
                            Value::String { val: "Japanese".into(), span: Span::test_data()},
                            Value::Float { val: 0.9090909361839294, span: Span::test_data() }
                        ],
                        span: Span::test_data() }),
                },
            ]),
            PluginSignature::build("charset decode")
            .usage("decode string or binary to utf-8")
            .category(Category::Strings)
            .input_output_types(vec![
                (Type::Binary, Type::String),
                (Type::String, Type::String),
            ])
            .optional("charset", SyntaxShape::String, "target charset")
            .plugin_examples(vec![
                PluginExample {
                    description: "automatically convert input string to utf-8".into(),
                    example: "0x[82 a2 82 eb 82 cd 82 c9 82 d9 82 d6 82 c6 82 bf 82 e8 82 ca 82 e9 82 f0] | charset decode".into(),
                    result: Some(Value::String {
                        val: "いろはにほへとちりぬるを".into(), 
                        span: Span::test_data() }),
                },
                PluginExample {
                    description: "convert input string to utf-8 with specific charset".into(),
                    example: "0x[82 a2 82 eb 82 cd 82 c9 82 d9 82 d6 82 c6 82 bf 82 e8 82 ca 82 e9 82 f0] | charset decode shift_jis".into(),
                    result: Some(Value::String {
                        val: "いろはにほへとちりぬるを".into(), 
                        span: Span::test_data() }),
                },
            ]),
            PluginSignature::build("charset encode")
            .usage("encode utf-8 string to charset")
            .category(Category::Strings)
            .input_output_types(vec![
                (Type::String, Type::Binary),
            ])
            .required("charset", SyntaxShape::String, "target charset")
            .plugin_examples(vec![
                PluginExample {
                    description: "convert input utf-8 string to specific charset".into(),
                    example: "\"いろはにほへとちりぬるを\" | charset encode shift_jis".into(),
                    result: Some(Value::Binary {
                        val: vec![130, 162, 130, 235, 130, 205, 130, 201, 130, 217, 130, 214,
                        130, 198, 130, 191, 130, 232, 130, 202, 130, 233, 130, 240],
                        span: Span::test_data()}),
                },
            ]),

        ]
    }

    fn run(
        &mut self,
        name: &str,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        match name {
            "charset" => detect_charset(call, input),
            "charset decode" => {
                let charset_name: Option<String> = call.opt(0)?;
                let label = match charset_name {
                    Some(c) => c,
                    None => {
                        let (charset, ..) = chardet::detect(input.as_binary()?);
                        charset
                    }
                };

                let encoding = encoding_rs::Encoding::for_label(label.as_bytes());
                match encoding {
                    Some(e) => {
                        let input = input.as_binary()?;
                        let (s, _, has_error) = e.decode(input);
                        if has_error {
                            Err(LabeledError {
                                label: "invalid input".into(),
                                msg: format!("input is an invalid {} string ", label),
                                span: Some(call.head),
                            })
                        } else {
                            Ok(Value::String {
                                val: s.into(),
                                span: call.head,
                            })
                        }
                    }
                    None => Err(LabeledError {
                        label: "Unknown encoding".into(),
                        msg: format!("Unknown encoding"),
                        span: match call.positional.first().and_then(|v| v.span().ok()) {
                            Some(v) => Some(v),
                            None => Some(call.head),
                        },
                    }),
                }
            }
            "charset encode" => {
                let label: String = call.req(0)?;
                let encoding = encoding_rs::Encoding::for_label(label.as_bytes());
                match encoding {
                    Some(e) => {
                        let input = input.as_string()?;
                        let (s, _, has_error) = e.encode(&input);
                        if has_error {
                            Err(LabeledError {
                                label: "invalid input".into(),
                                msg: format!("input is an invalid {} string ", label),
                                span: Some(call.head),
                            })
                        } else {
                            Ok(Value::Binary {
                                val: s.into(),
                                span: call.head,
                            })
                        }
                    }
                    None => Err(LabeledError {
                        label: "Unknown encoding".into(),
                        msg: format!("Unknown encoding"),
                        span: match call.positional.first().and_then(|v| v.span().ok()) {
                            Some(v) => Some(v),
                            None => Some(call.head),
                        },
                    }),
                }
            }
            _ => Err(LabeledError {
                label: "Plugin call with wrong name signature".into(),
                msg: "Plugin command should be one of decode, encode, list".into(),
                span: Some(call.head),
            }),
        }
    }
}

fn main() {
    serve_plugin(&mut Charset::new(), JsonSerializer)
}
