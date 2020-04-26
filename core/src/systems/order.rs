use serde::{Deserialize, Serialize};

/// Priority of systems
///
/// This is a primitive for [`SystemOrder`].
///
/// [`SystemOrder`]: ./struct.SystemOrder.html
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Deserialize, Serialize)]
pub enum SystemPriority {
    /// The system executes at the very first
    First,
    /// The system executes at the given priority, with lower number having higher priority
    Numbered(usize),
    /// The system executes at the very last
    Last,
}

/// The order of systems
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Deserialize, Serialize)]
pub struct SystemOrder(pub SystemPriority, pub SystemPriority);

impl SystemOrder {
    /// Create a new instance with the given numbers
    pub fn numbered(n1: usize, n2: usize) -> Self {
        Self(SystemPriority::Numbered(n1), SystemPriority::Numbered(n2))
    }

    /// Create a new instance with the given order in the first order
    pub fn from_first_number(n: usize) -> Self {
        Self(SystemPriority::First, SystemPriority::Numbered(n))
    }

    /// Create a new instance with the given order in the last order
    pub fn from_last_number(n: usize) -> Self {
        Self(SystemPriority::Last, SystemPriority::Numbered(n))
    }

    /// Create a new instance with the first order in the given order
    pub fn from_number_first(n: usize) -> Self {
        Self(SystemPriority::Numbered(n), SystemPriority::First)
    }

    /// Create a new instance with the last order in the given order
    pub fn from_number_last(n: usize) -> Self {
        Self(SystemPriority::Numbered(n), SystemPriority::Last)
    }

    /// Create a new instance with the first order
    pub fn first() -> Self {
        Self(SystemPriority::First, SystemPriority::First)
    }

    /// Create a new instance with the last order
    pub fn last() -> Self {
        Self(SystemPriority::Last, SystemPriority::Last)
    }
}
