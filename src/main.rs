use std::fmt::{self, Display, Formatter};
use std::net::TcpListener;

use actix_web::{server, App, HttpRequest, Responder};

fn greet(req: &HttpRequest) -> impl Responder {
    let to = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", to)
}

struct Book {
    id: u16,
    name: String,
    _author: String,
    _price: f32,
}

impl Display for Book {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{} {}", self.id, self.name)
    }
}

impl Book {
    fn new(id: u16, name: &str, author: &str, _price: f32) -> Book {
        Book {
            id,
            name: name.to_string(),
            _author: author.to_string(),
            _price,
        }
    }
}

fn books(_: &HttpRequest) -> impl Responder {
    let books = vec![
        Book::new(1, "1984", "George Orwell", 12f32),
        Book::new(2, "Animal Farm", "George Orwell", 10f32),
        Book::new(3, "To Kill A Mocking Bird", "Harper Lee", 15f32),
    ];

    books
        .iter()
        .map(Book::to_string)
        .collect::<Vec<String>>()
        .join("\n")
}

pub fn get_unused_tcp_port() -> u16 {
    TcpListener::bind("127.0.0.1:0")
        .unwrap()
        .local_addr()
        .unwrap()
        .port()
}

fn main() {
    let port = get_unused_tcp_port();
    println!("Listening on URL: http://localhost:{}/", port);

    server::new(|| {
        App::new()
            .resource("/", |r| r.f(greet))
            .resource("/books", |r| r.f(books))
            .resource("/{name}", |r| r.f(greet))
    })
    .bind(format!("127.0.0.1:{}", port))
    .expect(&format!("Can not bind to port {}", port))
    .run();
}
