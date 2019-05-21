extern crate postgres;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

const URL: &'static str = "postgresql://admin@localhost:5432/youtube";
const COUNT: &'static str = "SELECT COUNT(*) FROM youtube.stats.channels";

struct Channel {
    id: u32,
    serial: [char; 24]
}

fn main() {
    let params: &'static str = URL;
    let tls: postgres::TlsMode = postgres::TlsMode::None;

    let conn: postgres::Connection =
        postgres::Connection::connect(params, tls).unwrap();

    let query: &'static str = COUNT;
    let row: postgres::rows::Rows = conn.query(query, &[]).unwrap();
    let count: i64 = row.get(0).get(0);
    println!("Count is {}", count);

    println!("Hello, world!");
}
