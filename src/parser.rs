use dom;

use std::collections::HashMap;

// Parse an HTML document and return the root element
pub fn parse(source: String) -> dom::Node {
    let mut nodes = Parser {pos: 0, input: source }.parse_nodes();

    if nodes.len() == 1 {
        nodes.swap_remove(0)
    } else {
        dom::elem(nodes, "html".to_string(), HashMap::new())
    }
}

#[derive(Debug)]
struct Parser {
    pos: usize,
    input: String,
}

impl Parser {
    // Read the current character without consuming it
    fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    // Check if next characters starts with given string
    fn starts_with(&self, s: &str) -> bool {
        self.input[self.pos ..].starts_with(s)
    }

    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    // Return the current char and advances self.pos to the next char
    fn consume_char(&mut self) -> char {
        let mut iter = self.input[self.pos..].char_indices();
        let (_, cur_char) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        self.pos += next_pos;
        cur_char
    }

    // consume chars untill test returns false
    fn consume_while<F>(&mut self, test: F) -> String
            where F: Fn(char) -> bool {
        let mut result = String::new();
        while !self.eof() && test(self.next_char()) {
            result.push(self.consume_char());
        }
        result
    }

    //Ignore whitespace
    fn consume_whitespace(&mut self) {
        self.consume_while(char::is_whitespace);
    }

    //Parse a tag or attribute name 
    fn parse_tag_name(&mut self) -> String {
        self.consume_while(|c| match c {
            'a'...'z' | 'A'...'Z' | '0'...'9' => true,
            _ => false
        })
    }

    //Parse single node
    fn parse_node(&mut self) -> dom::Node {
        match self.next_char() {
            '<' => self.parse_element(),
            _   => self.parse_text()
        }
    }

    //Parse a text node 
    fn parse_text(&mut self) -> dom::Node {
        dom::text(self.consume_while(|c| c != '<'))
    }

    fn parse_element(&mut self) -> dom::Node {
        // Opening tag
        assert!(self.consume_char() == '<');
        let tag_name = self.parse_tag_name();
        let attrs = self.parse_attributes();
        assert!(self.consume_char() == '>');

        // Contents
        let children = self.parse_nodes();

        // Closing tag.
        assert!(self.consume_char() == '<');
        assert!(self.consume_char() == '/');
        assert!(self.parse_tag_name() == tag_name);
        assert!(self.consume_char() == '>');

        return dom::elem(children, tag_name, attrs)
    }

    // Parse a single attribute name=value pair
    fn parse_attr(&mut self) -> (String, String) {
        let name = self.parse_tag_name();
        assert!(self.consume_char() == '=');
        let value = self.parse_attr_value();
        return (name, value);
    }

    // Parse a qouted value
    fn parse_attr_value(&mut self) -> String {
        let open_qoute = self.consume_char();
        assert!(open_qoute == '"' || open_qoute == '\'');
        let value = self.consume_while(|c| c != open_qoute);
        assert!(self.consume_char() == open_qoute);
        value
    }

    // Parse a list of attributes pair separated by whitespace
    fn parse_attributes(&mut self) -> dom::AttrMap {
        let mut attributes  = HashMap::new();
        loop {
            self.consume_whitespace();
            if self.next_char() == '>' {
                break;
            }
            let (name, value) = self.parse_attr();
            attributes.insert(name, value);
        }
        attributes
    }

    //Parse a sequence of sibling parse_nodes
    fn parse_nodes(&mut self) -> Vec<dom::Node> {
        let mut nodes = Vec::new();
        loop {
            self.consume_whitespace();
            if self.eof() || self.starts_with("</") {
                break;
            }

            nodes.push(self.parse_node());
        }

        nodes
    }
}
