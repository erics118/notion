//! Notion IDs

use std::fmt;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

macro_rules! uuid_id {
    ($($name:ident;)*) => {
        $(

            #[derive(
                Copy, Clone, Default, Debug, Eq, Hash, PartialEq, PartialOrd, Ord, Deserialize, Serialize,
            )]
            pub struct $name(pub Uuid);

            impl $name {
                /// Immutably borrow inner Id.
                #[inline]
                #[must_use]
                pub fn as_uuid(&self) -> &Uuid {
                    &self.0
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
    PageId;
}
