mod web_content;
mod product;

use crate::web_content::Content;
use std::fs::File;
use std::io::BufWriter;


#[tokio::main]
async fn main() {
   let main_url = "https://auto.bazos.sk";
   let mut page = 0;
   let mut my_web = Content::new(&main_url);
   let file = File::create("products.json").expect("Nepodarilo sa vytvoriť súbor");
   let writer = BufWriter::new(file);
   let mut all_products = Vec::new();


    loop{
        let html = my_web.download().await;

        match html {
            Ok(_text) => my_web.set_content(_text), 
            
            Err(e) => println!("Chyba: {}", e), 
        }

        let products_to_write = my_web.products(&main_url);

        if page == 1000{
            break;
        }

        all_products.extend(products_to_write);

        page += 20;

        my_web.set_url(&format!("{}/{}/", main_url, page));

        println!("{}", my_web.get_url());
    }

    serde_json::to_writer_pretty(writer, &all_products).expect("Zápis zlyhal");
}
