mod models;
mod auth;

use models::{Product,Sale,Purchase,Store};
use auth::login;

use std::io::{self,Write};

fn main() {

    if !login(){
        println!("Invalid credentials! Access denied.");
        return;
    }
    println!("Access Granted!");

    let mut my_store=Store::new();
    println!("rusty_store is opened!!!");

    loop{
        println!("------MAIN MENU-----");
        println!("1.ADD PRODUCT");
        println!("2.EDIT PRODUCT");
        println!("3.DELETE PRODUCT");
        println!("4.RECORD SALE");
        println!("5.RECORD PURCHASE");
        println!("6.PRINT INVENTORY REPORT");
        println!("7.PRINT SALES REPORT");
        println!("8.PRINT PURCHASE REPORT");
        println!("9.EXIT STORE");

        println!("enter a choice from 1-9");
        io::stdout().flush().unwrap();
        let mut choice=String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1"=>{
                println!("1.ADD PRODUCT");

                println!("enter product id : ");
                io::stdout().flush().unwrap();
                let mut id_str=String::new();
                io::stdin().read_line(&mut id_str).unwrap();
                let id=match id_str.trim().parse::<u32>(){
                    Ok(num)=>num,
                    Err(_)=>{
                        println!("Invalid input");
                        continue;
                    }
                };

                println!("enter product name : ");
                io::stdout().flush().unwrap();
                let mut name_str=String::new();
                io::stdin().read_line(&mut name_str).unwrap();
                let name=name_str.trim().to_string();

                println!("enter product description : ");
                io::stdout().flush().unwrap();
                let mut description_str=String::new();
                io::stdin().read_line(&mut description_str).unwrap();
                let description=description_str.trim().to_string();

                println!("enter product cost_price : ");
                io::stdout().flush().unwrap();
                let mut cost_price_str=String::new();
                io::stdin().read_line(&mut cost_price_str).unwrap();
                let cost_price=match cost_price_str.trim().parse::<f64>(){
                    Ok(num)=>num,
                    Err(_)=>{
                        println!("Invalid input");
                        continue;
                    }
                };

                println!("enter product sell_price : ");
                io::stdout().flush().unwrap();
                let mut sell_price_str=String::new();
                io::stdin().read_line(&mut sell_price_str).unwrap();
                let sell_price=match sell_price_str.trim().parse::<f64>(){
                    Ok(num)=>num,
                    Err(_)=>{
                        println!("Invalid input");
                        continue;
                    }
                };

                println!("enter product quantity : ");
                io::stdout().flush().unwrap();
                let mut quantity_str=String::new();
                io::stdin().read_line(&mut quantity_str).unwrap();
                let quantity=match quantity_str.trim().parse::<u32>(){
                    Ok(num)=>num,
                    Err(_)=>{
                        println!("Invalid input");
                        continue;
                    }
                };

                match my_store.add_product(id, name, description, cost_price, sell_price, quantity){
                    Ok(_)=>println!("product added successfully"),
                    Err(e)=>println!("{}",e),
                }
            }

            "2"=>{
                println!("2.EDIT PRODUCT");

                println!("enter product id : ");
                io::stdout().flush().unwrap();
                let mut id_str=String::new();
                io::stdin().read_line(&mut id_str).unwrap();
                let id=match id_str.trim().parse::<u32>(){
                    Ok(num)=>num,
                    Err(_)=>{
                        println!("Invalid input");
                        continue;
                    }
                };

                println!("enter product new_sell_price : ");
                io::stdout().flush().unwrap();
                let mut sell_price_str=String::new();
                io::stdin().read_line(&mut sell_price_str).unwrap();
                let new_price=match sell_price_str.trim().parse::<f64>(){
                    Ok(num)=>num,
                    Err(_)=>{
                        println!("Invalid input");
                        continue;
                    }
                };

                println!("enter product quantity : ");
                io::stdout().flush().unwrap();
                let mut quantity_str=String::new();
                io::stdin().read_line(&mut quantity_str).unwrap();
                let new_quantity=match quantity_str.trim().parse::<u32>(){
                    Ok(num)=>num,
                    Err(_)=>{
                        println!("Invalid input");
                        continue;
                    }
                };

                match my_store.edit_product(id, new_price, new_quantity){
                    Ok(_)=>println!("product is updated"),
                    Err(e)=>println!("{}",e),
                }
            }

            "3"=>{
                println!("3.DELETE PRODUCT");

                println!("enter product id : ");
                io::stdout().flush().unwrap();
                let mut id_str=String::new();
                io::stdin().read_line(&mut id_str).unwrap();
                let id=match id_str.trim().parse::<u32>(){
                    Ok(num)=>num,
                    Err(_)=>{
                        println!("Invalid input");
                        continue;
                    }
                };
                match my_store.del_product(id){
                    Ok(_)=>println!("product got deleted"),
                    Err(e)=>println!("{}",e),
                }
            }

            "4"=>{
                println!("4.RECORD SALE");

                println!("enter sale id : ");
                io::stdout().flush().unwrap();
                let mut sale_id_str=String::new();
                io::stdin().read_line(&mut sale_id_str).unwrap();
                let sale_id=match sale_id_str.trim().parse::<u32>(){
                    Ok(num)=>num,
                    Err(_)=>{
                        println!("Invalid input");
                        continue;
                    }
                };

                println!("enter product id : ");
                io::stdout().flush().unwrap();
                let mut id_str=String::new();
                io::stdin().read_line(&mut id_str).unwrap();
                let id=match id_str.trim().parse::<u32>(){
                    Ok(num)=>num,
                    Err(_)=>{
                        println!("Invalid input");
                        continue;
                    }
                };

                 println!("enter product quantity : ");
                io::stdout().flush().unwrap();
                let mut quantity_str=String::new();
                io::stdin().read_line(&mut quantity_str).unwrap();
                let new_quantity=match quantity_str.trim().parse::<u32>(){
                    Ok(num)=>num,
                    Err(_)=>{
                        println!("Invalid input");
                        continue;
                    }
                };

                match my_store.record_sale(sale_id, id, new_quantity) {
                    Ok(_)=>println!("product sold"),
                    Err(e)=>println!("{}",e),
                }
            }

            "5"=>{
                println!("5.RECORD PURCHASE");

                println!("enter purchase id : ");
                io::stdout().flush().unwrap();
                let mut purchase_id_str=String::new();
                io::stdin().read_line(&mut purchase_id_str).unwrap();
                let purchase_id=match purchase_id_str.trim().parse::<u32>(){
                    Ok(num)=>num,
                    Err(_)=>{
                        println!("Invalid input");
                        continue;
                    }
                };

                println!("enter product id : ");
                io::stdout().flush().unwrap();
                let mut id_str=String::new();
                io::stdin().read_line(&mut id_str).unwrap();
                let id=match id_str.trim().parse::<u32>(){
                    Ok(num)=>num,
                    Err(_)=>{
                        println!("Invalid input");
                        continue;
                    }
                };

                println!("enter product quantity : ");
                io::stdout().flush().unwrap();
                let mut quantity_str=String::new();
                io::stdin().read_line(&mut quantity_str).unwrap();
                let new_quantity=match quantity_str.trim().parse::<u32>(){
                    Ok(num)=>num,
                    Err(_)=>{
                        println!("Invalid input");
                        continue;
                    }
                };

                println!("enter product new_cost_price : ");
                io::stdout().flush().unwrap();
                let mut cost_price_str=String::new();
                io::stdin().read_line(&mut cost_price_str).unwrap();
                let cost_price=match cost_price_str.trim().parse::<f64>(){
                    Ok(num)=>num,
                    Err(_)=>{
                        println!("Invalid input");
                        continue;
                    }
                };
                match my_store.record_purchase(purchase_id, id, new_quantity, cost_price) {
                    Ok(_)=>println!("product purchased"),
                    Err(e)=>println!("{}",e),
                }
                
            }

            "6"=>{
                println!("6.PRINT INVENTORY REPORT");
                my_store.print_inventory_report();
            }

            "7"=>{
                println!("7.PRINT SALES REPORT");
                my_store.print_sales_report();
            }

            "8"=>{
                println!("8.PRINT PURCHASE REPORT");
                my_store.print_purchase_report();
            }

            "9"=>{
                println!("Good Bye!!!");
                break;
            }

            _=>println!("Invalid input. Try again. "),
            

             
        
        
        }
    }


    // // add products
    // match my_store.add_product(101,String::from("laptop"),String::from("high speed"), 70000.00,90000.00, 100){
    //     Ok(_)=>println!("product added successfully"),
    //     Err(e)=>println!("{}",e),
    // }

    // match my_store.add_product(105,String::from("laptop3"),String::from("high speed +++"), 70000.00,90000.00, 100){
    //     Ok(_)=>println!("product added successfully"),
    //     Err(e)=>println!("{}",e),
    // }

    // match my_store.add_product(101,String::from("laptop2"),String::from("light weight"), 50000.00,70000.00, 50){
    //     Ok(_)=>println!("product overwrote"),
    //     Err(e)=>println!("{}",e),
    // }

    // // edit products
    // match my_store.edit_product(101, 75000.00, 150){
    //     Ok(_)=>println!("product is updated"),
    //     Err(e)=>println!("{}",e),
    // }
    // match my_store.edit_product(102, 75000.00, 150){
    //     Ok(_)=>println!("product is updated"),
    //     Err(e)=>println!("{}",e),
    // }

    // delete products
    // match my_store.del_product(101){
    //     Ok(_)=>println!("product got deleted"),
    //     Err(e)=>println!("{}",e),
    // }
    // match my_store.del_product(101){
    //     Ok(_)=>println!("product got deleted"),
    //     Err(e)=>println!("{}",e),
    // }

    // // sell products
    // match my_store.record_sale(1,101,2){
    //     Ok(_)=>println!("product sold"),
    //     Err(e)=>println!("{}",e),
    // }
    // match my_store.record_sale(1,101,200){
    //     Ok(_)=>println!("product sold"),
    //     Err(e)=>println!("{}",e),
    // }

    // // purchase products
    // match my_store.record_purchase(1, 101, 100, 80000.00){
    //     Ok(_)=>println!("product purchased"),
    //     Err(e)=>println!("{}",e),
    // }
    // match my_store.record_purchase(1, 1010, 100, 80000.00){
    //     Ok(_)=>print!("product purchased"),
    //     Err(e)=>println!("{}",e),
    // }
    // // print inventory report
    // my_store.print_inventory_report();

    // // print sales report
    // my_store.print_sales_report();

    // // print purchase report
    // my_store.print_purchase_report();

}
