pub trait SourceProcessor {
    fn process(&mut self, name: &str, source: String) -> String;
}

pub type Preprocessor = Box<dyn SourceProcessor>;