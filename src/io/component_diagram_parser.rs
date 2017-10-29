
use model::{ComponentDiagram};

pub trait ComponentDiagramParser {

    fn parse_str(&self, spec: &str) -> Result<ComponentDiagram, String> {
        return self.parse_bytes(spec.as_bytes());
    }

    fn parse_bytes(&self, spec: &[u8]) -> Result<ComponentDiagram, String>;

}