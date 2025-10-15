use crate::traits::random_enum::RandomEnum;

pub trait JobEnum: RandomEnum + ToString + Clone {}
