use crate::objects::block::BlockBuilder;

pub trait BlockBuilderTrait {
    fn build_block(&self) -> BlockBuilder;
}
