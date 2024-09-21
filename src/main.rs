use cmdline::Parameters;
use textwrap::LineEnding;

mod cmdline;
mod refold;
pub(crate) mod util;

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

    let refolder = refold::Refolder::new(&parameters);
    refolder.refold();
}
