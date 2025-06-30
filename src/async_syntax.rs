// use std::future::Future;
use trpl::{Either, Html};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    trpl::run(async {
        let url_1 = &args[1];
        let url_2 = &args[2];

        let title_fut_1 = page_title(url_1);
        let title_fut_2 = page_title(url_2);

        let (url, maybe_title) =
            match trpl::race(title_fut_1, title_fut_2).await {
                Either::Left(left) => left,
                Either::Right(right) => right,
            };

        println!("{url} finished first");

        match maybe_title {
            Some(title) => println!("title for {url} is {title}"),
            None => println!("{url} had no title"),
        }
    })
}

async fn page_title(url: &str) -> (&str, Option<String>) {
    let response_text = trpl::get(url).await.text().await;
    let title = Html::parse(&response_text)
        .select_first("title")
        .map(|title_element| title_element.inner_html());

    (url, title)
}

// fn page_title(url: &str) -> impl Future<Output = Option<String>> {
//     async move {
//         let text = trpl::get(url).await.text().await;
//         Html::parse(&text)
//             .select_first("title")
//             .map(|title_element| title_element.inner_html())
//     }
// }
