enum Feature {
    Application(),
    Sentence(),
}

struct Function {
    macro_type: MacroType,
}

impl Function {
    pub fn new(feature: feature) -> Self {
        Self { feature }
    }

    pub fn perform() {
        match macro_type {}
    }
}

