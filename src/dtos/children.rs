use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};
use utoipa::ToSchema;

/// A children record
///
/// This enumerator allow represents the children elements using their primary
/// keys (Ids option) or the true records (Record option).
#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum ChildrenEnum<T, U> {
    Ids(Vec<U>),
    Records(Vec<T>),
}

impl<T, U> Display for ChildrenEnum<T, U>
where
    T: Display,
    U: Display,
{
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            ChildrenEnum::Ids(_) => write!(f, "ids"),
            ChildrenEnum::Records(_) => write!(f, "records"),
        }
    }
}
