enum Publication {
    Book(Book),
    Magazine(Magazine),
}
struct Book {
    title: String,
    author: String,
    page_count: u32,
}
struct Magazine {
    title: String,
    issue: u32,
    topic: String,
}

fn main() {
    let books = vec![
        Publication::Book(Book {
            title: String::from("Nutuk"),
            author: String::from("Mustafa Kemal Atatürk"),
            page_count: 658,
        }),
        Publication::Book(Book {
            title: String::from("Savaş ve Barış"),
            author: String::from("Tolstoy"),
            page_count: 1180,
        }),
    ];

    let magazines = vec![
        Publication::Magazine(Magazine {
            title: String::from("Rust Gazette"),
            issue: 12,
            topic: String::from("Rust Community News"),
        }),
        Publication::Magazine(Magazine {
            title: String::from("Programming World"),
            issue: 45,
            topic: String::from("Latest Programming Trends"),
        }),
    ];

    print_publications(&books);
    print_publications(&magazines);
}

fn print_publications(publications: &Vec<Publication>) {
    for publ in publications {
        match publ {
            Publication::Book(book) => {
                println!(
                    "Kitap: {} Yazar: {}, {} Sayfa",
                    book.title, book.author, book.page_count
                );
            }
            Publication::Magazine(magazine) => {
                println!(
                    "Dergi: {} - Sayı: {}, Konu: {}",
                    magazine.title, magazine.issue, magazine.topic
                );
            }
        }
    }
}
