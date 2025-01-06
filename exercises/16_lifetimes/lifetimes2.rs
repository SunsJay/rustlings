/*
* @Date: 2024-12-23 21:54:33
* @LastEditors: SunsJay SunsJay0806@gmail.com
* @LastEditTime: 2025-01-06 22:14:32
* @FilePath: /rustlings/exercises/16_lifetimes/lifetimes2.rs

*/
// Don't change this function.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    // TODO: Fix the compiler error by moving one line.

    let string1 = String::from("long string is long");
    

    let string2 = String::from("xyz");
    let result = longest(&string1, &string2);

    println!("The longest string is '{result}'");
}
