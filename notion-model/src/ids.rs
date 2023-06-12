//! Notion IDs

use std::fmt;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

macro_rules! uuid_id {
    ($($name:ident;)*) => {
        $(

            #[derive(
                Copy, Clone, Default, Debug, Hash, Eq, PartialEq,
                Ord, PartialOrd, Deserialize, Serialize,
            )]
            pub struct $name(pub Uuid);

            impl $name {
                /// Immutably borrow inner Id.
                #[inline]
                #[must_use]
                pub const fn as_uuid(&self) -> &Uuid {
                    &self.0
                }
            }

            impl std::str::FromStr for $name {
                type Err = uuid::Error;

                fn from_str(uuid_str: &str) -> Result<Self, Self::Err> {
                    Ok(Self(Uuid::parse_str(uuid_str)?))
                }
            }


            impl TryFrom<&'_ str> for $name {
                type Error = uuid::Error;

                fn try_from(uuid_str: &'_ str) -> Result<Self, Self::Error> {
                    Ok(Self(Uuid::parse_str(uuid_str)?))
                }
            }

            impl<T: ?Sized> AsMut<T> for $name
            where
                Uuid: AsMut<T>,
            {
                fn as_mut(&mut self) -> &mut T {
                    self.0.as_mut()
                }
            }

            impl AsRef<Self> for $name {
                fn as_ref(&self) -> &Self {
                    self
                }
            }

            impl<'a> From<&'a Self> for $name {
                fn from(id: &'a Self) -> Self {
                    id.clone()
                }
            }

            impl From<Uuid> for $name {
                fn from(uuid: Uuid) -> Self {
                    Self(uuid)
                }
            }

            impl fmt::Display for $name {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    fmt::Display::fmt(&self.0, f)
                }
            }
        )*
    }
}

uuid_id! {
    BlockId;
    DatabaseId;
    PageId;
    UserId;
}
