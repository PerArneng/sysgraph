use std::str::FromStr;

pub enum OutputType {
    GraphViz
}

impl FromStr for OutputType {
    type Err = ();

    fn from_str(s: &str) -> Result<OutputType, ()> {
        match s {
            "graphviz" => Ok(OutputType::GraphViz),
            _ => Err(()),
        }
    }
}

