use std::io::{BufRead, Write};

use cmdline::Parameters;
use textwrap::LineEnding;

mod cmdline;

fn detect_prefix<'a>(prefix: &'_ mut Option<String>, line: &'a str) {
    if prefix.is_some() || line.is_empty() {
        return;
    }

    let mut prefix_len = 0;

    for (i, char) in line.char_indices() {
        if char.is_ascii_punctuation() | char.is_ascii_whitespace() {
            prefix_len = i + 1;
        } else {
            break;
        }
    }

    *prefix = Some(line[..prefix_len].to_owned());
}

impl Parameters {
    fn textwrap_options<'a>(
        &'_ self,
        line_ending: &'_ mut Option<LineEnding>,
        prefix: &'a str,
    ) -> textwrap::Options<'a> {
        let line_ending = if let Some(le) = *line_ending {
            le
        } else {
            *line_ending = Some(LineEnding::LF);
            LineEnding::LF
        };

        textwrap::Options::new(self.width)
            .word_separator(self.split_mode.word_separator())
            .break_words(self.split_mode.break_words())
            .line_ending(line_ending)
            .initial_indent(prefix)
            .subsequent_indent(prefix)
    }
}

fn main() {
    let parameters = Parameters::parse().unwrap_or_else(|err| {
        eprintln!("\x1b[1;31mrefold error:\x1b[m{}", err);
        std::process::exit(1)
    });

    let mut stdin = std::io::stdin().lock();
    let mut stdout = std::io::stdout().lock();

    let mut unwrapped_paragraph = String::new();

    let mut line = String::new();

    let mut prefix = parameters.prefix.clone();
    let mut line_ending = None;

    let mut trailing_newline = false;

    // TODO: refactor this horrible mess
    loop {
        line.clear();
        stdin.read_line(&mut line).unwrap_or_else(|err| {
            eprintln!("refold: failed to read from stdin: {err}");

            std::process::exit(err.raw_os_error().unwrap_or(1));
        });

        let mut eof = true;

        if line.ends_with('\n') {
            eof = false;
            line.pop();

            if line.ends_with('\r') {
                line_ending = Some(LineEnding::CRLF);
                line.pop();
            } else {
                line_ending = Some(LineEnding::LF);
            }
        }

        detect_prefix(&mut prefix, &line);

        let line = prefix
            .as_deref()
            .and_then(|prefix| line.strip_prefix(prefix))
            .unwrap_or(&line);

        if !line.is_empty() {
            unwrapped_paragraph += &line;

            if !eof {
                unwrapped_paragraph += " ";
            }

            trailing_newline = !eof;

            continue;
        }

        stdout
            .write_all(
                textwrap::fill(
                    &unwrapped_paragraph,
                    &parameters.textwrap_options(&mut line_ending, prefix.as_deref().unwrap_or("")),
                )
                .as_bytes(),
            )
            .and_then(|_| {
                if eof {
                    if trailing_newline {
                        if line_ending == Some(LineEnding::CRLF) {
                            stdout.write_all(b"\r\n")?;
                        } else {
                            stdout.write_all(b"\n")?;
                        }
                    }

                    return Ok(());
                }

                trailing_newline = false;

                if line_ending == Some(LineEnding::CRLF) {
                    stdout.write_all(b"\r\n")?;
                    stdout.write_all(prefix.as_deref().unwrap_or("").trim_end().as_bytes())?;
                    stdout.write_all(b"\r\n")?;
                } else {
                    stdout.write_all(b"\n")?;
                    stdout.write_all(prefix.as_deref().unwrap_or("").trim_end().as_bytes())?;
                    stdout.write_all(b"\n")?;
                }

                Ok(())
            })
            .unwrap_or_else(|err| {
                eprintln!("refold: failed to write to stdout: {err}");

                std::process::exit(err.raw_os_error().unwrap_or(1));
            });

        unwrapped_paragraph.clear();

        if eof {
            break;
        }
    }
}
