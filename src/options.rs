use getopts;
use regex;
use regex::Regex;
use std;
use std::env::args;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug)]
pub enum OptionsError {
    ParseError(String),
    ParseOptionError { short: String, long: String, error: String, value: String },
    RegexError(regex::Error),
    ShowUsage(String),
}

impl Display for OptionsError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            &OptionsError::ParseError(ref err) => fmt.write_str(err),
            &OptionsError::ParseOptionError {
                ref short,
                ref long,
                ref error,
                ref value,
            } => write!(fmt, "couldn't parse -{}/--{} {:?}: {}", short, long, value, error),
            &OptionsError::RegexError(ref err) => err.fmt(fmt),
            &OptionsError::ShowUsage(ref usage) => fmt.write_str(usage),
        }
    }
}
impl From<regex::Error> for OptionsError {
    fn from(err: regex::Error) -> Self {
        OptionsError::RegexError(err)
    }
}

#[derive(Clone, Debug)]
pub struct Options {
    pub regexes: Vec<Regex>,
    pub quiet: bool,

    batch_size: Option<usize>,
    device: Option<usize>,
    group_size: Option<usize>,
    num_matches: Option<usize>,
    threads: Option<usize>,
}

impl Options {
    pub fn get() -> Result<Options, OptionsError> {
        Options::get_from(args())
    }
    pub fn get_from<I: Iterator<Item=String>>(mut args: I) -> Result<Options, OptionsError> {
        // Create the getopts::Options struct.
        let mut opts = getopts::Options::new();
        // Modes of operation.
        opts.optflag("h", "help", "Shows this help message");
        opts.optflag("l", "list", "Lists available OpenCL devices");
        // Options for default mode.
        opts.optopt("B", "batch_size", "Size of OpenCL problem space", "SIZE");
        opts.optopt("D", "device", "OpenCL device to use", "ID");
        opts.optopt("G", "group_size", "Size of OpenCL workgroup", "SIZE");
        opts.optopt("n", "num_matches", "Number of matches to find before exiting", "MATCHES");
        opts.optflag("q", "quiet", "Only output matches");
        opts.optopt("T", "threads", "Number of CPU threads to use", "THREADS");

        // Get the arguments.
        let name = args.next().unwrap_or("fistulosum".to_string());
        let args = try!(opts.parse(args).map_err(|e| OptionsError::ParseError(format!("{}", e))));

        // Check for the -h and -l flags, otherwise return the Options struct.
        if args.opt_present("h") {
            Err(OptionsError::ShowUsage(opts.usage(&opts.short_usage(&name))))
        } else if args.opt_present("l") {
            super::core::hash::list_devices();
        } else {
            let regexes = try!(args.free.iter()
                .map(|re| Regex::new(re))
                .collect());
            Ok(Options {
                regexes: regexes,
                quiet: args.opt_present("q"),

                batch_size:  try!(Options::parse_flag(&args, "B", "batch_size")),
                device:      try!(Options::parse_flag(&args, "D", "device")),
                group_size:  try!(Options::parse_flag(&args, "G", "group_size")),
                num_matches: try!(Options::parse_flag(&args, "n", "num_matches")),
                threads:     try!(Options::parse_flag(&args, "T", "threads")),
            })
        }
    }
    fn parse_flag<T: FromStr>(m: &getopts::Matches, short: &str, long: &str) -> Result<Option<T>, OptionsError>
          where <T as FromStr>::Err: Display {
        // Weird function signature (and consequently weird code) is so that we can use try!() on
        // the return value.
        match m.opt_str(short) {
            Some(value) => value.parse::<T>().map(Option::Some).map_err(|err| {
                OptionsError::ParseOptionError {
                    short: short.to_string(),
                    long: long.to_string(),
                    error: format!("{}", err),
                    value: value.to_string(),
                }
            }),
            None => Ok(None),
        }
    }
}
