use anyhow::Result;

pub trait Producer<T> {
    fn produce(&mut self) -> impl std::future::Future<Output = Result<T>> + Send;
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test that Producer trait can be used with generic type parameters
    #[test]
    fn test_trait_usage() {
        // The trait is designed for use with generic type parameters, not as trait objects
        // because it uses `impl Future` in return position
        fn _accepts_generic<T: Producer<String>>(_producer: T) {}
    }
}
