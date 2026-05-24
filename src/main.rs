mod models;
use models::{Product,Sale,Purchase,Store};

fn main() {
    let mut my_store=Store::new();
    println!("rusty_store is opened!!!");

    match my_store.add_product(101,String::from("laptop"),String::from("high speed"), 70000.00, 100){
        Ok(_)=>println!("product added successfully"),
        Err(e)=>println!("{}",e),
    }

    match my_store.add_product(101,String::from("laptop2"),String::from("light weight"), 50000.00, 50){
        Ok(_)=>println!("product overwrote"),
        Err(e)=>println!("{}",e),
    }

    match my_store.edit_product(101, 75000.00, 150){
        Ok(_)=>println!("product is updated"),
        Err(e)=>println!("{}",e),
    }
    match my_store.edit_product(102, 75000.00, 150){
        Ok(_)=>println!("product is updated"),
        Err(e)=>println!("{}",e),
    }

    match my_store.del_product(101){
        Ok(_)=>println!("product got deleted"),
        Err(e)=>println!("{}",e),
    }
    match my_store.del_product(101){
        Ok(_)=>println!("product got deleted"),
        Err(e)=>println!("{}",e),
    }
}
