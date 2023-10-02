use scraper::{Html, Selector};

pub async fn scraper_impl() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://news.ycombinator.com/";
    let html = reqwest::get(url).await?.text().await?;

    let doc = Html::parse_document(&html);

    let stories = Selector::parse(".athing").unwrap();
    let titles = Selector::parse("td.title").unwrap();
    let a_tag = Selector::parse("a").unwrap();

    for story in doc.select(&stories) {
        let title = story.select(&titles).skip(1).next().unwrap();
        let a = title.select(&a_tag).next().unwrap();
        let story_title = a.text().collect::<Vec<_>>();
        println!("{}", story_title[0]);
    }
    Ok(())
}
