extern crate postgres;
extern crate reqwest;
extern crate scraper;
extern crate serde;
extern crate serde_json;

const POSTGRESQL_URL: &'static str = "postgresql://admin@localhost:5432/youtube";
const INSERT: &'static str =
    "INSERT INTO youtube.stats.channels (serial) VALUES ($1) ON CONFLICT DO NOTHING";
const PAGE_LIMIT: u32 = 114462;
const RANGE_LIMIT: u32 = PAGE_LIMIT + 1;

fn main() {
    let params: &'static str = POSTGRESQL_URL;
    let tls: postgres::TlsMode = postgres::TlsMode::None;

    let conn: postgres::Connection =
        postgres::Connection::connect(params, tls).unwrap();

    for i in 1..RANGE_LIMIT {
        let url: String = format!("http://www.channelcrawler.com/eng/results/136614/sort:Channel.subscribers/direction:desc/page:{}", i);
        let document: String = reqwest::get(url.as_str()).unwrap().text().unwrap();
        let document: &str = document.as_str();

        let doc: scraper::Html = scraper::Html::parse_document(document);

        let selectors: &str = "div.channel > h4 > a[href]";
        let channels: scraper::Selector = scraper::Selector::parse(selectors).unwrap();

        for element in doc.select(&channels) {
            let channel_url: &str = element.value().attr("href").unwrap();
            let n: usize = 31;

            let channel_serial: String = channel_url.chars().skip(31).collect();
            let query: &str = INSERT;

            let result: u64 = conn.execute(query, &[&channel_serial]).unwrap();
            if result == 1{
                println!("Inserting {}", channel_serial);
            } else {
                println!("Ignoring {}", channel_serial);
            }
        }
    }

    println!("Hello, world!");
}
