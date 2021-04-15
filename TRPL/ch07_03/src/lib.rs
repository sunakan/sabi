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
// これはこのまま

// この定義をfront_of_houseに別ファイルに切り出して更に、hostingも別ファイルにする
// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//         fn seat_at_table() {}
//     }
//     mod serving {
//         fn take_order() {}
//         fn serve_order() {}
//         fn take_payment() {}
//     }
// }

// これで、src/front_of_house.rsを読み込む
mod front_of_house;

// front_of_house::hosting::add_to_waitlist()とするのが面倒
// hosting::add_to_waitlist()とcallとできるように外部公開する
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
