use std::collections::HashMap;

pub type AttrMap = HashMap<String, String>;

#[derive(Debug)]
pub struct Node {
    // data for all nodes
    pub children: Vec<Node>,

    // type of the node
    pub node_type: NodeType,
}

#[derive(Debug)]
pub enum NodeType {
    Text(String),
    Element(ElementData),
}

#[derive(Debug)]
pub struct ElementData {
    pub tag_name: String,
    pub attributes: AttrMap,
}


pub fn text(data: String) -> Node {
    Node {
        children: Vec::new(),
        node_type: NodeType::Text(data),
    }
}

pub fn elem(children: Vec<Node>, tag: String, attr: AttrMap) -> Node {
    Node {
        children: children,
        node_type: NodeType::Element(ElementData {
            tag_name: tag,
            attributes: attr,
        }),
    }
}

impl Node {
    pub fn print(&self) {
        self.node_type.print();
        for i in &self.children {
            i.print();
        }
    }
}

impl NodeType {
    pub fn print(&self) {
        match *self {
            NodeType::Text(ref expr) => println!("{}", expr),
            NodeType::Element(ref expr) => println!("<{}>", expr.tag_name),
        }
    }
}
