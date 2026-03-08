use crate::models::hazard::hazard_field_filter::HazardComplexityEnum;

pub trait HasComplexity {
    fn complexity(&self) -> HazardComplexityEnum;
}
