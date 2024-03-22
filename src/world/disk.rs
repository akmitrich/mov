use crate::{rolling::Car, track::Track};

use super::World;

pub(super) fn save(path: impl AsRef<std::path::Path>, world: &World) {
    let mut w = serde_json::json!({});
    if let serde_json::Value::Object(map) = &mut w {
        map.insert(
            "tracks".to_owned(),
            serde_json::to_value(world.items_of_type::<Track>()).unwrap(),
        );
        map.insert(
            "cars".to_owned(),
            serde_json::to_value(world.items_of_type::<Car>()).unwrap(),
        );
    }
    let output = std::fs::File::create(path).unwrap();
    let _ = serde_json::to_writer_pretty(output, &w);
}
