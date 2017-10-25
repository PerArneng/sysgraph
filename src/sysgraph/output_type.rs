use std::str::FromStr;

#[derive(Debug)]
pub enum OutputType {
    GraphViz
}

impl FromStr for OutputType {
    type Err = String;

    fn from_str(s: &str) -> Result<OutputType, Self::Err> {
        match s {
            "graphviz" => Ok(OutputType::GraphViz),
            _ => Err(format!("unknown output type: {}", s)),
        }
    }
}

