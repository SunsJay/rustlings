/*
 * @Date: 2024-12-23 21:54:33
 * @LastEditors: SunsJay SunsJay0806@gmail.com
 * @LastEditTime: 2024-12-23 22:44:01
 * @FilePath: /exam-grading/rustlings/exercises/04_primitive_types/primitive_types3.rs
 * @Description: 
 */
fn main() {
    // TODO: Create an array called `a` with at least 100 elements in it.
    // let a = ???
    let a = [1; 100];
    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }
}
