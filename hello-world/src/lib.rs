use tide::{Request, Server};

pub fn create_app() -> Server<()> {
    let mut app = tide::new();
    app.at("/hello/:name").get(hello_handler);
    app
}

async fn hello_handler(req: Request<()>) -> tide::Result<String> {
    let name = req.param("name")?;
    Ok(hello(name))
}

fn hello(name: String) -> String {
    format!("Hello there, {}\n", name)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hello() {
        assert_eq!("Hello there, SomeName\n", hello(String::from("SomeName")));
    }
}
