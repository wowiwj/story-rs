use tide::log;

#[async_std::main]
async fn main() -> tide::Result<()> {
    log::start();
    let mut app = tide::new();
    app.at("/").get(|_| async {
        Ok("hello world")
    });
    log::info!("app is running");
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}