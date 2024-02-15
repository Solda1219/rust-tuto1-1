// use std::io;
// use std::cmp::Ordering;
// use rand::Rng;

fn main(){
    // //puzzle game
    // println!("Guess the number!");
    // let secret_number= rand::thread_rng().gen_range(1..=100);
    // loop {
    //     println!("Please input your guess.");
    //     let mut guess= String::new();
    //     io::stdin()
    //         .read_line(&mut guess)
    //         .expect("Failed to read line");
    //     let guess: u32 = match guess.trim().parse(){
    //         Ok(num)=>num,
    //         Err(_)=>continue,
    //     };
    //     println!("You guessed: {guess}");
    //     match guess.cmp(&secret_number){
    //         Ordering::Less=> println!("Too small!"),
    //         Ordering::Greater=>println!("Too big!"),
    //         Ordering::Equal=>{
    //             println!("You win!");
    //             break;
    //         }
    //     }
    //     //tuple study
    //     let tup= ((3, 4.5), 'w', 'd');
    //     println!("tup first first {}", tup.0.0);
    //     //array study
    //     let b= [6; 7];
    //     println!("array first {}", b[0]);
    // };
    // another function
    // another_function(5);
    // print_labeled_measurement(6, 'u');
    // let number= five();
    // println!("The value of x is : {number}");
    // if number % 4 == 0 {
    //     println!("number is divisible by 4");
    // } else if number % 3 == 0 {
    //     println!("number is divisible by 3");
    // } else if number % 2 == 0 {
    //     println!("number is divisible by 2");
    // } else {
    //     println!("number is not divisible by 4, 3, or 2");
    // }
    // let number= if number==5 {5} else {6};
    // println!("The value of new number is : {number}");
    // let mut xx= "sd";
    // if number==5{
    //     println!("xx= {xx}")
    // }else{
    //     xx= "sdfsd"
    // }
    // println!("xx= {xx}");
    
    //loop example
    // let mut count = 0;
    // 'counting_up: loop {
    //     println!("count = {count}");
    //     let mut remaining = 10;

    //     loop {
    //         println!("remaining = {remaining}");
    //         if remaining == 9 {
    //             break;
    //         }
    //         if count == 2 {
    //             break 'counting_up;
    //         }
    //         remaining -= 1;
    //     }

    //     count += 1;
    // }
    // println!("End count = {count}");
    // let mut number = 3;
    // while number!= 0{
    //     println!("{number}!");
    //     number-= 1;
    // }
    // println!("LIFTOFF!!!");
    // let a = [10, 20, 30, 40, 50];
    // let mut index = 0;
    // while index < 5 {
    //     println!("The value is : {}", a[index]);
    //     index+= 1;
    // }
    // for element in a {
    //     println!("the value is : {element}");
    // }
    // for number in (1..4).rev() {
    //     println!("{number}!");
    // }
    // println!("LIFTOFF!!!");
    let fahren= 45.3;
    let cels= fahren_to_cels(fahren);
    println!("Fahren {} is {} c", fahren, cels);
    let nth= 9;
    let fib_nth= nth_fibonacci(nth);
    println!("{nth} fibonacci is {fib_nth}");
    let my_string = String::from("hello world");
    let word= first_word(&my_string[0..6]);
    println!("word1 {}", word);
    let word= first_word(&my_string[..]);
    println!("word2 {}", word);
    let word= first_word(&my_string);
    println!("word3 {}", word);
    let my_string_literal= "hello world";
    let word= first_word(&my_string_literal[0..6]);
    println!("word4 {}", word);
    let word= first_word(&my_string_literal[..]);
    println!("word5 {}", word);
    let word= first_word(my_string_literal);
    println!("word6 {}", word);
    let a= [1, 2, 3, 4, 5];
    let slice= &a[1..3];
    assert_eq!(slice, &[2, 3]);

}
// fn another_function(x: i32){
//     println!("The value of x is : {x}");
// }
// fn print_labeled_measurement(value: i32, unit_label: char) {
//     println!("The measurement is: {value}{unit_label}");
// }

// fn five()->i32{
//     5
// }
fn fahren_to_cels(fahren: f64)->f64{
    let cels= (fahren-32.0)*5.0/9.0;
    cels
}
fn nth_fibonacci(n: i32)-> i32{
    let mut a= 0;
    let mut b= 1;
    let mut c;
    if n==0 {
        return a
    }
    for i in 2..n+1{
        c= a+b;
        a= b;
        b= c;
        println!("{i}")
    }
    b
}
fn first_word(s: &str)->&str {
    let bytes= s.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){
        if item== b' '{
            return &s[0..i];
        }
    }
    &s[..]
}