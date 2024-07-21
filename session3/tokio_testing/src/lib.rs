pub async fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test(flavor = "multi_thread")]
    async fn test_easy_way() {
        assert_eq!(add(1, 1).await, 2);
    }
}
