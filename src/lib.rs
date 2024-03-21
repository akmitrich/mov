pub mod base;
pub mod rolling;
pub mod track;
pub mod world;

pub type Num = f32;

pub fn run() {
    let mut w = world::World::default();

    let track_id = w.append(track::Track::new(105647.8, 2345.1, base::Direction::Even));
    let car_id = w.append(rolling::Car::new(75.0));
    if let Some(track) = w.item_mut::<track::Track>(track_id) {
        track.place_car(car_id, base::Direction::Even);
    }
    if let Some(car) = w.item_mut::<rolling::Car>(car_id) {
        car.you_are_on(track_id);
    }
    println!("World debug: {:#?}", w);
    println!(
        "Track: {}\nCar: {}",
        serde_json::to_string_pretty(w.item::<track::Track>(track_id).unwrap()).unwrap(),
        serde_json::to_string_pretty(w.item::<rolling::Car>(car_id).unwrap()).unwrap()
    );
}
