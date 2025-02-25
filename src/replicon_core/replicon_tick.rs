use std::cmp::Ordering;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// A tick that increments each time we need the server to compute and send an update.
///
/// Updated on clients every time they receive replication from the server.
/// Mapped to the Bevy's `Tick` in [`AckedTicks`](crate::server::AckedTicks).
/// See also [`TickPolicy`](crate::server::TickPolicy).
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, PartialEq, Resource, Serialize)]
pub struct RepliconTick(pub(crate) u32);

impl RepliconTick {
    /// Gets the value of this tick.
    #[inline]
    pub fn get(self) -> u32 {
        self.0
    }

    /// Increments current tick by the specified `value` and takes wrapping into account.
    #[inline]
    pub fn increment_by(&mut self, value: u32) {
        self.0 = self.0.wrapping_add(value);
    }

    /// Same as [`Self::increment_by`], but increments only by 1.
    #[inline]
    pub fn increment(&mut self) {
        self.increment_by(1)
    }
}

impl PartialOrd for RepliconTick {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let difference = self.0.wrapping_sub(other.0);
        if difference == 0 {
            Some(Ordering::Equal)
        } else if difference > u32::MAX / 2 {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Greater)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tick_comparsion() {
        assert_eq!(RepliconTick(0), RepliconTick(0));
        assert!(RepliconTick(0) < RepliconTick(1));
        assert!(RepliconTick(0) > RepliconTick(u32::MAX));
    }
}
