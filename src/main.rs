use std::collections::HashMap;

type AttrMap = HashMap<String, String>;

#[derive(Debug)]
struct Node {
    // data for all nodes
    children: Vec<Node>,

    // type of the node
    node_type: NodeType,
}

#[derive(Debug)]
enum NodeType {
    Text(String),
    Element(ElementData),
}

#[derive(Debug)]
struct ElementData {
    tag_name: String,
    attributes: AttrMap,
}


fn text(data: String) -> Node {
    Node { children: Vec::new(), node_type: NodeType::Text(data) }
}

fn elem(children: Vec<Node>, tag: String, attr: AttrMap) -> Node {
    Node {
        children: children,
        node_type: NodeType::Element(ElementData {
            tag_name: tag,
            attributes: attr
        })
    }
}

impl Node {
    fn print(&self) {
        self.node_type.print();
        for i in &self.children {
            i.print();
        }
    }
}

impl NodeType {
    fn print(&self) {
        match *self {
            NodeType::Text(ref expr) => println!("{}", expr),
            NodeType::Element(ref expr) => println!("<{}>", expr.tag_name),
        }
    }
}

fn main() {
    let mut root = elem(Vec::new(), "html".to_string(), AttrMap::new());
    let mut body = elem(Vec::new(), "body".to_string(), AttrMap::new());
    body.children.push(text("Hello World".to_string()));
    root.children.push(body);
    root.print();
}