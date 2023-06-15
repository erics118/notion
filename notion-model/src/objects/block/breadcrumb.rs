use serde::{Deserialize, Serialize};

use super::{BlockBuilder, BlockData};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Copy, Default)]
pub struct Breadcrumb {
    /// This is present so that serde serializes this into `{}` rather than as
    /// `null`.
    #[serde(skip)]
    _nothing: (),
}

impl Breadcrumb {
    pub fn builder() -> BreadcrumbBuilder {
        BreadcrumbBuilder(Self::default())
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Copy, Default)]
pub struct BreadcrumbBuilder(Breadcrumb);

impl BreadcrumbBuilder {
    pub fn build(&self) -> BlockBuilder {
        BlockBuilder::new(BlockData::Breadcrumb { breadcrumb: self.0 })
    }
}
