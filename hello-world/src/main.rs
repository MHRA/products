fn main() -> std::io::Result<()> {
    smol::run(hello_world::create_app().listen("0.0.0.0:3030"))
}
