pub mod base;
pub mod rolling;
pub mod track;

pub type Num = f32;

pub fn run() {
    let mut c = std::collections::HashMap::<uuid::Uuid, rolling::Car>::from_iter(vec![(
        uuid::Uuid::new_v4(),
        rolling::Car::new(25.3),
    )]);

    let mut t = std::collections::HashMap::<uuid::Uuid, track::Track>::from_iter(vec![(
        uuid::Uuid::new_v4(),
        track::Track::new(105874.6, 2395.6, base::Direction::Even),
    )]);

    c.iter_mut().next().map(|(car_id, car)| {
        t.iter_mut().next().map(|(track_id, track)| {
            car.you_are_on(*track_id);
            track.accept_car(*car_id, base::Direction::Even)
        })
    });

    println!("Cars: {}", serde_json::to_string_pretty(&c).unwrap());
    println!("Tracks: {}", serde_json::to_string_pretty(&t).unwrap());
}
