pub trait Consumer<T>: Send + Sync {
    fn consume(&mut self, action: &T) -> impl std::future::Future<Output = ()> + Send;
    fn stop(&mut self) -> impl std::future::Future<Output = ()> + Send;
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test that Consumer trait can be used with generic type parameters
    #[test]
    fn test_trait_usage() {
        // The trait is designed for use with generic type parameters, not as trait objects
        // because it uses `impl Future` in return position
        fn _accepts_generic<T: Consumer<String>>(_consumer: T) {}
    }
}
