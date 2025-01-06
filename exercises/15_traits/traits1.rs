/*
 * @Date: 2024-12-23 21:54:33
 * @LastEditors: SunsJay SunsJay0806@gmail.com
 * @LastEditTime: 2025-01-05 21:02:26
 * @FilePath: /rustlings/exercises/15_traits/traits1.rs
 * @Description: 
 */
// The trait `AppendBar` has only one function which appends "Bar" to any object
// implementing this trait.
trait AppendBar {
    fn append_bar(self) -> Self;
}

impl AppendBar for String {
    // TODO: Implement `AppendBar` for the type `String`.
    fn append_bar(self) -> Self {
        format!("{}Bar", self)
    }
    
}

fn main() {
    let s = String::from("Foo");
    let s = s.append_bar();
    println!("s: {s}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_foo_bar() {
        assert_eq!(String::from("Foo").append_bar(), "FooBar");
    }

    #[test]
    fn is_bar_bar() {
        assert_eq!(String::from("").append_bar().append_bar(), "BarBar");
    }
}
