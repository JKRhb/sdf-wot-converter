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
