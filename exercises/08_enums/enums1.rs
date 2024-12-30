/*
 * @Date: 2024-12-23 21:54:33
 * @LastEditors: SunsJay SunsJay0806@gmail.com
 * @LastEditTime: 2024-12-28 21:53:03
 * @FilePath: /exam-grading/rustlings/exercises/08_enums/enums1.rs
 * @Description:    
 */
#[derive(Debug)]
enum Message {
    // TODO: Define a few types of messages as used below.
    Resize, Move, Echo, ChangeColor, Quit
}

fn main() {
    println!("{:?}", Message::Resize);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::ChangeColor);
    println!("{:?}", Message::Quit);
}
