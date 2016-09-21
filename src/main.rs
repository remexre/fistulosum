extern crate fistulosum;

use fistulosum::*;
use std::io::prelude::*;

fn main() {
    let opts = Options::get().unwrap_or_else(|err| ExitStatus::from(err).exit());
    println!("Got options: {:?}", opts);
    unimplemented!();
}

pub enum ExitStatus {
    Success,
    OptionsError(OptionsError),
}

impl ExitStatus {
    pub fn exit(&self) -> ! {
        self.err().map(|err| writeln!(std::io::stderr(), "{}", err));
        std::process::exit(self.code());
    }
    fn code(&self) -> i32 {
        match self {
            &ExitStatus::Success => 0,
            &ExitStatus::OptionsError(_) => 1,
        }
    }
    fn err(&self) -> Option<String> {
        match self {
            &ExitStatus::Success => None,
            &ExitStatus::OptionsError(ref err) => Some(format!("{}", err)),
        }
    }
}
impl From<OptionsError> for ExitStatus {
    fn from(err: OptionsError) -> Self {
        ExitStatus::OptionsError(err)
    }
}
