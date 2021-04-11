// モジュールーツリー
// crate
//  └── front_of_house
//      ├── hosting
//      │   ├── add_to_waitlist
//      │   └── seat_at_table
//      └── serving
//          ├── take_order
//          ├── serve_order
//          └── take_payment
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }
    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

// front_of_houseとeat_at_restaurantは兄弟的な感じ
// add_to_waitlistをcallしたい時
pub fn eat_at_restaurant() {
    // Absolute path : 絶対パス
    crate::front_of_house::hosting::add_to_waitlist();
    // Relative path : 相対パス
    front_of_house::hosting::add_to_waitlist();
}
