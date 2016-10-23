pub mod dom;

fn main() {
    let mut root = dom::elem(Vec::new(), "html".to_string(), dom::AttrMap::new());
    let mut body = dom::elem(Vec::new(), "body".to_string(), dom::AttrMap::new());
    body.children.push(dom::text("Hello World".to_string()));
    root.children.push(body);
    root.print();
}