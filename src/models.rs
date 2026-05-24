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

