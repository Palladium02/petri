use std::range::Range;

pub trait Outline {
    fn outline(&self, span: Range<usize>) -> String;
}

impl Outline for &str {
    fn outline(&self, span: Range<usize>) -> String {
        let mut output = Vec::new();
        let source = &self[span];
        for line in source.lines() {
            output.push(line.to_string());
            output.push("^".repeat(line.len()));
        }

        output.join("\n")
    }
}
