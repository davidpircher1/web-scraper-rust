use serde::Serialize;

#[derive(Serialize)]
pub struct Product {
    pub name: String,
    pub price: String,
    pub link: String,
}

impl Product {
    pub fn new(name: String, price: String, link: String) -> Self{
        Product { name: name, price: price, link: link}
    }
}