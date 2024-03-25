pub mod base;
pub mod rolling;
pub mod track;
pub mod train;
pub mod world;

pub type Num = f32;

pub fn run() {
    let w = world::World::load("/tmp/world.json")
        .map(|w| {
            eprintln!("The world is succesfully loaded");
            w
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
        .map(|w| {
            eprintln!("World saved successfully");
            w
        })
        .inspect_err(|e| eprintln!("Could not save the world because of error: {:?}", e));
}

fn build_world() -> world::World {
    let mut fiat_lux = world::World::default();

    let track_id = fiat_lux.append(track::Track::new(105647.8, 1000., base::Direction::Odd));
    let car_id1 = fiat_lux.append(rolling::Car::new(75.0));
    let car_id2 = fiat_lux.append(rolling::Car::new(75.0));
    let train = train::Train::new(base::Direction::Odd)
        .make_up(track_id, &[car_id1, car_id2], &mut fiat_lux)
        .unwrap();
    let train_id = fiat_lux.append(train);

    eprintln!("Built the world from scratch");
    fiat_lux
}
