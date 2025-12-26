pub trait Sink {
    fn write(&mut self, line: String);
}

pub struct VecSink {
    lines: Vec<String>,
}

impl VecSink{
    pub fn new() -> Self {
        VecSink { lines: Vec::new() }
    }

    pub fn lines(&self) -> &[String] {
        &self.lines
    }
}

impl Sink for VecSink {
    fn write(&mut self, line: String) {
        self.lines.push(line);
    }
}

impl Default for VecSink {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_sink_new() {
        let sink = VecSink::new();
        assert!(sink.lines().is_empty());
    }

    #[test]
    fn test_vec_sink_default() {
        let sink = VecSink::default();
        assert!(sink.lines().is_empty());
    }

    #[test]
    fn test_vec_sink_write_single_line() {
        let mut sink = VecSink::new();
        sink.write("test line".to_string());
        assert_eq!(sink.lines().len(), 1);
        assert_eq!(sink.lines()[0], "test line");
    }

    #[test]
    fn test_vec_sink_write_multiple_lines() {
        let mut sink = VecSink::new();
        sink.write("line 1".to_string());
        sink.write("line 2".to_string());
        sink.write("line 3".to_string());

        assert_eq!(sink.lines().len(), 3);
        assert_eq!(sink.lines()[0], "line 1");
        assert_eq!(sink.lines()[1], "line 2");
        assert_eq!(sink.lines()[2], "line 3");
    }

    #[test]
    fn test_vec_sink_write_empty_string() {
        let mut sink = VecSink::new();
        sink.write(String::new());
        assert_eq!(sink.lines().len(), 1);
        assert_eq!(sink.lines()[0], "");
    }

    #[test]
    fn test_vec_sink_preserves_order() {
        let mut sink = VecSink::new();
        for i in 0..100 {
            sink.write(format!("line {}", i));
        }
        assert_eq!(sink.lines().len(), 100);
        for i in 0..100 {
            assert_eq!(sink.lines()[i], format!("line {}", i));
        }
    }

    #[test]
    fn test_vec_sink_lines_immutable_after_write() {
        let mut sink = VecSink::new();
        sink.write("first".to_string());
        let first_len = sink.lines().len();
        let first_value = sink.lines()[0].clone();
        
        sink.write("second".to_string());
        
        assert_eq!(first_len, 1);
        assert_eq!(sink.lines().len(), 2);
        assert_eq!(first_value, "first");
        assert_eq!(sink.lines()[0], "first");
        assert_eq!(sink.lines()[1], "second");
    }
}