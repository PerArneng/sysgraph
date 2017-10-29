#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]


extern crate clap;

use std::fmt;
use self::clap::{Arg, App, SubCommand};
use sysgraph::output_type::OutputType;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug)]
pub struct Args {
    pub spec_file: PathBuf,
    pub output_file: PathBuf ,
    pub output_type: OutputType
}

impl Args {

    pub fn parse() -> Result<Args, String> {
        let matches = App::new("SysGraph")
                                .version("1.0")
                                .author("Per Arneng <per.arneng@anyplanet.com>")
                                .about("Converts a system specification xml file to a graph")
                                .arg(Arg::with_name("spec")
                                    .short("s")
                                    .long("spec")
                                    .value_name("FILE")
                                    .help("The specification xml file")
                                    .takes_value(true)
                                    .required(true))
                                .arg(Arg::with_name("output")
                                    .short("o")
                                    .long("output")
                                    .value_name("FILE")
                                    .help("The output graph file")
                                    .takes_value(true)
                                    .required(true))
                                .arg(Arg::with_name("type")
                                    .short("t")
                                    .long("type")
                                    .value_name("TYPE")
                                    .help("The type of the graph")
                                    .takes_value(true)
                                    .default_value("graphviz")
                                    .required(false))
                                .get_matches();

        let spec =
            matches.value_of("spec").ok_or("no spec given")?;

        let output_file =
            matches.value_of("output").ok_or("no output given")?;

        let output_type_str =
            matches.value_of("type").ok_or("no type given")?;

        let output_type= OutputType::from_str(output_type_str)?;

        let args = Args {
            spec_file: PathBuf::from(spec),
            output_file: PathBuf::from(output_file),
            output_type
        };

        return Result::Ok(args);
    }
}
