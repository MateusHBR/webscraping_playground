use select::{
    document::Document,
    predicate::{Class, Name, Predicate},
};

pub async fn select_impl() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://news.ycombinator.com/";
    let html = reqwest::get(url).await?.text().await?;
    let document = Document::from_read(html.as_bytes())?;

    for (n, node) in document.find(Class("athing")).enumerate() {
        let story = node
            .find(Class("title").descendant(Name("a")))
            .next()
            .unwrap()
            .text();

        println!("{}. {}", n + 1, story);
    }
    Ok(())
}
