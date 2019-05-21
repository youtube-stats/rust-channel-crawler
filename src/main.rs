extern crate postgres;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

const POSTGRESQL_URL: &'static str = "postgresql://admin@localhost:5432/youtube";
const INSERT: &'static str =
    "INSERT INTO youtube.stats.channels (serial) VALUES ($1) ON CONFLICT DO NOTHING";
const PAGE_LIMIT: u32 = 114462;
const RANGE_LIMIT: u32 = PAGE_LIMIT + 1;

struct Channel {
    id: u32,
    serial: String
}

fn main() {
    /*let params: &'static str = POSTGRESQL_URL;
    let tls: postgres::TlsMode = postgres::TlsMode::None;

    let conn: postgres::Connection =
        postgres::Connection::connect(params, tls).unwrap();*/

    for i in 1..RANGE_LIMIT {
        let url: String =
            format!("http://www.channelcrawler.com/eng/results/136614/page:{}", i);
        let body: String = reqwest::get(url.as_str()).unwrap().text().unwrap();
        println!("{}", body);
    }

    println!("Hello, world!");
}
