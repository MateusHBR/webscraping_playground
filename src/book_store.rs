use csv::{ReaderBuilder, WriterBuilder};
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
struct CategoryRecord {
    category: String,
    url: String,
}

impl CategoryRecord {
    fn new(category: &str, url: &str) -> Self {
        Self {
            category: category.to_string(),
            url: url.to_string(),
        }
    }
}

pub async fn get_products_from_home() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new("usercode/output/book_store/out_products.txt");
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let mut output = File::create(path)?;

    let url = "http://books.toscrape.com/index.html";
    scrape_book_list(url, &mut output).await?;
    Ok(())
}

async fn scrape_book_list(url: &str, output: &mut File) -> Result<(), Box<dyn std::error::Error>> {
    let html = reqwest::get(url).await?.text().await?;

    let doc = Html::parse_document(&html);

    let books = Selector::parse(".product_pod").unwrap();
    let prices = Selector::parse(".price_color").unwrap();

    let h3_tag = Selector::parse("h3").unwrap();
    let a_tag = Selector::parse("a").unwrap();

    for book in doc.select(&books) {
        let price = book.select(&prices).next().unwrap();

        let h3 = book.select(&h3_tag).next().unwrap();
        let a = h3.select(&a_tag).next().unwrap();

        let book_title = a.text().collect::<Vec<_>>();
        let book_price = price.text().collect::<Vec<_>>();

        // println!("{}: {}", book_title[0], book_price[0]);
        writeln!(output, "{}, {}", book_title[0], book_price[0]).ok();
    }
    Ok(())
}

pub async fn get_categories() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new("usercode/output/book_store/out_categories.csv");
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let output = File::create(path)?;
    let mut writer = WriterBuilder::new().flexible(true).from_writer(output);

    let url = "http://books.toscrape.com/index.html";
    let html = reqwest::get(url).await?.text().await?;

    let doc = Html::parse_document(&html);

    let ul_tag = Selector::parse("ul").unwrap();
    let li_tag = Selector::parse("li").unwrap();
    let a_tag = Selector::parse("a").unwrap();

    let category_class = Selector::parse(".side_categories").unwrap();
    let categories = doc
        .select(&category_class)
        .next()
        .unwrap()
        .select(&ul_tag)
        .next()
        .unwrap()
        .select(&li_tag)
        .next()
        .unwrap()
        .select(&ul_tag)
        .next()
        .unwrap();

    for category in categories.select(&li_tag) {
        let category_title = category
            .text()
            .collect::<Vec<_>>()
            .join("")
            .trim()
            .to_string();

        let link = category
            .select(&a_tag)
            .next()
            .unwrap()
            .value()
            .attr("href")
            .unwrap();

        writer.serialize(vec![CategoryRecord::new(
            &category_title,
            &format!("https://books.toscrape.com/{}", link),
        )])?;
    }

    writer.flush()?;

    Ok(())
}

pub async fn save_books_for_categories() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new("usercode/output/book_store/out_categories.csv");
    let mut reader = ReaderBuilder::new().flexible(true).from_path(path).unwrap();
    let categories = reader
        .deserialize::<CategoryRecord>()
        .flatten()
        .collect::<Vec<CategoryRecord>>();

    let output_path = Path::new("usercode/output/book_store/category");
    fs::create_dir_all(output_path)?;

    let mut handles = vec![];
    for category in categories {
        let handle = tokio::spawn(async move {
            let mut output = File::create(format!(
                "usercode/output/book_store/category/out_{}.txt",
                category.category
            ))
            .unwrap();
            scrape_book_list(&category.url, &mut output).await.unwrap();
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.await?;
    }

    Ok(())
}
