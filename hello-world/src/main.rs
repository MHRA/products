#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    let mut app = tide::new();
    app.at("/hello/:name").get(hello_handler);
    app.listen("127.0.0.1:3030").await
}

async fn hello_handler(req: tide::Request<()>) -> tide::Result<String> {
    let name: String = req.param("name").expect("no name param");
    Ok(format!("Hello, {}\n", name))
}
