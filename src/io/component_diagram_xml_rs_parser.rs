


use xml::reader::{EventReader, XmlEvent};

use model::{ComponentDiagram};
use io::component_diagram_parser::{ComponentDiagramParser};


pub struct ComponentDiagramXmlRsParser {

}

impl ComponentDiagramXmlRsParser {
    pub fn new() -> ComponentDiagramXmlRsParser {
        return ComponentDiagramXmlRsParser {};
    }
/*
    fn parse_component_diagram<I>(&self, reader: I) -> Result<ComponentDiagram, String>
        where I: Iterator<Item=Result<XmlEvent>>  {

        //reader.next();

    }*/
}

impl ComponentDiagramParser for ComponentDiagramXmlRsParser {

    fn parse_bytes(&self, spec: &[u8]) -> Result<ComponentDiagram, String> {
        let parser = EventReader::new(spec);

        //self.hi(&(parser.into_iter()));

        let mut components_name: &str;

        for e in parser {
            match e {
                Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                    println!("{} {:?}", name, attributes);
                }
                Ok(XmlEvent::EndElement { name }) => {
                    println!("{}", name);
                }
                Err(e) => {
                    println!("Error: {}", e);
                    break;
                }
                _ => {}
            }
        }

        return Ok(ComponentDiagram::new("hello"));
    }

}

