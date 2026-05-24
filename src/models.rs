use std::collections::HashMap;
pub struct Product{
    pub id:u32,
    pub name:String,
    pub description:String,
    pub cost_price:f64,
    pub sell_price:f64,
    pub quantity:u32,
}
pub struct Sale{
    pub sale_id:u32,
    pub product_id:u32,
    pub quantity:u32,
    pub sell_price:f64,

}
pub struct Purchase{
    pub purchase_id:u32,
    pub product_id:u32,
    pub quantity:u32,
    pub cost_price:f64,
}
pub struct Store{
    pub products:HashMap<u32,Product>,
    pub sales:Vec<Sale>,
    pub purchases:Vec<Purchase>,

}

impl Store{
    pub fn new()->Self{
        Store { 
            products:HashMap::new() , 
            sales: Vec::new(), 
            purchases:Vec::new(),
         }
    }

    pub fn add_product(&mut self,id:u32,name:String,description:String,cost_price:f64,sell_price:f64,quantity:u32)->Result<(),String>{
        let product=Product{
            id,
            name,
            description,
            cost_price,
            sell_price,
            quantity,
        };
        if self.products.contains_key(&id){
            Err(String::from("Product already exists!!!"))
        }else{
            self.products.insert(id, product);
            Ok(())
        }
    }

    pub fn edit_product(&mut self,id:u32,new_price:f64,new_quantity:u32)->Result<(),String>{
        match self.products.get_mut(&id){
            Some(product)=>{
                product.sell_price=new_price;
                product.quantity=new_quantity;
                Ok(())
            }
            None=>{
                Err(String::from("product not found"))
            }
        }

    }

    pub fn del_product(&mut self,id:u32)->Result<(),String>{
        match self.products.remove(&id){
            Some(removed_product )=>{
                println!("product {} deleted",removed_product.name);
                Ok(())
            }
            None=>{
                Err(String::from("product does not exist"))
            }
        }
    }

    pub fn record_sale(&mut self,sale_id:u32,product_id:u32,quantity:u32)->Result<(),String>{
        match self.products.get_mut(&product_id){
            Some(product)=>{
                if product.quantity >= quantity {
                    product.quantity-=quantity;
                    let total_revenue=(quantity as f64) *product.sell_price;
                    let total_costprice=(quantity as f64)*product.cost_price;
                    let profit=total_revenue-total_costprice;
                    let sale=Sale{
                        sale_id,
                        product_id,
                        quantity,
                        sell_price:product.sell_price,
        
                    };
                    self.sales.push(sale);
                    println!("the profit for the sale {} is {}",sale_id,profit);
                    Ok(())
                }else{
                    Err(String::from("insufficient stock"))
                }
            }
            None=>{
                Err(String::from("product not found"))
            }
        }
    }

    pub fn record_purchase(&mut self,purchase_id:u32,product_id:u32,quantity:u32,new_cost_price:f64)->Result<(),String>{
        match self.products.get_mut(&product_id){
            Some(product)=>{
                product.quantity+=quantity;
                product.cost_price=new_cost_price;
                let purchase=Purchase{
                    purchase_id,
                    product_id,
                    quantity,
                    cost_price:new_cost_price,
                };
                self.purchases.push(purchase);
                Ok(())

            }
            None=>{
                Err(String::from("product must be added in store first"))
            }
        }
    }

    pub fn print_inventory_report(&self){
        println!("Inventory Report");
        println!("{:<10} {:<20} {:<20} {:<10} {:<10} {:<10}","ID","NAME","DESCRIPTION","COST_PRICE","SELL_PRICE","QUANTITY");
        println!("{}","-".repeat(65));
        for product in self.products.values(){
            println!("{:<10} {:<20} {:<20} ${:<9.2} ${:<9.2} {:<10}",product.id,
        product.name,product.description,product.cost_price,product.sell_price,product.quantity);
        }

    }
    pub fn print_sales_report(&self){
        println!("Sales Report");
        println!("{:<10} {:<10} {:<10} {:<10}","SALE_ID","PRODUCT_ID","QUANTITY","SELL_PRICE");
        println!("{}","-".repeat(60));
    for sale in self.sales.iter(){
        println!("{:<10} {:<10} {:<10} ${:<9.2}",sale.sale_id,sale.product_id
    ,sale.quantity,sale.sell_price);
    }
    }

    pub fn print_purchase_report(&self){
        println!("Purchase Report");
        println!("{:<10} {:<10} {:<10} {:<10}","PURCHASE_ID","PRODUCT_ID","QUANTITY","COST_PRICE");
        println!("{}","-".repeat(60));
    for purchase in self.purchases.iter(){
        println!("{:<10} {:<10} {:<10} ${:<9.2}",purchase.purchase_id,purchase.product_id
    ,purchase.quantity,purchase.cost_price);
    }
    }
}