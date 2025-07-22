use nanorand::{Rng, WyRand};
use strum::EnumCount;

pub trait RandomEnum: Default + EnumCount {
    fn from_repr(value: usize) -> Option<Self>;
    fn random() -> Self {
        Self::from_repr(WyRand::new().generate_range(0..Self::COUNT)).unwrap_or_default()
    }

    fn filtered_random(filter: &[Self]) -> Self
    where
        Self: Sized + Clone,
    {
        if filter.is_empty() {
            Self::random()
        } else {
            filter
                .get(WyRand::new().generate_range(0..filter.len()))
                .cloned()
                .unwrap_or_default()
        }
    }
}
