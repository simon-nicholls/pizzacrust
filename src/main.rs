extern crate nickel;
extern crate postgres;
extern crate serialize;

use std::io::net::ip::Ipv4Addr;
use std::collections::HashMap;
use nickel::{ Nickel, Request, Response, HttpRouter };

use postgres::{PostgresConnection, NoSsl};

#[deriving(Encodable)]
struct Bake {
    name: String,
}

const DB_CONF: &'static str = "postgres://postgres@localhost/food";

fn main() {
    let mut server = Nickel::new();

    fn bakes_handler (_request: &Request, response: &mut Response) {
        let conn = PostgresConnection::connect(DB_CONF, &NoSsl).unwrap();
        let stmt = conn.prepare("SELECT name FROM bakes").unwrap();
        let bakes = stmt.query([]).unwrap()
            .map(|r| Bake {name: r.get(0u)})
            .collect();
        let mut data = HashMap::<&str, Vec<Bake>>::new();
        data.insert("bakes", bakes);
        response.render("assets/bakes.tpl", &data);
    }

    fn root_handler (_request: &Request, response: &mut Response) {
        let mut data = HashMap::<&str, &str>::new();
        data.insert("name", "Pizza 🍕😻");
        response.render("assets/index.tpl", &data);
    }

    server.get("/", root_handler);
    server.get("/bakes", bakes_handler);
    server.listen(Ipv4Addr(127, 0, 0, 1), 6767);
}
