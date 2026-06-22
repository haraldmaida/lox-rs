use std::io;

pub struct RuntimeContext<'a> {
    stdout: &'a mut dyn io::Write,
    stderr: &'a mut dyn io::Write,
}

impl<'a> RuntimeContext<'a> {
    pub fn new(stdout: &'a mut impl io::Write, stderr: &'a mut impl io::Write) -> Self {
        Self { stdout, stderr }
    }

    pub fn stdout(&mut self) -> &mut dyn io::Write {
        &mut *self.stdout
    }

    pub fn stderr(&mut self) -> &mut dyn io::Write {
        &mut *self.stderr
    }
}
