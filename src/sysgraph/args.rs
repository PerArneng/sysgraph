#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]


extern crate clap;

use self::clap::{Arg, App, SubCommand};
use sysgraph::output_type::OutputType;

pub struct Args<'a> {
    xml_file: &'a str,
    output_file: &'a str,
    output_type: OutputType
}

impl<'a> Args<'a> {

    pub fn parse() -> Result<Args<'a>, ()> {
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

        let args = Args {
            xml_file:"x",
            output_file:"ss",
            output_type: OutputType::GraphViz
        };

        return Result::Ok(args);
    }
}
