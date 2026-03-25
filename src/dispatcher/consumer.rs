pub trait Consumer<T> {
    fn consume(&self, action: &T) -> impl std::future::Future<Output = ()> + Send;
    fn stop(&self) -> impl std::future::Future<Output = ()> + Send;
}
