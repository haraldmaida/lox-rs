use std::io;

pub struct RuntimeContext<'a> {
    stdout: Box<dyn io::Write + 'a>,
    stderr: Box<dyn io::Write + 'a>,
}

impl<'a> RuntimeContext<'a> {
    pub fn new(stdout: impl io::Write + 'a, stderr: impl io::Write + 'a) -> Self {
        Self {
            stdout: Box::new(stdout),
            stderr: Box::new(stderr),
        }
    }

    pub fn stdout(&mut self) -> &mut dyn io::Write {
        &mut *self.stdout
    }

    pub fn stderr(&mut self) -> &mut dyn io::Write {
        &mut *self.stderr
    }
}
