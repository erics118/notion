use crate::objects::block::Block;

pub trait BlockBuilderTrait {
    fn build_block(&self) -> Block;
}
