use crate::traits::origin::average_name_length::AverageNameLength;
use crate::traits::origin::context_size::ContextSize;
use crate::traits::origin::has_valid_genders::HasValidGenders;
use crate::traits::random_enum::RandomEnum;
use std::fmt::Display;

pub trait Ancestry:
    ContextSize + HasValidGenders + Clone + Default + RandomEnum + AverageNameLength + Display
{
}
