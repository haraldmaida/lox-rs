use miette::NamedSource;
use std::fmt::Display;
use std::io;

pub struct RuntimeContext<'a> {
    stdout: &'a mut dyn io::Write,
    stderr: &'a mut dyn io::Write,
    fancy_error_messages: bool,
    filename: Option<&'a str>,
    source: Option<&'a str>,
}

impl<'a> RuntimeContext<'a> {
    pub fn new(stdout: &'a mut impl io::Write, stderr: &'a mut impl io::Write) -> Self {
        Self {
            stdout,
            stderr,
            fancy_error_messages: false,
            filename: None,
            source: None,
        }
    }

    #[must_use]
    pub const fn with_fancy_error_messages(
        mut self,
        filename: Option<&'a str>,
        source: &'a str,
    ) -> Self {
        self.fancy_error_messages = true;
        self.filename = filename;
        self.source = Some(source);
        self
    }

    pub fn stdout(&mut self) -> &mut dyn io::Write {
        &mut *self.stdout
    }

    pub fn stderr(&mut self) -> &mut dyn io::Write {
        &mut *self.stderr
    }

    pub fn write_error(&mut self, error: impl Into<miette::Report>) {
        let error = error.into();
        let report = match (self.source, self.filename) {
            (None, _) => error,
            (Some(source), None) => error.with_source_code(source.to_owned()),
            (Some(source), Some(filename)) => {
                error.with_source_code(NamedSource::new(filename, source.to_owned()))
            },
        };
        if self.fancy_error_messages {
            writeln!(self.stderr(), "{report:?}")
                .unwrap_or_else(|io_err| panic!("failed to write to stderr: {io_err:?}"));
        } else {
            writeln!(self.stderr(), "{report}")
                .unwrap_or_else(|io_err| panic!("failed to write to stderr: {io_err}"));
        }
    }

    pub fn write_output(&mut self, message: impl Display) {
        writeln!(self.stdout(), "{message}")
            .unwrap_or_else(|io_err| panic!("failed to write to stdout: {io_err}"));
    }
}
