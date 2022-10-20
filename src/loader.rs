use loading::Loading;
use std::time::Instant;

pub struct Manager {
    start_time: Instant,
    loader: Loading,
    title: String,
    pub is_finished: bool,
}

impl Manager {
    pub fn new(text: &str) -> Self {
        let new_manager = Self {
            start_time: Instant::now(),
            loader: Loading::default(),
            title: text.to_string(),
            is_finished: false,
        };
        new_manager.loader.text(&new_manager.title);
        new_manager
    }

    pub fn end(&mut self, result: &str) {
        let time = Instant::now().duration_since(self.start_time).as_secs_f32();
        let formatted_text = format!("{} \x1B[1;32mDONE!\x1B[0m time: {}", self.title, time);

        match result {
            "success" => self.loader.success(formatted_text),
            "fail" => self.loader.fail(formatted_text),
            "warn" => self.loader.warn(formatted_text),
            "info" => self.loader.info(formatted_text),
            _ => panic!("Invalid Result Chosen!!"),
        }

        self.is_finished = true;
        self.loader.end();
    }
}
