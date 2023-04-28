use strum::{EnumString, EnumVariantNames, VariantNames};

/// info
#[derive(EnumString, EnumVariantNames, Debug, Clone)]
#[strum(serialize_all = "kebab_case")]
pub enum Infomation {
    Floor,
    Author,
    User,
}
