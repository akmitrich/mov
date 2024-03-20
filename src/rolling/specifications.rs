use crate::Num;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Specifications {
    inner: BaseSpecifications,
}

impl Specifications {
    pub fn new(mass: Num) -> Self {
        Self {
            inner: BaseSpecifications { mass },
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct BaseSpecifications {
    pub mass: Num,
}
