use strum::{EnumString, EnumVariantNames, VariantNames};

/// the day [today, tomorrow]
#[derive(EnumString, EnumVariantNames, Debug, Clone)]
#[strum(serialize_all = "kebab_case")]
pub enum Day {
    Today,
    Tomorrow,
}
