pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct Post {
    pub title: String,
    pub author: String,
    pub content: String,
}


// 为Post实现Summary特征
impl Summary for Post {
    fn summarize(&self) -> String {
        format!("{} by {} : {}", self.title, self.author, self.content)
    }
}


pub struct Weibo {
    pub username: String,
    pub content: String
}

impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("{}发表了微博{}", self.username, self.content)
    }
}

fn main() {

}