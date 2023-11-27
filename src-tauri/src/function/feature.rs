pub mod application;

pub enum Feature {
    Application { file_path: &'static str },
    Sentence(),
}

impl Feature {
    pub fn execute(&self) {
        println!("debug a");
        match self {
            Feature::Application { file_path } => {
                println!("debug b");
                application::run_application(file_path);
            }
            _ => {}
        }
    }
}
