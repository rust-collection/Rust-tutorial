use traitdemo::Article;
use traitdemo::Summary;

fn main() {
    let a = Article{
        username:String::from("aaa"),
        content:String::from("asfasfasf"),
    };
    println!("{}", a.summarize());
}
