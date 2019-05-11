extern crate mustache;

use mustache::compile_path;

fn main() {
    let content = compile_path("../site/templates/index.html").unwrap();
    println!("{:#?}", content);
}
