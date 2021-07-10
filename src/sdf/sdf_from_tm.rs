use super::definitions as sdf;
use crate::wot::definitions as wot;

/// Creates an info block from a Thing Model. I am a bit unsure how to map a
/// TM that has not been an SDF model before therefore this function only
/// returns `None` at the moment.
///
/// TODO: Investigate how to map this.
fn create_info_block(thing_model: &wot::ThingModel) -> Option<sdf::InfoBlock> {
    let title = thing_model.title.clone();
    let version = None;
    let copyright = None;
    let license = None;

    match (title, copyright, license, version) {
        (Some(title), Some(copyright), Some(license), Some(version)) => Some(sdf::InfoBlock {
            title,
            version,
            copyright,
            license,
        }),
        _ => None,
    }
}

/// Converts a WoT Thing Model into an SDF model.
impl From<wot::ThingModel> for sdf::SDFModel {
    fn from(thing_model: wot::ThingModel) -> Self {
        let info = create_info_block(&thing_model);
        let namespace = None;
        let default_namespace = None;
        let sdf_thing = None;
        let sdf_product = None;
        let sdf_object = None;
        let sdf_property = None;
        let sdf_action = None;
        let sdf_event = None;
        let sdf_data = None;

        sdf::SDFModel {
            info,
            namespace,
            default_namespace,
            sdf_thing,
            sdf_product,
            sdf_object,
            sdf_property,
            sdf_action,
            sdf_event,
            sdf_data,
        }
    }
}
