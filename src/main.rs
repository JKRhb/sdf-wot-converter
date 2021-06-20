mod sdf;
mod wot;
use sdf::definitions::SDFModel;
use wot::definitions::Thing;
use serde_json::Result;

fn sdf_test_function() -> Result<String> {
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
                        "type": "boolean",
                        "sdfType": "byte-string"
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

fn wot_test_function() -> Result<String> {
  let sdf_example = r##"{
    "@context": ["http://www.w3.org/ns/td", {"saref": "blah.com"}],
    "@type": "saref:yo",
    "id": "urn:dev:ops:32473-WoTLamp-1234",
    "created": "2018-11-14T19:10:23.824Z",
    "title": "MyLampThing",
    "securityDefinitions": {
        "basic_sc": {"scheme": "basic", "in": "header"}
    },
    "security": "basic_sc",
    "properties": {
        "status": {
            "type": "string",
            "forms": [{"href": "https://mylamp.example.com/status"}]
        }
    },
    "actions": {
        "toggle": {
            "forms": [{"href": "https://mylamp.example.com/toggle"}]
        }
    },
    "events":{
        "overheating":{
            "data": {"type": "string"},
            "forms": [{
                "href": "https://mylamp.example.com/oh",
                "subprotocol": "longpoll"
            }]
        }
    }
}"##;

  let sdf_model: Thing = serde_json::from_str(sdf_example)?;

  let j = serde_json::to_string_pretty(&sdf_model)?;

  return Ok(j);
}


fn main() {
    match sdf_test_function() {
        Ok(result)  => println!("{}", result),
        Err(e) => println!("{}", e),
    };

    match wot_test_function() {
        Ok(result)  => println!("{}", result),
        Err(e) => println!("{}", e),
    };
}
