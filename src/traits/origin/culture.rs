use crate::traits::origin::average_name_length::AverageNameLength;
use crate::traits::origin::context_size::ContextSize;
use crate::traits::random_enum::RandomEnum;
use std::fmt::Display;

pub trait Culture: ContextSize + AverageNameLength + Clone + RandomEnum + Display {}
