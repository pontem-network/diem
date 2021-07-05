pub trait SourceProcessor {
    fn process(&mut self, name: &'static str, source: String) -> String;
}

pub struct NoOp;

impl SourceProcessor for NoOp {
    fn process(&mut self, _: &'static str, source: String) -> String {
        source
    }
}
