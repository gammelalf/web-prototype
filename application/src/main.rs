use framework::db::Db;

#[tokio::main]
async fn main() {
    framework::Registry::builder()
        .register_module::<Db>()
        .init()
        .await
        .unwrap();
}
