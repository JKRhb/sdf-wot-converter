mod wot;
use wot::definitions::SDFModel;
use serde_json::Result;

fn test_function() -> Result<String> {
    let sdf_example = r##"{
        "info": {
            "title": "Example file for OneDM Semantic Definition Format",
            "version": "2019-04-24",
            "copyright": "Copyright 2019 Example Corp. All rights reserved.",
            "license": "https://example.com/license"
        },
        "namespace": {
            "cap": "https://example.com/capability/cap"
        },
        "defaultNamespace": "cap",
        "sdfObject": {
            "Switch": {
                "sdfProperty": {
                    "value": {
                        "description": "The state of the switch; false for off and true for on.",
                        "type": "boolean"
                    }
                },
                "sdfAction": {
                    "on": {
                        "description": "Turn the switch on; equivalent to setting value to true."
                    },
                    "off": {
                        "description": "Turn the switch off; equivalent to setting value to false."
                    },
                    "toggle": {
                        "description": "Toggle the switch; equivalent to setting value to its complement."
                    }
                }
            }
        }
    }"##;

    let sdf_model: SDFModel = serde_json::from_str(sdf_example)?;

    let j = serde_json::to_string_pretty(&sdf_model)?;

    return Ok(j);
}


fn main() {
    match test_function() {
        Ok(result)  => println!("{}", result),
        Err(e) => println!("{}", e),
    };
}
