mod basic_nlp;
use basic_nlp::main_code;

fn main() {

    main_code();

    let a: Vec<u64> = [0 ,30 ,45 , 60 ,90].to_vec();
    println!("a: {:#?}", a);

    let num1: i32 = 10;
    let num2: i32 = 20;

    let result: i32 = num1 + num2;
    let result2: i32 = num1 * num2 * 2;
    let result3: i32 = result + result2;

    println!("The value of result is: {}", result3);

   
}

