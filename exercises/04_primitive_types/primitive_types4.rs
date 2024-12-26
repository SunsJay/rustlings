/*
 * @Date: 2024-12-23 21:54:33
 * @LastEditors: SunsJay SunsJay0806@gmail.com
 * @LastEditTime: 2024-12-23 22:44:36
 * @FilePath: /exam-grading/rustlings/exercises/04_primitive_types/primitive_types4.rs
 * @Description: 
 */
fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn slice_out_of_array() {
        let a = [1, 2, 3, 4, 5];

        // TODO: Get a slice called `nice_slice` out of the array `a` so that the test passes.
        // let nice_slice = ???

        let nice_slice = &a[1..4];

        assert_eq!([2, 3, 4], nice_slice);
    }
}
