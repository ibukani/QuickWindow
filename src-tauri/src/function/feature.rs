pub mod application;

#[derive(Clone)]
pub enum Feature {
    Application {
        open_file_path: &'static str,
        process_file_path: &'static str,
    },
    Sentence(),
}

impl Feature {
    pub fn application_with_open(
        open_file_path: &'static str,
        process_file_path: &'static str,
    ) -> Self {
        Self::Application {
            open_file_path,
            process_file_path,
        }
    }

    pub fn application(file_path: &'static str) -> Self {
        Self::Application {
            open_file_path: &file_path,
            process_file_path: &file_path,
        }
    }

    pub fn execute(&self) {
        match self {
            Feature::Application {
                open_file_path,
                process_file_path,
            } => {
                application::run_application(open_file_path, process_file_path);
            }
            _ => {}
        }
    }
}
