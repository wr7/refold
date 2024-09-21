use std::{
    io::{self, BufRead as _, StdinLock},
    mem,
};

/// Iterator of lines in stdin.
///
/// This differs from the version in `std` in two ways:
///  - the `\n` and `\r\n` are not removed from the end
///  - if there are >= 2 trailing newlines, one is ignored
///
pub struct Lines {
    stdin: StdinLock<'static>,
    buf: String,
    next_line: io::Result<String>,
}

impl Lines {
    pub fn new(mut stdin: StdinLock<'static>) -> Self {
        let mut next_line_buf = String::new();
        let next_line = stdin.read_line(&mut next_line_buf).map(|_| next_line_buf);

        Self {
            stdin,
            buf: String::new(),
            next_line,
        }
    }

    pub fn set_buf(&mut self, mut buf: String) -> String {
        self.buf.clear();
        mem::swap(&mut self.buf, &mut buf);
        buf
    }
}

impl Iterator for Lines {
    type Item = io::Result<String>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf: String = self.set_buf(String::new());

        let mut line = self.stdin.read_line(&mut buf).map(|_| buf);
        std::mem::swap(&mut self.next_line, &mut line);

        let line: String = match line {
            Ok(l) => l,
            Err(e) => return Some(Err(e)),
        };

        // Ignore last newline if there are >= 2 trailing newlines
        if (&line == "\n" || &line == "\r\n") && self.next_line.as_ref().is_ok_and(String::is_empty)
        {
            return None;
        }

        if line.is_empty() {
            None
        } else {
            Some(Ok(line))
        }
    }
}
