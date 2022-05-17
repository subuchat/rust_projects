
pub trait Summery {
    fn summerize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summery for NewsArticle {
    fn summerize(&self) -> String{
        format!("{} , by {}: location ({})" , self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summery for Tweet{
    fn summerize(&self) -> String {
        format!("{} : {}",self.username, self.content)
    }
}

//trait as paramter

pub fn notify(item : &impl Summery){
    println!("Breaking news : {}", item.summerize());
}

fn main() {
    println!("Hello, world!");
}
