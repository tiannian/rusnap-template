use rusnap::{handler, main, Route};

#[handler]
pub async fn handle_hello() -> String {
    "hello world".into()
}

#[main]
async fn main() {
    Route::new(()).at("hello", handle_hello).serve();
}
