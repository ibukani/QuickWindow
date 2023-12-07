pub mod application;

#[derive(Clone)]
pub enum Feature {
    Application { file_path: &'static str },
    Sentence(),
}

impl Feature {
    pub fn execute(&self) {
        match self {
            Feature::Application { file_path } => {
                application::run_application(file_path);
            }
            _ => {}
        }
    }
}
