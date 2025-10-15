use crate::models::npc::gender_enum::Gender;

pub trait HasValidGenders: Default {
    fn get_valid_genders(&self) -> Vec<Gender>;

    fn has_at_least_one_gender_in_common(&self, genders: Vec<Gender>) -> bool {
        self.get_valid_genders()
            .iter()
            .any(|supp_gender| genders.contains(supp_gender))
    }
}
