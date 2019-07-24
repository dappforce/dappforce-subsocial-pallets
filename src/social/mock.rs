#![cfg(test)]

pub use super::blogs;
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

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Test;
impl system::Trait for Test {
  type Origin = Origin;
  type Index = u64;
  type BlockNumber = u64;
  type Hash = H256;
  type Hashing = BlakeTwo256;
  type Digest = Digest;
  type AccountId = u64;
  type Header = Header;
  type Event = ();
  type Log = DigestItem;
  type Lookup = IdentityLookup<u64>;
}

impl timestamp::Trait for Test {
  type Moment = u64;
  type OnTimestampSet = ();
}

impl blogs::Trait for Test {
  type Event = ();
  type BlogId = u32;
  type PostId = u32;
  type CommentId = u32;
  type ReactionId = u32;
}

pub fn build_ext() -> runtime_io::TestExternalities<Blake2Hasher> {
  let t = system::GenesisConfig::<Test>::default()
    .build_storage()
    .unwrap()
    .0;

  t.into()
}

pub type Blogs = blogs::Module<Test>;