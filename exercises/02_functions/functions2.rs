/*
 * @Date: 2024-12-23 21:54:33
 * @LastEditors: SunsJay SunsJay0806@gmail.com
 * @LastEditTime: 2024-12-23 22:27:10
 * @FilePath: /exam-grading/rustlings/exercises/02_functions/functions2.rs
 * @Description: 
 */
// TODO: Add the missing type of the argument `num` after the colon `:`.
fn call_me(num: i32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

fn main() {
    call_me(3);
}
