use con_book::*;

fn main() {
    let mut book = ContactBook::new();

    book.add(Contact::new(
        "fds".to_string(),
        123_u64,
        "1tsvgdbhn".to_string(),
    )).unwrap();

    book.remove_by_name("fds").unwrap();

    book.add(Contact::new(
        "Animesh".to_string(),
        4322_u64,
        "Animeshjoshi2007@gmail.com".to_string(),
    )).unwrap();
}
