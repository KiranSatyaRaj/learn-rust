use std::future::Future;
use trpl::{Either, Html};

// rust compiles this function into a non-async function
// whose body is an async block
async fn page_title(url: &str) -> Option<String> {
    let response_text = trpl::get(url).await.text().await;
    Html::parse(&response_text)
        .select_first("title")
        .map(|title_element| title_element.inner_html())
}

// this function is equivalent to the page_title function
// with async keyword in its signature
fn page_title_block(url: &str) -> impl Future<Output = Option<String>> + '_ {
    // rust compiles this block with async keyword
    // into a unique, anonymous data structure
    // that implements Future trait
    async move {
        let response_text = trpl::get(url).await.text().await;
        Html::parse(&response_text)
            .select_first("title")
            .map(|title_element| title_element.inner_html())
    }
}
fn main() {
    let args: Vec<String> = std::env::args().collect();
    trpl::run(
        async {
            let url = &args[1];
            match page_title(url).await {
                Some(title) => println!("The title for {url} was {title}"),

                None => println!("{url} has no title"),
            }
        }
    );

    trpl::run(async {
        let title_fut_1 = page_title_race(&args[1]);
        let title_fut_2 = page_title_race(&args[2]);

        let (url, maybe_title) = 
        match trpl::race(title_fut_1, title_fut_2).await {
            Either::Left(left) => left,
            Either::Right(right) => right,
        };
        println!("{url} returned first");
        match maybe_title {
            Some(title) => println!("Its page title is: {title}"),
            None => println!("Its title could not be parsed"),
        }
    })
}

async fn page_title_race(url: &str) -> (&str, Option<String>) {
    let text = trpl::get(url).await.text().await;
    let title = Html::parse(&text)
        .select_first("title")
        .map(|title_element| title_element.inner_html());
    (url, title)
}