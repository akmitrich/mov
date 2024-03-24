pub mod base;
pub mod rolling;
pub mod track;
pub mod world;

pub type Num = f32;

pub fn run() {
    let w = world::World::load("/tmp/world.json")
        .inspect(|_| {
            eprintln!("The world is succesfully loaded");
        })
        .inspect_err(|e| eprintln!("Error while loading {:?}. Let's start from scratch", e))
        .unwrap_or_else(|_| build_world());

    let track_id = w
        .items_of_type::<track::Track>()
        .iter()
        .next()
        .map(|(id, _)| *id)
        .unwrap();
    let (_, car) = w
        .items_of_type::<rolling::Car>()
        .iter()
        .next()
        .map(|(id, car)| (*id, *car))
        .unwrap();
    println!(
        "Track: {}\nCar: {}",
        serde_json::to_string_pretty(w.item::<track::Track>(track_id).unwrap()).unwrap(),
        car.global_position(&w)
    );

    let _ = w
        .save("/tmp/world.json")
        .inspect(|_| eprintln!("World saved successfully"))
        .inspect_err(|e| eprintln!("Could not save the world because of error: {:?}", e));
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

    eprintln!("Built the world from scratch");
    fiat_lux
}
