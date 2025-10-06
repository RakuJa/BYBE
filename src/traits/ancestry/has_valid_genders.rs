use crate::models::npc::gender_enum::Gender;

pub trait HasValidGenders: Default {
    fn get_valid_genders(&self) -> Vec<Gender>;
}
