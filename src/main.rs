use std::fmt::{self, Display, Formatter};

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
    fn new(id: u16, name: &str, author: &str, price: f32) -> Book {
        Book {
            id: id,
            name: name.to_string(),
            _author: author.to_string(),
            _price: price,
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
        .map(|b| b.to_string())
        .collect::<Vec<String>>()
        .join("\n")
}

fn main() {
    server::new(|| {
        App::new()
            .resource("/", |r| r.f(greet))
            .resource("/books", |r| r.f(books))
            .resource("/{name}", |r| r.f(greet))
    })
    .bind("127.0.0.1:8000")
    .expect("Can not bind to port 8000")
    .run();
}
