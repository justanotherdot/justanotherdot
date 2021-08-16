use crate::context::{Blog, Post, Rss, Sitemap, Tag};
use ramhorns::Template;

pub struct PostTemplate<'a>(pub Template<'a>);

impl<'a> PostTemplate<'a> {
    pub fn render(&self, context: &Post) -> String {
        self.0.render(context)
    }
}

pub struct IndexTemplate<'a>(pub Template<'a>);

impl<'a> IndexTemplate<'a> {
    pub fn render(&self, context: &Blog) -> String {
        self.0.render(context)
    }
}

pub struct RssTemplate<'a>(pub Template<'a>);

impl<'a> RssTemplate<'a> {
    pub fn render(&self, context: &Rss) -> String {
        self.0.render(context)
    }
}

pub struct SitemapTemplate<'a>(pub Template<'a>);

impl<'a> SitemapTemplate<'a> {
    pub fn render(&self, context: &Sitemap) -> String {
        self.0.render(context)
    }
}

pub struct TagsTemplate<'a>(pub Template<'a>);

impl<'a> TagsTemplate<'a> {
    pub fn render(&self, context: &Tag) -> String {
        self.0.render(context)
    }
}

pub fn template(path: &str) -> Template {
    let source = std::fs::read_to_string(path).expect("could not read template");
    Template::new(source).unwrap()
}
