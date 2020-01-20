use super::defaults::*;
use super::messages::*;

use rstd::prelude::*;
use parity_codec::Codec;
use parity_codec_derive::{Encode, Decode};
use srml_support::{StorageMap, StorageValue, decl_module, decl_storage, decl_event, ensure, Parameter};
use runtime_primitives::traits::{SimpleArithmetic, As, Member, MaybeDebug, MaybeSerializeDebug};
use system::{self, ensure_signed};
use {timestamp};

pub trait Trait: system::Trait + timestamp::Trait + MaybeDebug {

  type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;

  type SpaceId: Parameter + Member + SimpleArithmetic + Codec + Default + Copy
    + As<usize> + As<u64> + MaybeSerializeDebug + PartialEq;

  type PostId: Parameter + Member + SimpleArithmetic + Codec + Default + Copy
    + As<usize> + As<u64> + MaybeSerializeDebug + PartialEq;

  type CommentId: Parameter + Member + SimpleArithmetic + Codec + Default + Copy
    + As<usize> + As<u64> + MaybeSerializeDebug + PartialEq;

  type ReactionId: Parameter + Member + SimpleArithmetic + Codec + Default + Copy
    + As<usize> + As<u64> + MaybeSerializeDebug + PartialEq;
}

// TODO: Rename WhoAndWhen, MutatedBy?
#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Clone, Encode, Decode, PartialEq)]
pub struct Change<T: Trait> {
  pub on_behalf: SpacedAccount<T>,
  pub block: T::BlockNumber,
  pub time: T::Moment,
}

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Clone, Copy, Encode, Decode, PartialEq, Eq)]
pub struct SpacedAccount<T: Trait> {
  pub account: T::AccountId,
  pub space: Option<T::SpaceId>,
}

// TODO add a schema along w/ JSON, maybe create a struct?

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Clone, Encode, Decode, PartialEq)]
pub struct Space<T: Trait> {
  pub id: T::SpaceId,
  pub created: Change<T>,
  pub updated: Option<Change<T>>,
  pub hidden: bool,

  // Can be updated by the owner:
  // pub owners: Vec<T::AccountId>,
  pub handle: Vec<u8>,

  pub ipfs_hash: Option<Vec<u8>>,
  pub edit_history: Vec<SpaceHistoryRecord<T>>,

  pub followers_count: u32,
  pub following_count: u16,
  pub posts_count: u32,

  pub score: i32,
}

impl <T: Trait> Space<T> {
  pub fn is_owner(self, account: T::AccountId) -> bool {
    self.created.on_behalf.account == account
  }
}

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Clone, Encode, Decode, PartialEq)]
pub struct SpaceUpdate {
  // pub owners: Option<Vec<T::AccountId>>,
  pub handle: Option<Vec<u8>>,
  pub ipfs_hash: Option<Vec<u8>>,
  pub hidden: Option<bool>,
}

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Clone, Encode, Decode, PartialEq)]
pub struct SpaceHistoryRecord<T: Trait> {
  pub edited: Change<T>,
  pub old_data: SpaceUpdate,
}

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Clone, Encode, Decode, PartialEq)]
pub struct Post<T: Trait> {
  pub id: T::PostId,
  pub space_id: T::SpaceId,
  pub created: Change<T>,
  pub updated: Option<Change<T>>,
  pub hidden: bool,
  pub extension: PostExtension<T>,

  // Next fields can be updated by the owner only:

  pub ipfs_hash: Vec<u8>,

  pub comments_count: u16,
  pub upvotes_count: u16,
  pub downvotes_count: u16,
  pub shares_count: u16,

  pub edit_history: Vec<PostHistoryRecord<T>>,

  pub score: i32,
}

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Clone, Encode, Decode, PartialEq)]
pub struct PostUpdate<T: Trait> {
  pub space_id: Option<T::SpaceId>,
  pub ipfs_hash: Option<Vec<u8>>,
  pub hidden: Option<bool>,
}

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Clone, Encode, Decode, PartialEq)]
pub struct PostHistoryRecord<T: Trait> {
  pub edited: Change<T>,
  pub old_data: PostUpdate<T>,
}

#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
#[derive(Clone, Copy, Encode, Decode, PartialEq, Eq)]
pub enum PostExtension<T: Trait> {
    RegularPost,
    SharedPost(T::PostId),
    SharedComment(T::CommentId),
}

impl <T: Trait> Default for PostExtension<T> {
    fn default() -> Self {
        PostExtension::RegularPost
    }
}

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Clone, Encode, Decode, PartialEq)]
pub struct Comment<T: Trait> {
  pub id: T::CommentId,
  pub parent_id: Option<T::CommentId>,
  pub post_id: T::PostId,
  pub created: Change<T>,
  pub updated: Option<Change<T>>,
  pub hidden: bool,

  // Can be updated by the owner:
  pub ipfs_hash: Vec<u8>,

  pub upvotes_count: u16,
  pub downvotes_count: u16,
  pub shares_count: u16,
  pub direct_replies_count: u16,

  pub edit_history: Vec<CommentHistoryRecord<T>>,

  pub score: i32,
}

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Clone, Encode, Decode, PartialEq)]
pub struct CommentUpdate {
  pub ipfs_hash: Option<Vec<u8>>,
  pub hidden: Option<bool>,
}

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Clone, Encode, Decode, PartialEq)]
pub struct CommentHistoryRecord<T: Trait> {
  pub edited: Change<T>,
  pub old_data: CommentUpdate,
}

#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
#[derive(Clone, Copy, Encode, Decode, PartialEq, Eq)]
pub enum ReactionKind {
    Upvote,
    Downvote,
}

impl Default for ReactionKind {
    fn default() -> Self {
        ReactionKind::Upvote
    }
}

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Clone, Encode, Decode, PartialEq)]
pub struct Reaction<T: Trait> {
  pub id: T::ReactionId,
  pub created: Change<T>,
  pub updated: Option<Change<T>>,
  pub kind: ReactionKind,
}

/*
#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
#[derive(Clone, Copy, Encode, Decode, PartialEq, Eq)]
pub enum ScoringAction {
  UpvotePost,
  DownvotePost,
  SharePost,
  CreateComment,
  UpvoteComment,
  DownvoteComment,
  ShareComment,
  FollowSpace,
}

impl Default for ScoringAction {
  fn default() -> Self {
    ScoringAction::FollowSpace
  }
}
*/

decl_storage! {
  trait Store for Module<T: Trait> as Spaces {

    pub HandleMinLen get(handle_min_len): u32 = DEFAULT_HANDLE_MIN_LEN;
    pub HandleMaxLen get(handle_max_len): u32 = DEFAULT_HANDLE_MAX_LEN;

    pub IpfsHashLen get(ipfs_hash_len): u32 = DEFAULT_IPFS_HASH_LEN;

    pub SpaceMaxLen get(space_max_len): u32 = DEFAULT_SPACE_MAX_LEN;
    pub PostMaxLen get(post_max_len): u32 = DEFAULT_POST_MAX_LEN;
    pub CommentMaxLen get(comment_max_len): u32 = DEFAULT_COMMENT_MAX_LEN;

    /*
    pub UpvotePostActionWeight get (upvote_post_action_weight): i16 = DEFAULT_UPVOTE_POST_ACTION_WEIGHT;
    pub DownvotePostActionWeight get (downvote_post_action_weight): i16 = DEFAULT_DOWNVOTE_POST_ACTION_WEIGHT;
    pub SharePostActionWeight get (share_post_action_weight): i16 = DEFAULT_SHARE_POST_ACTION_WEIGHT;
    pub CreateCommentActionWeight get (create_comment_action_weight): i16 = DEFAULT_CREATE_COMMENT_ACTION_WEIGHT;
    pub UpvoteCommentActionWeight get (upvote_comment_action_weight): i16 = DEFAULT_UPVOTE_COMMENT_ACTION_WEIGHT;
    pub DownvoteCommentActionWeight get (downvote_comment_action_weight): i16 = DEFAULT_DOWNVOTE_COMMENT_ACTION_WEIGHT;
    pub ShareCommentActionWeight get (share_comment_action_weight): i16 = DEFAULT_SHARE_COMMENT_ACTION_WEIGHT;
    pub FollowSpaceActionWeight get (follow_space_action_weight): i16 = DEFAULT_FOLLOW_SPACE_ACTION_WEIGHT;
    */

    pub SpaceById get(space_by_id): map T::SpaceId => Option<Space<T>>;
    pub PostById get(post_by_id): map T::PostId => Option<Post<T>>;
    pub CommentById get(comment_by_id): map T::CommentId => Option<Comment<T>>;
    pub ReactionById get(reaction_by_id): map T::ReactionId => Option<Reaction<T>>;

    pub PostIdsBySpaceId get(post_ids_by_space_id): map T::SpaceId => Vec<T::PostId>;
    pub CommentIdsByPostId get(comment_ids_by_post_id): map T::PostId => Vec<T::CommentId>;

    pub ReactionIdsByPostId get(reaction_ids_by_post_id): map T::PostId => Vec<T::ReactionId>;
    pub ReactionIdsByCommentId get(reaction_ids_by_comment_id): map T::CommentId => Vec<T::ReactionId>;
    pub PostReactionIdByAccount get(post_reaction_id_by_account): map (SpacedAccount<T>, T::PostId) => T::ReactionId;
    pub CommentReactionIdByAccount get(comment_reaction_id_by_account): map (SpacedAccount<T>, T::CommentId) => T::ReactionId;

    pub SpaceIdByHandle get(space_id_by_handle): map Vec<u8> => Option<T::SpaceId>;
    pub SpaceIdsByOwner get(space_ids_by_owner): map T::AccountId => Vec<T::SpaceId>;

    pub SpaceFollowers get(space_followers): map T::SpaceId => Vec<T::SpaceId>;
    pub SpacesFollowedBySpace get(spaces_followed_by_space): map T::SpaceId => Vec<T::SpaceId>;
    pub SpaceFollowedBySpace get(space_followed_by_space): map (T::SpaceId, T::SpaceId) => bool;

    pub NextSpaceId get(next_space_id): T::SpaceId = T::SpaceId::sa(1);
    pub NextPostId get(next_post_id): T::PostId = T::PostId::sa(1);
    pub NextCommentId get(next_comment_id): T::CommentId = T::CommentId::sa(1);
    pub NextReactionId get(next_reaction_id): T::ReactionId = T::ReactionId::sa(1);

    // pub AccountReputationDiffByAccount get(account_reputation_diff_by_account): map (T::AccountId, T::AccountId, ScoringAction) => Option<i16>; // TODO shorten name (?refactor)
    // pub PostScoreByAccount get(post_score_by_account): map (T::AccountId, T::PostId, ScoringAction) => Option<i16>;
    // pub CommentScoreByAccount get(comment_score_by_account): map (T::AccountId, T::CommentId, ScoringAction) => Option<i16>;

    pub PostSharesByAccount get(post_shares_by_account): map (T::AccountId, T::PostId) => u16;
    pub SharedPostIdsByOriginalPostId get(shared_post_ids_by_original_post_id): map T::PostId => Vec<T::PostId>;

    pub CommentSharesByAccount get(comment_shares_by_account): map (T::AccountId, T::CommentId) => u16;
    pub SharedPostIdsByOriginalCommentId get(shared_post_ids_by_original_comment_id): map T::CommentId => Vec<T::PostId>;
  }
}

decl_event! {
  pub enum Event<T> where
    SpacedAccount = SpacedAccount<T>,
    <T as Trait>::SpaceId,
    <T as Trait>::PostId,
    <T as Trait>::CommentId,
    <T as Trait>::ReactionId
  {
    SpaceCreated(SpacedAccount, SpaceId),
    SpaceUpdated(SpacedAccount, SpaceId),
    SpaceDeleted(SpacedAccount, SpaceId),

    SpaceFollowed(SpacedAccount, SpaceId),
    SpaceUnfollowed(SpacedAccount, SpaceId),

    // AccountReputationChanged(AccountId, ScoringAction, u32),

    PostCreated(SpacedAccount, PostId),
    PostUpdated(SpacedAccount, PostId),
    PostDeleted(SpacedAccount, PostId),
    PostShared(SpacedAccount, PostId),

    CommentCreated(SpacedAccount, CommentId),
    CommentUpdated(SpacedAccount, CommentId),
    CommentDeleted(SpacedAccount, CommentId),
    CommentShared(SpacedAccount, CommentId),

    PostReactionCreated(SpacedAccount, PostId, ReactionId),
    PostReactionUpdated(SpacedAccount, PostId, ReactionId),
    PostReactionDeleted(SpacedAccount, PostId, ReactionId),

    CommentReactionCreated(SpacedAccount, CommentId, ReactionId),
    CommentReactionUpdated(SpacedAccount, CommentId, ReactionId),
    CommentReactionDeleted(SpacedAccount, CommentId, ReactionId),
  }
}

decl_module! {
  pub struct Module<T: Trait> for enum Call where origin: T::Origin {

    pub fn deposit_event<T>() = default;

    fn on_initialize(_now: T::BlockNumber) {
      // Stub
    }

    fn on_finalize(_now: T::BlockNumber) {
      // Stub
    }

    // TODO use SpaceUpdate to pass data
    pub fn create_space(origin, on_behalf: Option<T::SpaceId>, handle: Vec<u8>, ipfs_hash: Option<Vec<u8>>) {
      let owner = ensure_signed(origin)?;

      Self::is_space_handle_valid(handle.clone())?;
      if let Some(ipfs_hash_unwrapped) = ipfs_hash.clone() {
        Self::is_ipfs_hash_valid(ipfs_hash_unwrapped)?;
      }

      let space_id = Self::next_space_id();
      let spaced_account = Self::new_spaced_account(owner.clone(), on_behalf)?;

      let ref mut new_space: Space<T> = Space {
        id: space_id,
        created: Self::new_change(spaced_account.clone()),
        updated: None,
        hidden: false,
        // owners: vec![],
        handle: handle.clone(),
        ipfs_hash,
        edit_history: vec![],
        followers_count: 0,
        following_count: 0,
        posts_count: 0,
        score: 0
      };

      // Space creator automatically follows their space:
      Self::add_space_follower_and_insert_space(spaced_account, new_space, true)?;

      <SpaceIdsByOwner<T>>::mutate(owner.clone(), |ids| ids.push(space_id));
      <SpaceIdByHandle<T>>::insert(handle, space_id);
      <NextSpaceId<T>>::mutate(|n| { *n += T::SpaceId::sa(1); });
    }

    pub fn follow_space(origin, on_behalf: T::SpaceId, space_id: T::SpaceId) {
      let follower = ensure_signed(origin)?;

      let ref mut space = Self::space_by_id(space_id).ok_or(MSG_SPACE_NOT_FOUND)?;
      ensure!(!Self::space_followed_by_space((on_behalf, space_id)), MSG_ACCOUNT_IS_FOLLOWING_SPACE);

      Self::add_space_follower_and_insert_space(Self::new_spaced_account(follower.clone(), Some(on_behalf))?, space, false)?;
    }

    pub fn unfollow_space(origin, on_behalf: T::SpaceId, space_id: T::SpaceId) {
      let follower = ensure_signed(origin)?;

      let ref mut space = Self::space_by_id(space_id).ok_or(MSG_SPACE_NOT_FOUND)?;
      let mut follower_space = Self::space_by_id(on_behalf).ok_or(MSG_ON_BEHALF_SPACE_NOT_FOUND)?;
      ensure!(Self::space_followed_by_space((on_behalf, space_id)), MSG_ACCOUNT_IS_NOT_FOLLOWING_SPACE);

      follower_space.following_count = follower_space.following_count
        .checked_sub(1)
        .ok_or(MSG_UNDERFLOW_UNFOLLOWING_SPACE)?;
      space.followers_count = space.followers_count.checked_sub(1).ok_or(MSG_UNDERFLOW_UNFOLLOWING_SPACE)?;

      let spaced_account = Self::new_spaced_account(follower.clone(), Some(on_behalf))?;

      // if space.created.account != follower {
      //   let author = space.created.account.clone();
      //   if let Some(score_diff) = Self::account_reputation_diff_by_account((follower.clone(), author.clone(), ScoringAction::FollowSpace)) {
      //     space.score = space.score.checked_sub(score_diff as i32).ok_or(MSG_OUT_OF_BOUNDS_UPDATING_SPACE_SCORE)?;
      //     Self::change_social_account_reputation(author.clone(), follower.clone(), score_diff * -1, ScoringAction::FollowSpace)?;
      //   }
      // }

      <SpacesFollowedBySpace<T>>::mutate(on_behalf, |space_ids| Self::vec_remove_on(space_ids, space_id));
      <SpaceFollowers<T>>::mutate(space_id, |account_ids| Self::vec_remove_on(account_ids, on_behalf));
      <SpaceFollowedBySpace<T>>::remove((on_behalf, space_id));
      <SpaceById<T>>::insert(on_behalf, follower_space);
      <SpaceById<T>>::insert(space_id, space);

      Self::deposit_event(RawEvent::SpaceUnfollowed(spaced_account, space_id));
    }

    // TODO use PostUpdate to pass data?
    pub fn create_post(origin, on_behalf: Option<T::SpaceId>, space_id: T::SpaceId, ipfs_hash: Vec<u8>, extension: PostExtension<T>) {
      let owner = ensure_signed(origin)?;

      let mut space = Self::space_by_id(space_id).ok_or(MSG_SPACE_NOT_FOUND)?;
      Self::ensure_account_is_space_owner(owner.clone(), space_id)?;

      let new_post_id = Self::next_post_id();

      // Sharing functions contain check for post/comment existance
      match extension {
        PostExtension::RegularPost => {
          Self::is_ipfs_hash_valid(ipfs_hash.clone())?;
        },
        PostExtension::SharedPost(post_id) => {
          let post = Self::post_by_id(post_id).ok_or(MSG_ORIGINAL_POST_NOT_FOUND)?;
          ensure!(post.extension == PostExtension::RegularPost, MSG_CANNOT_SHARE_SHARED_POST);
          Self::share_post(owner.clone(), on_behalf, post_id, new_post_id)?;
        },
        PostExtension::SharedComment(comment_id) => {
          Self::share_comment(owner.clone(), on_behalf, comment_id, new_post_id)?;
        },
      }

      let spaced_account = Self::new_spaced_account(owner.clone(), on_behalf)?;
      let new_post: Post<T> = Post {
        id: new_post_id,
        space_id,
        created: Self::new_change(spaced_account.clone()),
        updated: None,
        extension,
        hidden: false,
        ipfs_hash,
        comments_count: 0,
        upvotes_count: 0,
        downvotes_count: 0,
        shares_count: 0,
        edit_history: vec![],
        score: 0,
      };

      space.posts_count = space.posts_count.checked_add(1).ok_or(MSG_OVERFLOW_ADDING_POST_ON_SPACE)?;
      
      <PostById<T>>::insert(new_post_id, new_post);
      <PostIdsBySpaceId<T>>::mutate(space_id, |ids| ids.push(new_post_id));
      <NextPostId<T>>::mutate(|n| { *n += T::PostId::sa(1); });
      <SpaceById<T>>::insert(space_id, space);

      Self::deposit_event(RawEvent::PostCreated(spaced_account, new_post_id));
    }

    // TODO use CommentUpdate to pass data?
    pub fn create_comment(origin, on_behalf: Option<T::SpaceId>, post_id: T::PostId, parent_id: Option<T::CommentId>, ipfs_hash: Vec<u8>) {
      let owner = ensure_signed(origin)?;

      let ref mut post = Self::post_by_id(post_id).ok_or(MSG_POST_NOT_FOUND)?;
      let spaced_account = Self::new_spaced_account(owner.clone(), on_behalf)?;
      Self::is_ipfs_hash_valid(ipfs_hash.clone())?;

      let comment_id = Self::next_comment_id();
      let new_comment: Comment<T> = Comment {
        id: comment_id,
        parent_id,
        post_id,
        created: Self::new_change(spaced_account.clone()),
        updated: None,
        hidden: false,
        ipfs_hash,
        upvotes_count: 0,
        downvotes_count: 0,
        shares_count: 0,
        direct_replies_count: 0,
        edit_history: vec![],
        score: 0,
      };

      post.comments_count = post.comments_count.checked_add(1).ok_or(MSG_OVERFLOW_ADDING_COMMENT_ON_POST)?;

      // Self::change_post_score(owner.clone(), post, ScoringAction::CreateComment)?;

      if let Some(id) = parent_id {
        let mut parent_comment = Self::comment_by_id(id).ok_or(MSG_UNKNOWN_PARENT_COMMENT)?;
        parent_comment.direct_replies_count = parent_comment.direct_replies_count.checked_add(1).ok_or(MSG_OVERFLOW_REPLYING_ON_COMMENT)?;
        <CommentById<T>>::insert(id, parent_comment);
      }

      <CommentById<T>>::insert(comment_id, new_comment);
      <CommentIdsByPostId<T>>::mutate(post_id, |ids| ids.push(comment_id));
      <NextCommentId<T>>::mutate(|n| { *n += T::CommentId::sa(1); });
      <PostById<T>>::insert(post_id, post);

      Self::deposit_event(RawEvent::CommentCreated(spaced_account, comment_id));
    }

    pub fn create_post_reaction(origin, on_behalf: Option<T::SpaceId>, post_id: T::PostId, kind: ReactionKind) {
      let owner = ensure_signed(origin)?;

      let spaced_account = Self::new_spaced_account(owner.clone(), on_behalf)?;
      ensure!(
        !<PostReactionIdByAccount<T>>::exists((spaced_account.clone(), post_id)),
        MSG_ACCOUNT_ALREADY_REACTED_TO_POST
      );

      let ref mut post = Self::post_by_id(post_id).ok_or(MSG_POST_NOT_FOUND)?;
      let reaction_id = Self::new_reaction(spaced_account.clone(), kind.clone());
      // let action: ScoringAction;

      match kind {
        ReactionKind::Upvote => {
          post.upvotes_count = post.upvotes_count.checked_add(1).ok_or(MSG_OVERFLOW_UPVOTING_POST)?;
          // action = ScoringAction::UpvotePost;
        },
        ReactionKind::Downvote => {
          post.downvotes_count = post.downvotes_count.checked_add(1).ok_or(MSG_OVERFLOW_DOWNVOTING_POST)?;
          // action = ScoringAction::DownvotePost;
        },
      }

      // if post.created.account != owner {
      //   Self::change_post_score(owner.clone(), post, action)?;
      // }
      // else {
      <PostById<T>>::insert(post_id, post);
      // }

      <ReactionIdsByPostId<T>>::mutate(post_id, |ids| ids.push(reaction_id));
      <PostReactionIdByAccount<T>>::insert((spaced_account.clone(), post_id), reaction_id);

      Self::deposit_event(RawEvent::PostReactionCreated(spaced_account, post_id, reaction_id));
    }

    pub fn create_comment_reaction(origin, on_behalf: Option<T::SpaceId>, comment_id: T::CommentId, kind: ReactionKind) {
      let owner = ensure_signed(origin)?;

      let spaced_account = Self::new_spaced_account(owner.clone(), on_behalf)?;
      ensure!(
        !<CommentReactionIdByAccount<T>>::exists((spaced_account.clone(), comment_id)),
        MSG_ACCOUNT_ALREADY_REACTED_TO_COMMENT
      );

      let ref mut comment = Self::comment_by_id(comment_id).ok_or(MSG_COMMENT_NOT_FOUND)?;
      let reaction_id = Self::new_reaction(spaced_account.clone(), kind.clone());
      // let action: ScoringAction;

      match kind {
        ReactionKind::Upvote => {
          comment.upvotes_count = comment.upvotes_count.checked_add(1).ok_or(MSG_OVERFLOW_UPVOTING_COMMENT)?;
          // action = ScoringAction::UpvoteComment;
        },
        ReactionKind::Downvote => {
          comment.downvotes_count = comment.downvotes_count.checked_add(1).ok_or(MSG_OVERFLOW_DOWNVOTING_COMMENT)?;
          // action = ScoringAction::DownvoteComment;
        },
      }
      // if comment.created.account != owner {
      //   Self::change_comment_score(owner.clone(), comment, action)?;
      // }
      // else {
      <CommentById<T>>::insert(comment_id, comment);
      // }

      <ReactionIdsByCommentId<T>>::mutate(comment_id, |ids| ids.push(reaction_id));
      <CommentReactionIdByAccount<T>>::insert((spaced_account.clone(), comment_id), reaction_id);

      Self::deposit_event(RawEvent::CommentReactionCreated(spaced_account, comment_id, reaction_id));
    }

    pub fn update_space(origin, on_behalf: Option<T::SpaceId>, space_id: T::SpaceId, update: SpaceUpdate) {
      let owner = ensure_signed(origin)?;
      
      let spaced_account = Self::new_spaced_account(owner.clone(), on_behalf)?;
      let has_updates = 
        // update.writers.is_some() ||
        update.handle.is_some() ||
        update.ipfs_hash.is_some() ||
        update.hidden.is_some();

      ensure!(has_updates, MSG_NOTHING_TO_UPDATE_IN_SPACE);

      let mut space = Self::space_by_id(space_id).ok_or(MSG_SPACE_NOT_FOUND)?;

      // TODO ensure: space writers also should be able to edit this space:
      ensure!(spaced_account == space.created.on_behalf, MSG_ONLY_SPACE_OWNER_CAN_UPDATE_SPACE);

      let mut fields_updated = 0;
      let mut new_history_record = SpaceHistoryRecord {
        edited: Self::new_change(spaced_account.clone()),
        old_data: SpaceUpdate {/*writers: None, */handle: None, ipfs_hash: None, hidden: None}
      };

      /*
      if let Some(writers) = update.writers {
        if writers != space.writers {
          // TODO validate writers.
          // TODO update SpaceIdsByWriter: insert new, delete removed, update only changed writers.
          new_history_record.old_data.writers = Some(space.writers);
          space.writers = writers;
          fields_updated += 1;
        }
      }
      */

      if let Some(hidden) = update.hidden {
        if hidden != space.hidden {
          new_history_record.old_data.hidden = Some(space.hidden);
          space.hidden = hidden;
          fields_updated += 1;
        }
      }

      if let Some(ipfs_hash) = update.ipfs_hash {
        if Some(ipfs_hash.clone()) != space.ipfs_hash {
          Self::is_ipfs_hash_valid(ipfs_hash.clone())?;
          new_history_record.old_data.ipfs_hash = space.ipfs_hash;
          space.ipfs_hash = Some(ipfs_hash);
          fields_updated += 1;
        }
      }

      if let Some(handle) = update.handle {
        if handle != space.handle {
          let handle_len = handle.len();
          ensure!(handle_len >= Self::handle_min_len() as usize, MSG_SPACE_HANDLE_IS_TOO_SHORT);
          ensure!(handle_len <= Self::handle_max_len() as usize, MSG_SPACE_HANDLE_IS_TOO_LONG);
          ensure!(!<SpaceIdByHandle<T>>::exists(handle.clone()), MSG_SPACE_HANDLE_IS_NOT_UNIQUE);

          <SpaceIdByHandle<T>>::remove(space.handle.clone());
          <SpaceIdByHandle<T>>::insert(handle.clone(), space_id);
          new_history_record.old_data.handle = Some(space.handle);
          space.handle = handle;
          fields_updated += 1;
        }
      }

      // Update this space only if at least one field should be updated:
      if fields_updated > 0 {
        space.updated = Some(Self::new_change(spaced_account.clone()));
        space.edit_history.push(new_history_record);
        <SpaceById<T>>::insert(space_id, space);
        Self::deposit_event(RawEvent::SpaceUpdated(spaced_account, space_id));
      }
    }
    
    pub fn update_post(origin, on_behalf: Option<T::SpaceId>, post_id: T::PostId, update: PostUpdate<T>) {
      let owner = ensure_signed(origin)?;
      
      let spaced_account = Self::new_spaced_account(owner.clone(), on_behalf)?;
      let has_updates = 
        update.space_id.is_some() ||
        update.ipfs_hash.is_some() ||
        update.hidden.is_some();

      ensure!(has_updates, MSG_NOTHING_TO_UPDATE_IN_POST);

      let mut post = Self::post_by_id(post_id).ok_or(MSG_POST_NOT_FOUND)?;

      // TODO ensure: space writers also should be able to edit this post:
      ensure!(spaced_account == post.created.on_behalf, MSG_ONLY_POST_OWNER_CAN_UPDATE_POST);

      let mut fields_updated = 0;
      let mut new_history_record = PostHistoryRecord {
        edited: Self::new_change(spaced_account.clone()),
        old_data: PostUpdate {space_id: None, ipfs_hash: None, hidden: None}
      };

      if let Some(ipfs_hash) = update.ipfs_hash {
        if ipfs_hash != post.ipfs_hash {
          Self::is_ipfs_hash_valid(ipfs_hash.clone())?;
          new_history_record.old_data.ipfs_hash = Some(post.ipfs_hash);
          post.ipfs_hash = ipfs_hash;
          fields_updated += 1;
        }
      }

      if let Some(hidden) = update.hidden {
        if hidden != post.hidden {
          new_history_record.old_data.hidden = Some(post.hidden);
          post.hidden = hidden;
          fields_updated += 1;
        }
      }

      // Move this post to another space:
      if let Some(space_id) = update.space_id {
        if space_id != post.space_id {
          Self::ensure_space_exists(space_id)?;
          
          // Remove post_id from its old space:
          <PostIdsBySpaceId<T>>::mutate(post.space_id, |post_ids| Self::vec_remove_on(post_ids, post_id));
          
          // Add post_id to its new space:
          <PostIdsBySpaceId<T>>::mutate(space_id.clone(), |ids| ids.push(post_id));
          new_history_record.old_data.space_id = Some(post.space_id);
          post.space_id = space_id;
          fields_updated += 1;
        }
      }

      // Update this post only if at least one field should be updated:
      if fields_updated > 0 {
        post.updated = Some(Self::new_change(spaced_account.clone()));
        post.edit_history.push(new_history_record);
        <PostById<T>>::insert(post_id, post);

        Self::deposit_event(RawEvent::PostUpdated(spaced_account, post_id));
      }
    }
    
    pub fn update_comment(origin, on_behalf: Option<T::SpaceId>, comment_id: T::CommentId, update: CommentUpdate) {
      let owner = ensure_signed(origin)?;

      let spaced_account = Self::new_spaced_account(owner.clone(), on_behalf)?;
      let has_updates = 
        update.ipfs_hash.is_some() ||
        update.hidden.is_some();
      
      ensure!(has_updates, MSG_NOTHING_TO_UPDATE_IN_COMMENT);

      let mut comment = Self::comment_by_id(comment_id).ok_or(MSG_COMMENT_NOT_FOUND)?;

      // TODO: Make is_owner in impl
      ensure!(owner.clone() == comment.created.on_behalf.account, MSG_ONLY_COMMENT_AUTHOR_CAN_UPDATE_COMMENT);

      let mut fields_updated = 0;
      let mut new_history_record = CommentHistoryRecord {
        edited: Self::new_change(spaced_account.clone()),
        old_data: CommentUpdate {ipfs_hash: None, hidden: None}
      };

      if let Some(ipfs_hash) = update.ipfs_hash {
        if ipfs_hash != comment.ipfs_hash {
          Self::is_ipfs_hash_valid(ipfs_hash.clone())?;
          new_history_record.old_data.ipfs_hash = Some(comment.ipfs_hash);
          comment.ipfs_hash = ipfs_hash;
          fields_updated += 1;
        }
      }

      if let Some(hidden) = update.hidden {
        if hidden != comment.hidden {
          new_history_record.old_data.hidden = Some(comment.hidden);
          comment.hidden = hidden;
          fields_updated += 1;
        }
      }

      if fields_updated > 0 {
        comment.updated = Some(Self::new_change(spaced_account.clone()));
        comment.edit_history.push(new_history_record);
        <CommentById<T>>::insert(comment_id, comment);

        Self::deposit_event(RawEvent::CommentUpdated(spaced_account, comment_id));
      }
    }

    pub fn update_post_reaction(origin, on_behalf: Option<T::SpaceId>, post_id: T::PostId, reaction_id: T::ReactionId, new_kind: ReactionKind) {
      let owner = ensure_signed(origin)?;

      let spaced_account = Self::new_spaced_account(owner.clone(), on_behalf)?;
      ensure!(
        <PostReactionIdByAccount<T>>::exists((spaced_account.clone(), post_id)),
        MSG_ACCOUNT_HAS_NOT_REACTED_TO_POST
      );

      let mut reaction = Self::reaction_by_id(reaction_id).ok_or(MSG_REACTION_NOT_FOUND)?;
      let ref mut post = Self::post_by_id(post_id).ok_or(MSG_POST_NOT_FOUND)?;

      ensure!(spaced_account == reaction.created.on_behalf, MSG_ONLY_REACTION_OWNER_CAN_UPDATE_REACTION);
      ensure!(reaction.kind != new_kind, MSG_NEW_REACTION_KIND_DO_NOT_DIFFER);

      reaction.kind = new_kind;
      reaction.updated = Some(Self::new_change(spaced_account.clone()));
      // let action: ScoringAction;
      // let action_to_cancel: ScoringAction;
      
      match new_kind {
        ReactionKind::Upvote => {
          post.upvotes_count += 1;
          post.downvotes_count -= 1;
          // action_to_cancel = ScoringAction::DownvotePost;
          // action = ScoringAction::UpvotePost;
        },
        ReactionKind::Downvote => {
          post.downvotes_count += 1;
          post.upvotes_count -= 1;
          // action_to_cancel = ScoringAction::UpvotePost;
          // action = ScoringAction::DownvotePost;
        },
      }
      // Self::change_post_score(owner.clone(), post, action_to_cancel)?;
      // Self::change_post_score(owner.clone(), post, action)?;

      <ReactionById<T>>::insert(reaction_id, reaction);
      <PostById<T>>::insert(post_id, post);

      Self::deposit_event(RawEvent::PostReactionUpdated(spaced_account, post_id, reaction_id));
    }

    pub fn update_comment_reaction(origin, on_behalf: Option<T::SpaceId>, comment_id: T::CommentId, reaction_id: T::ReactionId, new_kind: ReactionKind) {
      let owner = ensure_signed(origin)?;

      let spaced_account = Self::new_spaced_account(owner.clone(), on_behalf)?;
      ensure!(
        <CommentReactionIdByAccount<T>>::exists((spaced_account.clone(), comment_id)),
        MSG_ACCOUNT_HAS_NOT_REACTED_TO_COMMENT
      );

      let mut reaction = Self::reaction_by_id(reaction_id).ok_or(MSG_REACTION_NOT_FOUND)?;
      let ref mut comment = Self::comment_by_id(comment_id).ok_or(MSG_COMMENT_NOT_FOUND)?;

      ensure!(spaced_account == reaction.created.on_behalf, MSG_ONLY_REACTION_OWNER_CAN_UPDATE_REACTION);
      ensure!(reaction.kind != new_kind, MSG_NEW_REACTION_KIND_DO_NOT_DIFFER);

      reaction.kind = new_kind;
      reaction.updated = Some(Self::new_change(spaced_account.clone()));
      // let action: ScoringAction;
      // let action_to_cancel: ScoringAction;
      
      match new_kind {
        ReactionKind::Upvote => {
          comment.upvotes_count += 1;
          comment.downvotes_count -= 1;
          // action_to_cancel = ScoringAction::DownvoteComment;
          // action = ScoringAction::UpvoteComment;
        },
        ReactionKind::Downvote => {
          comment.downvotes_count += 1;
          comment.upvotes_count -= 1;
          // action_to_cancel = ScoringAction::UpvoteComment;
          // action = ScoringAction::DownvoteComment;
        },
      }
      // Self::change_comment_score(owner.clone(), comment, action_to_cancel)?;
      // Self::change_comment_score(owner.clone(), comment, action)?;

      <ReactionById<T>>::insert(reaction_id, reaction);
      <CommentById<T>>::insert(comment_id, comment);

      Self::deposit_event(RawEvent::CommentReactionUpdated(spaced_account, comment_id, reaction_id));
    }

    // TODO fn delete_space(origin, space_id: T::SpaceId) {
      // TODO only owner can delete
      // TODO unfollow all space followers
    // }
    
    // TODO fn delete_post(origin, post_id: T::PostId) {}
    
    // TODO fn delete_comment(origin, comment_id: T::CommentId) {}

    pub fn delete_post_reaction(origin, on_behalf: Option<T::SpaceId>, post_id: T::PostId, reaction_id: T::ReactionId) {
      let owner = ensure_signed(origin)?;

      let spaced_account = Self::new_spaced_account(owner.clone(), on_behalf)?;
      ensure!(
        <PostReactionIdByAccount<T>>::exists((spaced_account.clone(), post_id)),
        MSG_NO_POST_REACTION_BY_ACCOUNT_TO_DELETE
      );
      
      // let action_to_cancel: ScoringAction;
      let reaction = Self::reaction_by_id(reaction_id).ok_or(MSG_REACTION_NOT_FOUND)?;
      let ref mut post = Self::post_by_id(post_id).ok_or(MSG_POST_NOT_FOUND)?;

      ensure!(spaced_account == reaction.created.on_behalf, MSG_ONLY_REACTION_OWNER_CAN_UPDATE_REACTION);

      match reaction.kind {
        ReactionKind::Upvote => {
          post.upvotes_count -= 1;
          // action_to_cancel = ScoringAction::UpvotePost;
        },
        ReactionKind::Downvote => {
          post.downvotes_count -= 1;
          // action_to_cancel = ScoringAction::DownvotePost;

        },
      }
      // Self::change_post_score(owner.clone(), post, action_to_cancel)?;

      <PostById<T>>::insert(post_id, post);
      <ReactionById<T>>::remove(reaction_id);
      <ReactionIdsByPostId<T>>::mutate(post_id, |ids| Self::vec_remove_on(ids, reaction_id));
      <PostReactionIdByAccount<T>>::remove((spaced_account.clone(), post_id));

      Self::deposit_event(RawEvent::PostReactionDeleted(spaced_account, post_id, reaction_id));
    }

    pub fn delete_comment_reaction(origin, on_behalf: Option<T::SpaceId>, comment_id: T::CommentId, reaction_id: T::ReactionId) {
      let owner = ensure_signed(origin)?;

      let spaced_account = Self::new_spaced_account(owner.clone(), on_behalf)?;
      ensure!(
        <CommentReactionIdByAccount<T>>::exists((spaced_account.clone(), comment_id)),
        MSG_NO_COMMENT_REACTION_BY_ACCOUNT_TO_DELETE
      );
      
      // let action_to_cancel: ScoringAction;
      let reaction = Self::reaction_by_id(reaction_id).ok_or(MSG_REACTION_NOT_FOUND)?;
      let ref mut comment = Self::comment_by_id(comment_id).ok_or(MSG_COMMENT_NOT_FOUND)?;
      
      ensure!(spaced_account == reaction.created.on_behalf, MSG_ONLY_REACTION_OWNER_CAN_UPDATE_REACTION);

      match reaction.kind {
        ReactionKind::Upvote => {
          comment.upvotes_count -= 1;
          // action_to_cancel = ScoringAction::UpvoteComment
        },
        ReactionKind::Downvote => {
          comment.downvotes_count -= 1;
          // action_to_cancel = ScoringAction::DownvoteComment
        },
      }
      // Self::change_comment_score(owner.clone(), comment, action_to_cancel)?;

      <CommentById<T>>::insert(comment_id, comment);
      <ReactionIdsByCommentId<T>>::mutate(comment_id, |ids| Self::vec_remove_on(ids, reaction_id));
      <ReactionById<T>>::remove(reaction_id);
      <CommentReactionIdByAccount<T>>::remove((spaced_account.clone(), comment_id));

      Self::deposit_event(RawEvent::CommentReactionDeleted(spaced_account, comment_id, reaction_id));
    }

    // TODO spend some tokens on: create/update a space/post/comment.
  }
}
