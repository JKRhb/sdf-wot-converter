use serde_json::{json, Map, Number, Value};

type ThingModel = Map<String, Value>;

enum TmConversionOutput {
    ThingModel(ThingModel),
    ThingModelCollection(Map<String, ThingModel>),
}

fn convert_sdf_to_wot_tm(
    sdf_model: Map<String, Value>,
    sdf_mapping_files: Option<Vec<Map<String, Value>>>,
    origin_url: Option<String>,
    set_instance_version: Option<bool>,
    suppress_roundtripping: Option<bool>,
) -> TmConversionOutput {
    let mut blah = sdf_model.clone();

    blah["test"] = json!(5);

    TmConversionOutput::ThingModel(blah)
}
