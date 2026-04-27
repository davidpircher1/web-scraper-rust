mod web_content;
mod product;

use crate::{product::Product, web_content::Content};
use std::fs::File;
use std::io::BufWriter;
use futures::stream::{self, StreamExt};


#[tokio::main]
async fn main() {
   let main_url = "https://auto.bazos.sk";
   let file = File::create("products.json").expect("Nepodarilo sa vytvoriť súbor");
   let writer = BufWriter::new(file);

   // create all url adresses for all pages :*
   let urls: Vec<String> = (0..=1000)
    .step_by(20)
    .map(|page| {
        if page == 0 {
            main_url.to_string()
        } else {
            format!("{}/{}/", main_url, page)
        }
    })
    .collect();

    let all_products: Vec<Product> = stream::iter(urls)
    .map(|url| {
        // for every url, we create paralel task
        async move {
            let mut my_web = Content::new(&url);
            match my_web.download().await {
                Ok(text) => {
                    my_web.set_content(text);
                    println!("Downloaded: {}", url);
                    my_web.products(main_url)
                }
                Err(e) => {
                    println!("Chyba pri {}: {}", url, e);
                    Vec::new() // error, empty list
                }
            }
        }
    })
    .buffer_unordered(5) // max 5 tasks
    .collect::<Vec<Vec<_>>>() // list of products list
    .await
    .into_iter()
    .flatten() // merge all lists together
    .collect();

    serde_json::to_writer_pretty(writer, &all_products).expect("Zápis zlyhal");
}
