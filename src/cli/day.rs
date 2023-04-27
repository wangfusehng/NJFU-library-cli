use strum::{EnumString, EnumVariantNames, VariantNames};

#[derive(EnumString, EnumVariantNames, Debug, Clone)]
#[strum(serialize_all = "kebab_case")]
pub enum Day {
    Today,
    Tomorrow,
}
