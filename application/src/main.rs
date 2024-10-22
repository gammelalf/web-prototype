use framework::core::module::Module;
use framework::db::module::Db;

#[tokio::main]
async fn main() {
    framework::Registry::builder()
        .register_module::<Db>()
        .init()
        .await
        .unwrap();

    let _db = Db::global();
}
