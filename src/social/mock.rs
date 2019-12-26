#![cfg(test)]

pub use super::spaces;
pub use system;

pub use primitives::{H256, Blake2Hasher};
pub use runtime_primitives::{
  BuildStorage,
  traits::{BlakeTwo256, IdentityLookup},
  testing::{Digest, DigestItem, Header}
};

use srml_support::impl_outer_origin;

impl_outer_origin! {
  pub enum Origin for Test {}
}

pub type AccountId = u64;
pub type SpaceId = u32;
pub type PostId = u32;
pub type CommentId = u32;
pub type ReactionId = u32;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Test;
impl system::Trait for Test {
  type Origin = Origin;
  type Index = u64;
  type BlockNumber = u64;
  type Hash = H256;
  type Hashing = BlakeTwo256;
  type Digest = Digest;
  type AccountId = AccountId;
  type Header = Header;
  type Event = ();
  type Log = DigestItem;
  type Lookup = IdentityLookup<u64>;
}

impl timestamp::Trait for Test {
  type Moment = u64;
  type OnTimestampSet = ();
}

impl spaces::Trait for Test {
  type Event = ();
  type SpaceId = SpaceId;
  type PostId = PostId;
  type CommentId = CommentId;
  type ReactionId = ReactionId;
}

pub fn build_ext() -> runtime_io::TestExternalities<Blake2Hasher> {
  let t = system::GenesisConfig::<Test>::default()
    .build_storage()
    .unwrap()
    .0;

  t.into()
}

pub type Spaces = spaces::Module<Test>;