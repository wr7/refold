use std::io::{self, StdoutLock, Write as _};

use textwrap::LineEnding;

use crate::{cmdline::Parameters, util};

pub struct Refolder<'a> {
    stdin: util::Lines,
    stdout: StdoutLock<'static>,

    parameters: &'a Parameters,

    prefix: Option<String>,
    line_ending: Option<LineEnding>,

    /// Whether or not to put a trailing newline at the end of the output
    trailing_newline: bool,

    /// The accumulated text that will be wrapped when the current paragraph
    /// ends
    current_paragraph: String,
    /// Buffer for current line of stdin
    line_buf: String,
}

impl<'a> Refolder<'a> {
    pub fn new(parameters: &'a Parameters) -> Self {
        Self {
            stdin: util::Lines::new(std::io::stdin().lock()),
            stdout: std::io::stdout().lock(),

            parameters,

            prefix: parameters.prefix.clone(),
            line_ending: None,
            trailing_newline: false,

            current_paragraph: String::new(),
            line_buf: String::new(),
        }
    }

    fn detect_prefix(&mut self) {
        if self.prefix.is_some() || self.line_buf.is_empty() {
            return;
        }

        let mut prefix_len = 0;

        for (i, char) in self.line_buf.char_indices() {
            if char.is_ascii_punctuation() | char.is_ascii_whitespace() {
                prefix_len = i + 1;
            } else {
                break;
            }
        }

        self.prefix = Some(self.line_buf[..prefix_len].to_owned());
    }

    pub fn refold(mut self) {
        loop {
            self.stdin.set_buf(self.line_buf);

            self.line_buf = self
                .stdin
                .next()
                .transpose()
                .unwrap_or_else(|err| {
                    eprintln!("refold: failed to read from stdin: {err}");

                    std::process::exit(err.raw_os_error().unwrap_or(1));
                })
                .unwrap_or_default();

            let mut eof = true;

            if self.line_buf.ends_with('\n') {
                eof = false;
                self.line_buf.pop();

                if self.line_buf.ends_with('\r') {
                    self.line_ending = Some(LineEnding::CRLF);
                    self.line_buf.pop();
                } else {
                    self.line_ending = Some(LineEnding::LF);
                }
            }

            self.detect_prefix();

            let line = self
                .prefix
                .as_deref()
                .and_then(|prefix| self.line_buf.strip_prefix(prefix))
                .unwrap_or(&self.line_buf);

            if !line.is_empty() {
                self.current_paragraph += &line;

                if !eof {
                    self.current_paragraph += " ";
                }

                self.trailing_newline = !eof;

                continue;
            }

            self.wrap_paragraph(eof).unwrap_or_else(|err| {
                eprintln!("refold: failed to write to stdout: {err}");

                std::process::exit(err.raw_os_error().unwrap_or(1));
            });

            if eof {
                break;
            }
        }
    }

    fn wrap_paragraph(&mut self, eof: bool) -> io::Result<()> {
        self.stdout.write_all(
            textwrap::fill(
                &self.current_paragraph,
                self.parameters
                    .textwrap_options(&mut self.line_ending, self.prefix.as_deref().unwrap_or("")),
            )
            .as_bytes(),
        )?;

        if eof {
            if self.trailing_newline {
                if self.line_ending == Some(LineEnding::CRLF) {
                    self.stdout.write_all(b"\r\n")?;
                } else {
                    self.stdout.write_all(b"\n")?;
                }
            }

            return Ok(());
        }

        self.trailing_newline = false;

        if self.line_ending == Some(LineEnding::CRLF) {
            self.stdout.write_all(b"\r\n")?;
            self.stdout
                .write_all(self.prefix.as_deref().unwrap_or("").trim_end().as_bytes())?;
            self.stdout.write_all(b"\r\n")?;
        } else {
            self.stdout.write_all(b"\n")?;
            self.stdout
                .write_all(self.prefix.as_deref().unwrap_or("").trim_end().as_bytes())?;
            self.stdout.write_all(b"\n")?;
        }

        self.current_paragraph.clear();

        Ok(())
    }
}
