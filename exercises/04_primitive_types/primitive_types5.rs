/*
 * @Date: 2024-12-23 21:54:33
 * @LastEditors: SunsJay SunsJay0806@gmail.com
 * @LastEditTime: 2024-12-23 22:45:02
 * @FilePath: /exam-grading/rustlings/exercises/04_primitive_types/primitive_types5.rs
 * @Description: 
 */
fn main() {
    let cat = ("Furry McFurson", 3.5);

    // TODO: Destructure the `cat` tuple in one statement so that the println works.
    // let /* your pattern here */ = cat;

    let (name, age) = cat;

    println!("{name} is {age} years old");
}
