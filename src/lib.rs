pub mod base;
pub mod rolling;
pub mod track;
pub mod world;

pub type Num = f32;

pub fn run() {
    let w = world::World::load("/tmp/world.json")
        .map(|w| {
            eprintln!("The world is succesfully loaded");
            w
        })
        .unwrap_or_else(build_world);

    let track_id = w
        .items_of_type::<track::Track>()
        .iter()
        .next()
        .map(|(id, _)| *id)
        .unwrap();
    let car_id = w
        .items_of_type::<rolling::Car>()
        .iter()
        .next()
        .map(|(id, _)| *id)
        .unwrap();
    println!(
        "Track: {}\nCar: {}",
        serde_json::to_string_pretty(w.item::<track::Track>(track_id).unwrap()).unwrap(),
        serde_json::to_string_pretty(w.item::<rolling::Car>(car_id).unwrap()).unwrap()
    );

    w.save("/tmp/world.json");
}

fn build_world() -> world::World {
    let mut fiat_lux = world::World::default();

    let track_id = fiat_lux.append(track::Track::new(105647.8, 2345.1, base::Direction::Even));
    let car_id = fiat_lux.append(rolling::Car::new(75.0));
    if let Some(track) = fiat_lux.item_mut::<track::Track>(track_id) {
        track.place_car(car_id, base::Direction::Even);
    }
    if let Some(car) = fiat_lux.item_mut::<rolling::Car>(car_id) {
        car.you_are_on(track_id);
    }
    fiat_lux
}
