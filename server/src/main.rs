mod app;

use app::App;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    App::new()?.run().await
}
