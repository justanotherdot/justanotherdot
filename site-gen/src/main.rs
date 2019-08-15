extern crate ramhorns;
extern crate serde;

use ramhorns::{Content, Template};
//use serde;

#[derive(Content)]
struct Post<'a> {
    title: &'a str,
    author: &'a str,
    date: &'a str,
    content: &'a str,
    url: &'a str,
    tags: Vec<Tag<'a>>,
}

#[derive(Content)]
struct Tag<'a> {
    url: &'a str,
    tag: &'a str,
}

#[derive(Content)]
struct Blog<'a> {
    title: &'a str,       // Strings are cool
    posts: Vec<Post<'a>>, // &'a [Post<'a>] would work too
    tags: Vec<Tag<'a>>,
}

fn main() {
    let source = std::fs::read_to_string("../site/templates/index.html").unwrap_or_else(|_| {
        eprintln!("could not read template");
        std::process::exit(1);
    });
    let tpl = Template::new(source).unwrap();
    let rendered = tpl.render(&Blog {
        title: "justanotherdot",
        posts: vec![Post {
            title: "A Fesh Start",
            author: "there",
            date: "2019-01-01T00:00:00+00:00",
            url: "name-of-markdown.html",
            content: "<p>lorem ipsum</p>",
            tags: vec![
                Tag {
                    url: "/tags/rust",
                    tag: "rust",
                },
                Tag {
                    url: "/tags/software",
                    tag: "software",
                },
            ],
        }],
        tags: vec![
            Tag {
                url: "/tags/rust.html",
                tag: "rust",
            },
            Tag {
                url: "/tags/software.html",
                tag: "software",
            },
        ],
    });
    println!("{}", rendered);
}
