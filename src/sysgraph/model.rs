

#[derive(Debug)]
pub struct Interface<'a> {
    pub id: String,
    pub protocol: String,
    pub parent: &'a Component<'a>
}

#[derive(Debug)]
pub struct Component<'a> {
    pub name: String,
    pub id: String,
    pub interfaces: Vec<Interface<'a>>,
    pub dependencies: Vec<String>
}

impl<'a>  Component<'a>  {

    /*
    pub fn new() -> Component<'a> {
        Component {
            name: ""
        };
    }
    */
}