pub struct NewsArticle {
    pub headline: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{} {} {}", self.headline, self.author, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("{}", self.summarize_author())
    }
}
