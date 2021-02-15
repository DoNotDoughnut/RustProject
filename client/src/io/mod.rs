pub mod input;
pub mod configuration;
// mod level_loader;

#[async_trait::async_trait(?Send)]
pub trait PersistentData {

    async fn load() -> Self;

    async fn save(&self);

}