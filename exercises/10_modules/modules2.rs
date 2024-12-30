// You can bring module paths into scopes and provide new names for them with
// the `use` and `as` keywords.

pub mod delicious_snacks {
    // TODO：修复后添加以下两个“use”语句。
    pub use self::fruits::PEAR as fruit; 
    pub use self::veggies::CUCUMBER as veggie;

    pub mod fruits {
        pub const PEAR: &str = "Pear";
        pub const APPLE: &str = "Apple";
    }

    pub mod veggies {
        pub const CUCUMBER: &str = "Cucumber";
        pub const CARROT: &str = "Carrot";
    }
}

fn main() {
    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::fruit,
        delicious_snacks::veggie,
    );
}
