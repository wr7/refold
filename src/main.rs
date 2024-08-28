use std::io::{BufRead, Write};

use cmdline::Parameters;
use textwrap::LineEnding;

mod cmdline;

fn main() {
    let parameters = Parameters::parse().unwrap_or_else(|err| {
        eprintln!("\x1b[1;31mrefold error:\x1b[m{}", err);
        std::process::exit(1)
    });

    let mut stdin = std::io::stdin().lock();
    let mut stdout = std::io::stdout().lock();

    let mut textwrap_options = textwrap::Options::new(parameters.width)
        .word_separator(parameters.split_mode.word_separator())
        .break_words(parameters.split_mode.break_words());

    let mut unwrapped_paragraph = String::new();

    let mut line = String::new();

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
                textwrap_options.line_ending = LineEnding::CRLF;
                line.pop();
            }
        }

        if !line.is_empty() {
            unwrapped_paragraph += &line;

            if !eof {
                unwrapped_paragraph += "\n";
            }
            continue;
        }

        stdout
            .write_all(textwrap::refill(&unwrapped_paragraph, &textwrap_options).as_bytes())
            .and_then(|_| {
                if eof {
                    Ok(())
                } else if textwrap_options.line_ending == LineEnding::CRLF {
                    stdout.write_all(b"\r\n")
                } else {
                    stdout.write_all(b"\n")
                }
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
