use crate::traits::random_enum::RandomEnum;

pub trait ClassEnum: RandomEnum + ToString + Clone {}
