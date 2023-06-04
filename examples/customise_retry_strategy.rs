use lastfm::{retry_strategy::RetryStrategy, ClientBuilder};
use std::time::Duration;

/// A retry strategy that will retry 3 times with the following delays:
/// - Retry 0: 0 second delay
/// - Retry 1: 1 second delay
/// - Retry 2: 2 seconds delay
struct SimpleRetryStrategy {}

impl RetryStrategy for SimpleRetryStrategy {
    fn should_retry_after(&self, attempt: usize) -> Option<std::time::Duration> {
        // if retrying more than 3 times stop
        if attempt >= 3 {
            return None;
        }

        // otherwise wait a second per number of attempts
        Some(Duration::from_secs(attempt as u64))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let retry_strategy = SimpleRetryStrategy {};

    let client = ClientBuilder::new("some-api-key", "loige")
        .retry_strategy(Box::from(retry_strategy))
        .build();

    // do something with client...
    dbg!(client);

    Ok(())
}
