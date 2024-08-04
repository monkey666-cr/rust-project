use jieba_rs::Jieba;

fn main() {
    let mut jieba = Jieba::new();

    jieba.add_word("中出", None, None);

    let words = jieba.cut("我们中出了一个叛徒", false);

    println!("{:?}", words);

    let tokenize = jieba.tokenize("我们中出了一个叛徒", jieba_rs::TokenizeMode::Search, true);

    println!("{:?}", tokenize);
    
    let t = jieba.tag("我们", false);

    println!("{:?}", t);
}
