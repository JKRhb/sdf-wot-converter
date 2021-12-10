use assert_json_diff::assert_json_include;
use sdf_wot_converter::converter;
use serde_json::json;

fn test_sdf_wot_conversion(sdf_input: &str, expected_result: serde_json::Value) {
    let result = converter::convert_sdf_to_wot_tm(sdf_input.to_string()).unwrap();
    let result_json: serde_json::Value = serde_json::from_str(result.as_str()).unwrap();

    assert_json_include!(actual: result_json, expected: expected_result);
}

#[test]
fn test_empty_sdf_tm_conversion() {
    let sdf_input = "{}";
    let expected_tm_result = json!(
        {
            "@context": [
              "https://www.w3.org/2019/wot/td/v1"
            ],
            "@type": "Thing"
        }
    );

    test_sdf_wot_conversion(sdf_input, expected_tm_result);
}

#[test]
fn test_sdf_tm_type_conversion() {
    let sdf_input = r#"{
        "sdfProperty": {
            "foo": {
                "type": "integer",
                "minimum": 0,
                "maximum": 9002,
                "exclusiveMinimum": 0,
                "exclusiveMaximum": 9000,
                "multipleOf": 2
            },
            "bar": {
                "type": "number",
                "minimum": 0.0,
                "maximum": 9002.0,
                "exclusiveMinimum": 0.0,
                "exclusiveMaximum": 9000.0,
                "multipleOf": 2.0
            },
            "baz": {
                "type": "string",
                "minLength": 3,
                "maxLength": 5,
                "pattern": "email",
                "format": "uri-reference"
            },
            "foobar": {
                "type": "array",
                "minItems": 2,
                "maxItems": 5
            },
            "barfoo": {
                "type": "object",
                "required": ["foo"]
            }
        }
    }"#;
    let expected_tm_result = json!(
        {
            "@context": [
              "https://www.w3.org/2019/wot/td/v1"
            ],
            "@type": "Thing",
            "properties": {
                "foo": {
                    "type": "integer",
                    "minimum": 0,
                    "maximum": 9002,
                    "exclusiveMinimum": 0,
                    "exclusiveMaximum": 9000,
                    "multipleOf": 2
                },
                "bar": {
                    "type": "number",
                    "minimum": 0.0,
                    "maximum": 9002.0,
                    "exclusiveMinimum": 0.0,
                    "exclusiveMaximum": 9000.0,
                    "multipleOf": 2.0
                },
                "baz": {
                    "type": "string",
                    "minLength": 3,
                    "maxLength": 5,
                    "pattern": "email",
                    "format": "uri-reference"
                },
                "foobar": {
                    "type": "array",
                    "minItems": 2,
                    "maxItems": 5
                },
                "barfoo": {
                    "type": "object",
                    "required": ["foo"]
                }
            }
        }
    );

    test_sdf_wot_conversion(sdf_input, expected_tm_result);
}

#[test]
fn test_sdf_tm_action_conversion() {
    let sdf_input = r#"{
        "sdfAction": {
            "foobar": {
                "sdfInputData": {
                    "type": "string"
                },
                "sdfOutputData": {
                    "type": "string"
                }
            }
        }
    }"#;
    let expected_tm_result = json!(
        {
            "@context": [
              "https://www.w3.org/2019/wot/td/v1"
            ],
            "@type": "Thing",
            "actions": {
                "foobar": {
                    "input": {
                        "type": "string"
                    },
                    "output": {
                        "type": "string"
                    }
                }
            }
        }
    );

    test_sdf_wot_conversion(sdf_input, expected_tm_result);
}

#[test]
fn test_sdf_tm_event_conversion() {
    let sdf_input = r#"{
        "sdfEvent": {
            "foobar": {
                "sdfOutputData": {
                    "type": "string"
                }
            }
        }
    }"#;
    let expected_tm_result = json!(
        {
            "@context": [
              "https://www.w3.org/2019/wot/td/v1"
            ],
            "@type": "Thing",
            "events": {
                "foobar": {
                    "data": {
                        "type": "string"
                    },
                }
            }
        }
    );

    test_sdf_wot_conversion(sdf_input, expected_tm_result);
}

#[test]
fn test_sdf_tm_sdf_ref_conversion() {
    let sdf_input = r##"{
        "sdfAction": {
            "foobar": {
                "label": "hi"
            },
            "foobaz": {
                "sdfRef": "#/sdfAction/foobar"
            }
        },
        "sdfEvent": {
            "foobar": {
                "label": "hi"
            },
            "foobaz": {
                "sdfRef": "#/sdfEvent/foobar"
            }
        },
        "sdfProperty": {
            "foobar": {
                "label": "hi"
            },
            "foobaz": {
                "sdfRef": "#/sdfProperty/foobar"
            }
        }
    }"##;
    let expected_tm_result = json!(
        {
            "@context": [
              "https://www.w3.org/2019/wot/td/v1"
            ],
            "@type": "Thing",
            "actions": {
                "foobar": {
                    "title": "hi"
                },
                "foobaz": {
                    "title": "hi"
                }
            },
            "properties": {
                "foobar": {
                    "title": "hi"
                },
                "foobaz": {
                    "title": "hi"
                }
            },
            "events": {
                "foobar": {
                    "title": "hi"
                },
                "foobaz": {
                    "title": "hi"
                }
            }
        }
    );

    test_sdf_wot_conversion(sdf_input, expected_tm_result);
}

fn test_wot_tm_sdf_conversion(wot_tm_input: &str, expected_result: serde_json::Value) {
    let result = converter::convert_wot_tm_to_sdf(wot_tm_input.to_string()).unwrap();
    let result_json: serde_json::Value = serde_json::from_str(result.as_str()).unwrap();

    assert_json_include!(actual: result_json, expected: expected_result);
}

#[test]
fn test_empty_tm_sdf_conversion() {
    let wot_tm_input = r#"
        {
            "@context": [
              "https://www.w3.org/2019/wot/td/v1"
            ],
            "@type": "Thing"
        }
    "#;
    let expected_sdf_result = json!({});

    test_wot_tm_sdf_conversion(wot_tm_input, expected_sdf_result);
}

#[test]
fn test_tm_sdf_property_conversion() {
    let wot_tm_input = r#"
        {
            "@context": [
              "https://www.w3.org/2019/wot/td/v1"
            ],
            "@type": "Thing",
            "properties": {
                "boo": {
                    "type": "boolean",
                    "const": true,
                    "default": true
                },
                "foo": {
                    "type": "integer",
                    "minimum": 0,
                    "maximum": 9001,
                    "exclusiveMaximum": 9002,
                    "exclusiveMinimum": 1,
                    "multipleOf": 1,
                    "const": 5,
                    "default": 5
                },
                "bar": {
                    "type": "number",
                    "minimum": 0.0,
                    "maximum": 9001.0,
                    "exclusiveMaximum": 9002.0,
                    "exclusiveMinimum": 1.0,
                    "multipleOf": 1.0,
                    "const": 5.0,
                    "default": 5.0
                },
                "baz": {
                    "type": "string",
                    "minLength": 3,
                    "maxLength": 5,
                    "pattern": "email"
                },
                "foobar": {
                    "type": "array",
                    "minItems": 2,
                    "maxItems": 5
                },
                "barfoo": {
                    "type": "object",
                    "required": ["foo"]
                }
            }
        }
    "#;
    let expected_sdf_result = json!({
        "sdfProperty": {
            "boo": {
                "type": "boolean",
                "const": true,
                "default": true
            },
            "foo": {
                "type": "integer",
                "minimum": 0,
                "maximum": 9001,
                "exclusiveMaximum": 9002,
                "exclusiveMinimum": 1,
                "multipleOf": 1,
                "const": 5,
                "default": 5
            },
            "bar": {
                "type": "number",
                "minimum": 0.0,
                "maximum": 9001.0,
                "exclusiveMaximum": 9002.0,
                "exclusiveMinimum": 1.0,
                "multipleOf": 1.0,
                "const": 5.0,
                "default": 5.0
            },
            "baz": {
                "type": "string",
                "minLength": 3,
                "maxLength": 5,
                "pattern": "email"
            },
            "foobar": {
                "type": "array",
                "minItems": 2,
                "maxItems": 5
            },
            "barfoo": {
                "type": "object",
                "required": ["foo"]
            }
        }
    });

    test_wot_tm_sdf_conversion(wot_tm_input, expected_sdf_result);
}

#[test]
fn test_tm_sdf_action_conversion() {
    let wot_tm_input = r#"
        {
            "@context": [
              "https://www.w3.org/2019/wot/td/v1"
            ],
            "@type": "Thing",
            "actions": {
                "foo": {
                    "input": {
                        "type": "integer"
                    },
                    "output": {
                    }
                }
            }
        }
    "#;
    let expected_sdf_result = json!({
        "sdfAction": {
            "foo": {
                "sdfInputData": {
                    "type": "integer"
                },
                "sdfOutputData": {
                }
            }
        }
    });

    test_wot_tm_sdf_conversion(wot_tm_input, expected_sdf_result);
}

#[test]
fn test_tm_sdf_event_conversion() {
    let wot_tm_input = r#"
        {
            "@context": [
              "https://www.w3.org/2019/wot/td/v1"
            ],
            "@type": "Thing",
            "events": {
                "foo": {
                    "data": {
                        "type": "integer"
                    }
                }
            }
        }
    "#;
    let expected_sdf_result = json!({
        "sdfEvent": {
            "foo": {
                "sdfOutputData": {
                    "type": "integer"
                }
            }
        }
    });

    test_wot_tm_sdf_conversion(wot_tm_input, expected_sdf_result);
}
