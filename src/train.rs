use crate::{
    base::Direction,
    rolling::Car,
    track::Track,
    world::{Item, World},
    Num,
};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Train {
    cars: Vec<uuid::Uuid>,
    direction: Direction,
}

impl Train {
    pub fn new(direction: Direction) -> Self {
        Self {
            cars: Vec::new(),
            direction,
        }
    }

    pub fn make_up(
        mut self,
        track_id: uuid::Uuid,
        cars: &[uuid::Uuid],
        w: &mut World,
    ) -> Option<Self> {
        self.cars = cars.to_vec();
        let track_length = w.item::<Track>(track_id)?.length();
        let length = self.length(w);
        let mut car_pos = 0.5
            * (track_length + self.direction * (length - calc_car_length(*self.cars.first()?, w)));
        for car_id in self.cars.iter() {
            let _ = w.item_mut::<Car>(*car_id).map(|car| {
                car.place_at(car_pos);
                car.you_are_on(track_id);
            });
            let _ = w
                .item_mut::<Track>(track_id)
                .map(|track| track.place_car(*car_id, -self.direction));
            car_pos -= calc_car_length(*car_id, w);
        }
        Some(self)
    }

    pub fn length(&self, w: &World) -> Num {
        calc_train_length(&self.cars, w)
    }
}

impl Item for Train {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

fn calc_train_length(cars: &[uuid::Uuid], w: &World) -> Num {
    cars.iter()
        .fold(0.0, |length, car_id| length + calc_car_length(*car_id, w))
}

fn calc_car_length(car: uuid::Uuid, w: &World) -> Num {
    w.item::<Car>(car).map(Car::length).unwrap_or(0.0)
}
