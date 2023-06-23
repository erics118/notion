use serde::{Deserialize, Serialize};

use super::{Block, BlockData};

/// # Breadcrumb block
///
/// Breadcrumb block objects do not contain any information.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Copy, Default)]
pub struct Breadcrumb {
    /// This is present so that serde serializes this into `{}` rather than as
    /// `null`.
    #[serde(skip)]
    _nothing: (),
}

impl Breadcrumb {
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn build(self) -> Block {
        Block::new(BlockData::Breadcrumb(self))
    }
}



#[cfg(test)]
mod test {
    use serde_test::{assert_tokens, Token};

    use super::*;

    #[test]
    fn empty() {
        let value = Breadcrumb::new().build();

        assert_tokens(
            &value,
            &[
                Token::Map { len: None },
                Token::Str("object"),
                Token::Str("block"),
                Token::Str("breadcrumb"),
                Token::Struct {
                    name: "Breadcrumb",
                    len: 0,
                },
                Token::StructEnd,
                Token::MapEnd,
            ],
        );
    }
}
