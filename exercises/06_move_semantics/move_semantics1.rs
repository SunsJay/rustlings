/*
 * @Date: 2024-12-23 21:54:33
 * @LastEditors: SunsJay SunsJay0806@gmail.com
 * @LastEditTime: 2024-12-26 20:54:27
 * @FilePath: /exam-grading/rustlings/exercises/06_move_semantics/move_semantics1.rs
 * @Description: 
 */
// TODO: Fix the compiler error in this function.
fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(88);

    vec
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics1() {
        let vec0 = vec![22, 44, 66];
        let vec1 = fill_vec(vec0);
        assert_eq!(vec1, vec![22, 44, 66, 88]);
    }
}
