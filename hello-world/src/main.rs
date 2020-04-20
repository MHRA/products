#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    hello_world::create_app().listen("0.0.0.0:3030").await
}
