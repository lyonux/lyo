use anyhow::Result;

pub trait Producer<T> {
    fn produce(&self) -> impl std::future::Future<Output = Result<T>> + Send;
}
