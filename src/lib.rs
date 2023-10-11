use rusnap::{handler, Route};

#[handler]
pub async fn handle_hello() -> String {
    "hello world".into()
}

async fn main() {
    Route::new(()).at("hello", handle_hello).serve();
}

rusnap::entry!(main);
