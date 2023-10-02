mod book_store;
mod scraper_impl;
mod select_impl;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    book_store::get_categories().await?;
    book_store::save_books_for_categories().await
}
