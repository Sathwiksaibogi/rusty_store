mod models;
use models::{Product,Sale,Purchase,Store};

fn main() {
    let mut my_store=Store::new();
    println!("rusty_store is opened!!!");

    // add products
    match my_store.add_product(101,String::from("laptop"),String::from("high speed"), 70000.00,90000.00, 100){
        Ok(_)=>println!("product added successfully"),
        Err(e)=>println!("{}",e),
    }

    match my_store.add_product(105,String::from("laptop3"),String::from("high speed +++"), 70000.00,90000.00, 100){
        Ok(_)=>println!("product added successfully"),
        Err(e)=>println!("{}",e),
    }

    match my_store.add_product(101,String::from("laptop2"),String::from("light weight"), 50000.00,70000.00, 50){
        Ok(_)=>println!("product overwrote"),
        Err(e)=>println!("{}",e),
    }

    // edit products
    match my_store.edit_product(101, 75000.00, 150){
        Ok(_)=>println!("product is updated"),
        Err(e)=>println!("{}",e),
    }
    match my_store.edit_product(102, 75000.00, 150){
        Ok(_)=>println!("product is updated"),
        Err(e)=>println!("{}",e),
    }

    // delete products
    // match my_store.del_product(101){
    //     Ok(_)=>println!("product got deleted"),
    //     Err(e)=>println!("{}",e),
    // }
    // match my_store.del_product(101){
    //     Ok(_)=>println!("product got deleted"),
    //     Err(e)=>println!("{}",e),
    // }

    // sell products
    match my_store.record_sale(1,101,2){
        Ok(_)=>println!("product sold"),
        Err(e)=>println!("{}",e),
    }
    match my_store.record_sale(1,101,200){
        Ok(_)=>println!("product sold"),
        Err(e)=>println!("{}",e),
    }

    // purchase products
    match my_store.record_purchase(1, 101, 100, 80000.00){
        Ok(_)=>println!("product purchased"),
        Err(e)=>println!("{}",e),
    }
    match my_store.record_purchase(1, 1010, 100, 80000.00){
        Ok(_)=>print!("product purchased"),
        Err(e)=>println!("{}",e),
    }
    // print inventory report
    my_store.print_inventory_report();

    // print sales report
    my_store.print_sales_report();

    // print purchase report
    my_store.print_purchase_report();

}
