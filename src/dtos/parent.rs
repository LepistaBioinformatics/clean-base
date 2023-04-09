use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};
use utoipa::ToSchema;

/// A parent record
///
/// This enumerator allow represents the parent elements using their primary
/// key (Id option) or the true record (Record option).
#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum ParentEnum<T, U> {
    Record(T),
    Id(U),
}

impl<T, U> Display for ParentEnum<T, U>
where
    T: Display,
    U: Display,
{
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            ParentEnum::Record(_) => write!(f, "record"),
            ParentEnum::Id(_) => write!(f, "id"),
        }
    }
}
