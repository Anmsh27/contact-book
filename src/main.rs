use con_book::*;

fn main() {

    let mut book = ContactBook::new();

    book.add(Contact::new(
        "fds".to_string(),
        123_u64,
        "1tsvgdbhn".to_string(),
    ));

}
