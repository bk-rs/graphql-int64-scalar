#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Int64Scalar(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UInt64Scalar(pub u64);

#[cfg(feature = "async-graphql")]
pub mod async_graphql;

#[cfg(feature = "cynic")]
pub mod cynic;
