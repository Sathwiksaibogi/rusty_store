use std::collections::HashMap;
pub struct Product{
    pub id:u32,
    pub name:String,
    pub description:String,
    pub price:f64,
    pub quantity:u32,
}
pub struct Sale{
    pub sale_id:u32,
    pub product_id:u32,
    pub quantity:u32,
    pub price:f64,

}
pub struct Purchase{
    pub purchase_id:u32,
    pub product_id:u32,
    pub quantity:u32,
    pub price:f64,
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

    pub fn add_product(&mut self,id:u32,name:String,description:String,price:f64,quantity:u32)->Result<(),String>{
        let product=Product{
            id,
            name,
            description,
            price,
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
                product.price=new_price;
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
}