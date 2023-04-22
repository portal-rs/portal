use std::{future::Future, time::Duration};

use tokio::time::timeout as to;

pub enum TimeoutResult<O, E> {
    Timeout,
    Error(E),
    Ok(O),
}

/// Requires a future to complete before the specified duration has elapsed.
/// This type returns one of three distinct variants:
///
/// - [`TimeoutResult::Timeout`] indicates that the specified duration has
///   elapsed.
/// - [`TimeoutResult::Error`] indicates that the future resulted in an error.
/// - [`TimeoutResult::Ok`] indicates the future resolved correctly and has
///   returned a success value.
///
/// ### Example
///
/// ```ignore
/// use std::time::Duration;
///
/// use portal::utils::{timeout, TimeoutResult};
/// use tokio::net::UdpSocket;
///
/// let socket = match timeout(Duration::from_secs(2), UdpSocket::bind("127.0.0.1:0")).await {
///     TimeoutResult::Timeout => panic!("Binding UDP socket timed out"),
///     TimeoutResult::Error(err) => panic!("An error occurred: {}", err),
///     TimeoutResult::Ok(socket) => socket,
/// };
/// ```
pub async fn timeout<T: Future<Output = Result<O, E>>, O, E>(
    d: Duration,
    f: T,
) -> TimeoutResult<O, E> {
    match to(d, f).await {
        Ok(res) => match res {
            Ok(o) => TimeoutResult::Ok(o),
            Err(err) => TimeoutResult::Error(err),
        },
        Err(_) => TimeoutResult::Timeout,
    }
}
