use super::World;
use crate::{rolling::Car, track::Track};
use serde_json::json;

pub(super) fn save(path: impl AsRef<std::path::Path>, world: &World) -> anyhow::Result<()> {
    let output = std::fs::File::create(path)?;
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
    Ok(serde_json::to_writer_pretty(output, &w)?)
}

pub(super) fn load(path: impl AsRef<std::path::Path>) -> anyhow::Result<World> {
    let input = std::fs::File::open(path)?;
    let mut builder = super::builder::B::default();
    if let serde_json::Value::Object(map) = serde_json::from_reader(input)? {
        builder = builder
            .tracks(
                serde_json::from_value(map.get("tracks").cloned().unwrap_or_else(|| json!({})))
                    .ok(),
            )
            .cars(
                serde_json::from_value(map.get("cars").cloned().unwrap_or_else(|| json!({}))).ok(),
            );
    }
    builder.build().ok_or_else(|| anyhow::Error::msg("ERROR"))
}
