use std::borrow::Cow;

use argtea::{argtea_impl, simple_format};
use textwrap::WordSeparator;

#[derive(Debug)]
pub enum SplitMode {
    Spaces,
    Boundaries,
    Characters,
}

#[derive(Debug)]
pub struct Parameters {
    pub split_mode: SplitMode,
    pub width: usize,
}

argtea_impl! {
    {
        /// Prints this help message.
        ("--help" | "-h") => {
            eprint!("{}", Self::HELP);

            std::process::exit(1);
        }

        /// Sets the width to wrap at (default 80).
        ("--width" | "-w", width) => {
            let width = width.ok_or("expected width")?;

            width_ = width
                .parse()
                .map_err(|_| format!("invalid width `\x1b[1m{width}`\x1b[m"))?;
        }

        /// Sets the split mode to "boundaries" mode (default).
        ///
        /// In character mode, line wrapping may occur in-between unicode breakable
        /// characters.
        ("--boundaries" | "-b" | "--unicode-boundaries") => {
            split_mode = SplitMode::Boundaries;
        }

        /// Sets the split mode to "space" mode.
        ///
        /// In space mode, line wrapping may occur in-between words separated by ASCII
        /// spaces.
        ("--spaces" | "-s") => {
            split_mode = SplitMode::Spaces;
        }

        /// Sets the split mode to "character" mode.
        ///
        /// In character mode, line wrapping may occur in-between any two characters.
        ("--characters" | "-c" | "--break-words" | "--break") => {
            split_mode = SplitMode::Characters;
        }

        #[hidden]
        (invalid_flag) => {
            return Err(format!("invalid flag `\x1b[1m{invalid_flag}\x1b[m`").into());
        }
    }
    impl Parameters {
        const HELP: &'static str = simple_format!(
            "refold: rewraps line of text"
            ""
            "Usage: refold [FLAGS...]"
            ""
            "refold reads from stdin and writes to stdout"
            ""
            "Options:"
            docs!()
        );

        fn parse() -> Result<Self, Cow<'static, str>> {
            let mut split_mode = SplitMode::Boundaries;
            let mut width_ = 80;

            let mut args = std::env::args().skip(1);

            parse!(args);

            return Ok(Self { split_mode, width: width_ });
        }
    }
}

impl SplitMode {
    pub fn break_words(&self) -> bool {
        match self {
            SplitMode::Spaces => false,
            SplitMode::Boundaries => false,
            SplitMode::Characters => true,
        }
    }
    pub fn word_separator(&self) -> WordSeparator {
        match self {
            SplitMode::Spaces => WordSeparator::AsciiSpace,
            SplitMode::Boundaries => WordSeparator::UnicodeBreakProperties,
            SplitMode::Characters => WordSeparator::UnicodeBreakProperties,
        }
    }
}