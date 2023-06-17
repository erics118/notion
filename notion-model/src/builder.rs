use crate::objects::block::Block;

pub trait BlockBuilderTrait {
    fn build(&self) -> Block;
}
