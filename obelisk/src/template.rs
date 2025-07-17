use crate::context::{Index, Post, Rss, Sitemap, Tag};
use handlebars::Handlebars;

pub struct PostTemplate {
    handlebars: Handlebars<'static>,
}

impl PostTemplate {
    pub fn new(template_path: &str) -> Self {
        let mut handlebars = Handlebars::new();
        let source = std::fs::read_to_string(template_path).expect("could not read template");
        handlebars
            .register_template_string("post", source)
            .expect("failed to register template");
        Self { handlebars }
    }

    pub fn render(&self, context: &Post) -> String {
        self.handlebars
            .render("post", context)
            .expect("failed to render template")
    }
}

pub struct IndexTemplate {
    handlebars: Handlebars<'static>,
}

impl IndexTemplate {
    pub fn new(template_path: &str) -> Self {
        let mut handlebars = Handlebars::new();
        let source = std::fs::read_to_string(template_path).expect("could not read template");
        handlebars
            .register_template_string("index", source)
            .expect("failed to register template");
        Self { handlebars }
    }

    pub fn render(&self, context: &Index) -> String {
        self.handlebars
            .render("index", context)
            .expect("failed to render template")
    }
}

pub struct RssTemplate {
    handlebars: Handlebars<'static>,
}

impl RssTemplate {
    pub fn new(template_path: &str) -> Self {
        let mut handlebars = Handlebars::new();
        let source = std::fs::read_to_string(template_path).expect("could not read template");
        handlebars
            .register_template_string("rss", source)
            .expect("failed to register template");
        Self { handlebars }
    }

    pub fn render(&self, context: &Rss) -> String {
        self.handlebars
            .render("rss", context)
            .expect("failed to render template")
    }
}

pub struct SitemapTemplate {
    handlebars: Handlebars<'static>,
}

impl SitemapTemplate {
    pub fn new(template_path: &str) -> Self {
        let mut handlebars = Handlebars::new();
        let source = std::fs::read_to_string(template_path).expect("could not read template");
        handlebars
            .register_template_string("sitemap", source)
            .expect("failed to register template");
        Self { handlebars }
    }

    pub fn render(&self, context: &Sitemap) -> String {
        self.handlebars
            .render("sitemap", context)
            .expect("failed to render template")
    }
}

pub struct TagsTemplate {
    handlebars: Handlebars<'static>,
}

impl TagsTemplate {
    pub fn new(template_path: &str) -> Self {
        let mut handlebars = Handlebars::new();
        // Set strict mode to false for compatibility with ramhorns behavior
        handlebars.set_strict_mode(false);
        let source = std::fs::read_to_string(template_path).expect("could not read template");
        handlebars
            .register_template_string("tags", source)
            .expect("failed to register template");
        Self { handlebars }
    }

    pub fn render(&self, context: &Tag) -> String {
        self.handlebars
            .render("tags", context)
            .expect("failed to render template")
    }
}