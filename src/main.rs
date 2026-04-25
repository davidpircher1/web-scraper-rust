mod web_content;
mod product;

use crate::web_content::Content;
use std::fs::File;
use std::io::BufWriter;


#[tokio::main]
async fn main() {
   let mut my_web = Content::new("https://auto.bazos.sk");
   let file = File::create("products.json").expect("Nepodarilo sa vytvoriť súbor");
    let writer = BufWriter::new(file);

   let html = my_web.download().await;

    match html {
        Ok(_text) => my_web.set_content(_text), 
        
        Err(e) => println!("Chyba: {}", e), 
    }

    serde_json::to_writer_pretty(writer, &my_web.products()).expect("Zápis zlyhal");




}
