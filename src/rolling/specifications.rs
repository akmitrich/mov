use crate::Num;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Specifications {
    inner: BaseSpecifications,
}

impl Specifications {
    pub fn new(mass: Num, length: Num) -> Self {
        Self {
            inner: BaseSpecifications { mass, length },
        }
    }

    pub fn length(&self) -> Num {
        self.inner.length
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct BaseSpecifications {
    pub mass: Num,
    pub length: Num,
}
