pub mod feature;

use feature::Feature;

#[derive(Clone)]
pub struct Function {
    feature: Feature,
}

impl Function {
    pub fn new(feature: Feature) -> Self {
        Self { feature }
    }

    pub fn perform(&self) {
        self.feature.execute()
    }
}
