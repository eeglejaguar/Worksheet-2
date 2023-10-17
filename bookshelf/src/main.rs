use bookshelf::Book;

fn main() {
    let my_book = Book {
        title: String::from("David Copperfield"),
        author: String::from("ABC"),
        pages: 550,
    };

    println!("Title: {}", my_book.title);
}
