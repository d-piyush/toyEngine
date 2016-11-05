pub mod dom;
pub mod parser;

fn main() {
    let html: String = "<html>
                            <body>
                                <h1>Title</h1>
                                <div id='main' class='test'>
                                    <p>Hello <em>world</em>!</p>
                                </div>
                            </body>
                        </html>".to_string();

    let root = parser::parse(html);
    root.print();
}