mod builder;
pub mod errors;
pub mod util;
pub mod wrapper;
pub use builder::*;

pub use wrapper::sql::*;

#[macro_export]
#[cfg(feature = "async-std")]
macro_rules! block_on {
    (move || $block:expr) => {
        async_std::task::spawn_blocking(move || $block).await?
    };
}

#[macro_export]
#[cfg(feature = "tokio")]
macro_rules! block_on {
    (move || $block:expr) => {
        tokio::task::spawn_blocking(move || $block).await??
    };
}
