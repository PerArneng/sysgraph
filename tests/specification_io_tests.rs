#![allow(unused_variables)]

extern crate sysgraph;

use sysgraph::io::component_diagram_parser::ComponentDiagramParser;
use sysgraph::io::component_diagram_xml_rs_parser::ComponentDiagramXmlRsParser;

#[test]
fn test_parse_empty_diagram() {

    let parser =
        ComponentDiagramXmlRsParser::new();

    let result = parser.parse_str("\
        <?xml version='1.0' encoding='UTF-8'?>\
        <components name='tes1'></components>\
    ");

    //assert_eq!(result.is_f, "test1");

}