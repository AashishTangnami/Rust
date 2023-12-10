fn one_hot_encode(text: &str) -> Vec<i32> {
    let mut one_hot: Vec<i32> = vec![0; 31];
    for c in text.chars() {
        let index = (c as u8 - 'a' as u8) as usize;
        if index < 31 {
            one_hot[index] = 1;
        }
    }
    one_hot
}

fn to_lowercase(text: &str) -> String {
    text.to_lowercase()
}

fn tokenize(text: &str) -> Vec<&str> {
    text.split_whitespace().collect()
}

pub fn main_code(){
    let text: &str = " I am learning Rust programming and I am doing NLP in rust";
    let lower_text: &str = &to_lowercase(text);
    let tokens: Vec<&str> = tokenize(lower_text);
    let mut text_one_hot: Vec<Vec<i32>> = Vec::new();

    for token in tokens {
        println!("{}", token);
        let one_hot: Vec<i32> = one_hot_encode(token);
        text_one_hot.push(one_hot);
    }
    println!("one hot encoding for given text{:#?}", text_one_hot);
}