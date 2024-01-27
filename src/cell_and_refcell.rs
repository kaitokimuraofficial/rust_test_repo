use std::cell::{Cell, RefCell};
// use std::cell::RefCell;

pub fn use_cell_and_refcell() {
    // use_cell();
    // use_refcell();
    prac1();
}

fn prac1() {
    let a: i32 = 5;
    println!("aのアドレス: {:p}", &a);
    let c = Cell::new(a);
    println!("cのアドレス: {:p}", &c);
    let ptr = c.as_ptr();
    println!("cの中身のアドレス: {:p}", ptr);
    // println!("{}", *ptr)  これはコンパイラ的にはアウト

    println!("-------------");

    let mut c2 = Cell::new(5);
    println!("c2のアドレス: {:p}", &c2);
    let ptr2 = c2.as_ptr();
    let ptr2_getmut = c2.get_mut();
    println!("c23の中身のアドレス: {:p}", ptr2_getmut);
    *ptr2_getmut += 1;

    // However be cautious: this method expects self to be mutable,
    // which is generally not the case when using a Cell

    // If you require interior mutability by reference,
    // consider using RefCell which provides run-time
    // checked mutable borrows through its borrow_mut method.

    println!("-------------");

    let r = RefCell::new(5);
    let borrowed_r = r.borrow();
    println!("rのアドレス: {}", borrowed_r);
}

// struct SomeStruct2 {
//     regular_field: u8,
//     special_field: RefCell<u8>,
// }

// RefCell
// A mutable memory location with dynamically checked borrow rules

// fn use_refcell() {
//     let my_struct = SomeStruct2 {
//         regular_field: 0,
//         special_field: RefCell::new(1),
//     };

//     let new_value: u8 = 100;

//     // 参照
//     let borrowed_field = my_struct.special_field.borrow();

// }

// struct SomeStruct {
//     regular_field: u8,
//     special_field: Cell<u8>,
// }

// fn use_cell() {
//     let my_struct = SomeStruct {
//         regular_field: 0,
//         special_field: Cell::new(1),
//     };

//     let new_value = 100;

//     // my_struct.regular_field = new_value; これはエラーが発生する
//     println!("mystructのアドレス: {:p}", &my_struct);
//     println!("regular_fieldのアドレス: {:p}", &my_struct.regular_field);
//     println!("special_fieldのアドレス: {:p}", &my_struct.special_field);
//     println!(
//         "special_fieldのアドレス: {:p}",
//         my_struct.special_field.as_ptr()
//     );
//     let p = Cell::new(2);
//     println!("pのアドレス: {:p}", p.as_ptr());
//     my_struct.special_field.set(new_value);
//     println!(
//         "special_fieldのアドレス: {:p}",
//         my_struct.special_field.as_ptr()
//     );
//     assert_eq!(my_struct.special_field.get(), new_value);
// }
