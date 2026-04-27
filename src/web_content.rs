use reqwest;
use crate::product::Product;
use scraper::{Html, Selector};


pub struct Content {
    url: String,
    content: String,
}

impl Content {
    pub fn new(url: &str) -> Content {
        Content { url: url.to_string(), content: String::new()}
    }

    pub fn set_url(&mut self, url: &str) {
        self.url = url.to_string();
    }

    pub fn get_url(&self) -> &String {
        return &self.url;
    }

    pub fn set_content(&mut self, content: String) {
        self.content = content;
    }

    pub fn get_content(&self) -> &String {
        &self.content
    }

    pub async fn download(&self) -> Result<String, reqwest::Error> {
        let html = reqwest::get(&self.url).await?.text().await?;
        Ok(html)
    }

    pub fn products(&self, main_url: &str) -> Vec<Product>{
        let mut products = Vec::new();
        let doc =  Html::parse_document(self.get_content());

        // selector for box product
        let item_selector = Selector::parse(".inzeraty").unwrap();

        // selector for inside
        let name_selector = Selector::parse(".nadpis").unwrap(); // this will select href and name also for product
        let anchor_selector = Selector::parse("a").unwrap(); // select for link inside name
        let price_selector = Selector::parse(".inzeratycena").unwrap(); // this will select price


        for element in doc.select(&item_selector) {
            let name = element.select(&name_selector)
                .next()
                .map(|e|e.text().collect::<String>().trim().to_string())
                .unwrap_or_else(|| "no name".into());

            let link = element.select(&name_selector)
                    .next()
                    .and_then(|e| e.select(&anchor_selector).next()) 
                    .and_then(|a| a.value().attr("href"))          
                    .map(|s| format!("{}{}", main_url, s))                   
                    .unwrap_or_else(|| "no link".into());

            let price = element.select(&price_selector)
                .next()
                .map(|e|e.text().collect::<String>().trim().to_string())
                .unwrap_or_else(|| "no price".into());

            let product = Product::new(name, price, link);
            products.push(product);
        }
        products
    }
}
    