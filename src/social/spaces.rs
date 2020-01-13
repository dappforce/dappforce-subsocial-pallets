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

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Clone, Copy, Encode, Decode, PartialEq)]
pub struct Change<T: Trait> {
  pub account: T::AccountId,
  pub block: T::BlockNumber,
  pub time: T::Moment,
}

// TODO add a schema along w/ JSON, maybe create a struct?

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Clone, Encode, Decode, PartialEq)]
pub struct Space<T: Trait> {
  pub id: T::SpaceId,
  pub created: Change<T>,
  pub updated: Option<Change<T>>,

  // Can be updated by the owner:
  // pub owners: Vec<T::AccountId>,
  pub handle: Vec<u8>,

  pub ipfs_hash: Option<Vec<u8>>,
  pub edit_history: Vec<SpaceHistoryRecord<T>>,

  pub followers_count: u32,
  pub posts_count: u32,

  pub score: i32,
}

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Clone, Encode, Decode, PartialEq)]
pub struct SpaceUpdate {
  // pub owners: Option<Vec<T::AccountId>>,
  pub handle: Option<Vec<u8>>,
  pub ipfs_hash: Option<Vec<u8>>,
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
  pub ipfs_hash: Vec<u8>,
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

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Clone, Encode, Decode, PartialEq)]
pub struct SocialAccount<T: Trait> {
  pub followers_count: u32,
  pub following_accounts_count: u16,
  pub following_spaces_count: u16,
  pub reputation: u32,
  pub profile: Option<Profile<T>>,
}

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Clone, Encode, Decode, PartialEq)]
pub struct Profile<T: Trait> {
  pub created: Change<T>,
  pub updated: Option<Change<T>>,

  pub username: Vec<u8>,
  pub ipfs_hash: Vec<u8>,
  
  pub edit_history: Vec<ProfileHistoryRecord<T>>,
}

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Clone, Encode, Decode, PartialEq)]
pub struct ProfileUpdate {
  pub username: Option<Vec<u8>>,
  pub ipfs_hash: Option<Vec<u8>>,
}

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Clone, Encode, Decode, PartialEq)]
pub struct ProfileHistoryRecord<T: Trait> {
  pub edited: Change<T>,
  pub old_data: ProfileUpdate,
}

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
  FollowAccount,
}

impl Default for ScoringAction {
  fn default() -> Self {
    ScoringAction::FollowAccount
  }
}

decl_storage! {
  trait Store for Module<T: Trait> as Spaces {

    pub HandleMinLen get(handle_min_len): u32 = DEFAULT_HANDLE_MIN_LEN;
    pub HandleMaxLen get(handle_max_len): u32 = DEFAULT_HANDLE_MAX_LEN;

    pub IpfsHashLen get(ipfs_hash_len): u32 = DEFAULT_IPFS_HASH_LEN;

    pub UsernameMinLen get(username_min_len): u32 = DEFAULT_USERNAME_MIN_LEN;
    pub UsernameMaxLen get(username_max_len): u32 = DEFAULT_USERNAME_MAX_LEN;

    pub SpaceMaxLen get(space_max_len): u32 = DEFAULT_SPACE_MAX_LEN;
    pub PostMaxLen get(post_max_len): u32 = DEFAULT_POST_MAX_LEN;
    pub CommentMaxLen get(comment_max_len): u32 = DEFAULT_COMMENT_MAX_LEN;

    pub UpvotePostActionWeight get (upvote_post_action_weight): i16 = DEFAULT_UPVOTE_POST_ACTION_WEIGHT;
    pub DownvotePostActionWeight get (downvote_post_action_weight): i16 = DEFAULT_DOWNVOTE_POST_ACTION_WEIGHT;
    pub SharePostActionWeight get (share_post_action_weight): i16 = DEFAULT_SHARE_POST_ACTION_WEIGHT;
    pub CreateCommentActionWeight get (create_comment_action_weight): i16 = DEFAULT_CREATE_COMMENT_ACTION_WEIGHT;
    pub UpvoteCommentActionWeight get (upvote_comment_action_weight): i16 = DEFAULT_UPVOTE_COMMENT_ACTION_WEIGHT;
    pub DownvoteCommentActionWeight get (downvote_comment_action_weight): i16 = DEFAULT_DOWNVOTE_COMMENT_ACTION_WEIGHT;
    pub ShareCommentActionWeight get (share_comment_action_weight): i16 = DEFAULT_SHARE_COMMENT_ACTION_WEIGHT;
    pub FollowSpaceActionWeight get (follow_space_action_weight): i16 = DEFAULT_FOLLOW_SPACE_ACTION_WEIGHT;
    pub FollowAccountActionWeight get (follow_account_action_weight): i16 = DEFAULT_FOLLOW_ACCOUNT_ACTION_WEIGHT;

    pub SpaceById get(space_by_id): map T::SpaceId => Option<Space<T>>;
    pub PostById get(post_by_id): map T::PostId => Option<Post<T>>;
    pub CommentById get(comment_by_id): map T::CommentId => Option<Comment<T>>;
    pub ReactionById get(reaction_by_id): map T::ReactionId => Option<Reaction<T>>;
    pub SocialAccountById get(social_account_by_id): map T::AccountId => Option<SocialAccount<T>>;

    pub SpaceIdsByOwner get(space_ids_by_owner): map T::AccountId => Vec<T::SpaceId>;
    pub PostIdsBySpaceId get(post_ids_by_space_id): map T::SpaceId => Vec<T::PostId>;
    pub CommentIdsByPostId get(comment_ids_by_post_id): map T::PostId => Vec<T::CommentId>;

    pub ReactionIdsByPostId get(reaction_ids_by_post_id): map T::PostId => Vec<T::ReactionId>;
    pub ReactionIdsByCommentId get(reaction_ids_by_comment_id): map T::CommentId => Vec<T::ReactionId>;
    pub PostReactionIdByAccount get(post_reaction_id_by_account): map (T::AccountId, T::PostId) => T::ReactionId;
    pub CommentReactionIdByAccount get(comment_reaction_id_by_account): map (T::AccountId, T::CommentId) => T::ReactionId;

    pub SpaceIdByHandle get(space_id_by_handle): map Vec<u8> => Option<T::SpaceId>;

    pub SpacesFollowedByAccount get(spaces_followed_by_account): map T::AccountId => Vec<T::SpaceId>;
    pub SpaceFollowers get(space_followers): map T::SpaceId => Vec<T::AccountId>;
    pub SpaceFollowedByAccount get(space_followed_by_account): map (T::AccountId, T::SpaceId) => bool;

    pub AccountFollowedByAccount get(account_followed_by_account): map (T::AccountId, T::AccountId) => bool;
    pub AccountsFollowedByAccount get(accounts_followed_by_account): map T::AccountId => Vec<T::AccountId>;
    pub AccountFollowers get(account_followers): map T::AccountId => Vec<T::AccountId>;

    pub NextSpaceId get(next_space_id): T::SpaceId = T::SpaceId::sa(1);
    pub NextPostId get(next_post_id): T::PostId = T::PostId::sa(1);
    pub NextCommentId get(next_comment_id): T::CommentId = T::CommentId::sa(1);
    pub NextReactionId get(next_reaction_id): T::ReactionId = T::ReactionId::sa(1);

    pub AccountReputationDiffByAccount get(account_reputation_diff_by_account): map (T::AccountId, T::AccountId, ScoringAction) => Option<i16>; // TODO shorten name (?refactor)
    pub PostScoreByAccount get(post_score_by_account): map (T::AccountId, T::PostId, ScoringAction) => Option<i16>;
    pub CommentScoreByAccount get(comment_score_by_account): map (T::AccountId, T::CommentId, ScoringAction) => Option<i16>;

    pub PostSharesByAccount get(post_shares_by_account): map (T::AccountId, T::PostId) => u16;
    pub SharedPostIdsByOriginalPostId get(shared_post_ids_by_original_post_id): map T::PostId => Vec<T::PostId>;

    pub CommentSharesByAccount get(comment_shares_by_account): map (T::AccountId, T::CommentId) => u16;
    pub SharedPostIdsByOriginalCommentId get(shared_post_ids_by_original_comment_id): map T::CommentId => Vec<T::PostId>;

    pub AccountByProfileUsername get(account_by_profile_username): map Vec<u8> => Option<T::AccountId>;
  }
}

decl_event! {
  pub enum Event<T> where
    <T as system::Trait>::AccountId,
    <T as Trait>::SpaceId,
    <T as Trait>::PostId,
    <T as Trait>::CommentId,
    <T as Trait>::ReactionId
  {
    SpaceCreated(AccountId, SpaceId),
    SpaceUpdated(AccountId, SpaceId),
    SpaceDeleted(AccountId, SpaceId),

    SpaceFollowed(AccountId, SpaceId),
    SpaceUnfollowed(AccountId, SpaceId),

    AccountReputationChanged(AccountId, ScoringAction, u32),

    AccountFollowed(AccountId, AccountId),
    AccountUnfollowed(AccountId, AccountId),

    PostCreated(AccountId, PostId),
    PostUpdated(AccountId, PostId),
    PostDeleted(AccountId, PostId),
    PostShared(AccountId, PostId),

    CommentCreated(AccountId, CommentId),
    CommentUpdated(AccountId, CommentId),
    CommentDeleted(AccountId, CommentId),
    CommentShared(AccountId, CommentId),

    PostReactionCreated(AccountId, PostId, ReactionId),
    PostReactionUpdated(AccountId, PostId, ReactionId),
    PostReactionDeleted(AccountId, PostId, ReactionId),

    CommentReactionCreated(AccountId, CommentId, ReactionId),
    CommentReactionUpdated(AccountId, CommentId, ReactionId),
    CommentReactionDeleted(AccountId, CommentId, ReactionId),

    ProfileCreated(AccountId),
    ProfileUpdated(AccountId),
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
    pub fn create_space(origin, handle: Vec<u8>, ipfs_hash: Option<Vec<u8>>) {
      let owner = ensure_signed(origin)?;

      ensure!(handle.len() >= Self::handle_min_len() as usize, MSG_SPACE_HANDLE_IS_TOO_SHORT);
      ensure!(handle.len() <= Self::handle_max_len() as usize, MSG_SPACE_HANDLE_IS_TOO_LONG);
      ensure!(!<SpaceIdByHandle<T>>::exists(handle.clone()), MSG_SPACE_HANDLE_IS_NOT_UNIQUE);
      if let Some(ipfs_hash_unwrapped) = ipfs_hash.clone() {
        Self::is_ipfs_hash_valid(ipfs_hash_unwrapped)?;
      }

      let space_id = Self::next_space_id();
      let ref mut new_space: Space<T> = Space {
        id: space_id,
        created: Self::new_change(owner.clone()),
        updated: None,
        // owners: vec![],
        handle: handle.clone(),
        ipfs_hash: ipfs_hash,
        posts_count: 0,
        followers_count: 0,
        edit_history: vec![],
        score: 0
      };

      // Space creator automatically follows their space:
      Self::add_space_follower_and_insert_space(owner.clone(), new_space, true)?;

      <SpaceIdsByOwner<T>>::mutate(owner.clone(), |ids| ids.push(space_id));
      <SpaceIdByHandle<T>>::insert(handle, space_id);
      <NextSpaceId<T>>::mutate(|n| { *n += T::SpaceId::sa(1); });
    }

    pub fn follow_space(origin, space_id: T::SpaceId) {
      let follower = ensure_signed(origin)?;

      let ref mut space = Self::space_by_id(space_id).ok_or(MSG_SPACE_NOT_FOUND)?;
      ensure!(!Self::space_followed_by_account((follower.clone(), space_id)), MSG_ACCOUNT_IS_FOLLOWING_SPACE);

      Self::add_space_follower_and_insert_space(follower.clone(), space, false)?;
    }

    pub fn unfollow_space(origin, space_id: T::SpaceId) {
      let follower = ensure_signed(origin)?;

      let ref mut space = Self::space_by_id(space_id).ok_or(MSG_SPACE_NOT_FOUND)?;
      ensure!(Self::space_followed_by_account((follower.clone(), space_id)), MSG_ACCOUNT_IS_NOT_FOLLOWING_SPACE);

      let mut social_account = Self::social_account_by_id(follower.clone()).ok_or(MSG_SOCIAL_ACCOUNT_NOT_FOUND)?;
      social_account.following_spaces_count = social_account.following_spaces_count
        .checked_sub(1)
        .ok_or(MSG_UNDERFLOW_UNFOLLOWING_SPACE)?;
      space.followers_count = space.followers_count.checked_sub(1).ok_or(MSG_UNDERFLOW_UNFOLLOWING_SPACE)?;

      if space.created.account != follower {
        let author = space.created.account.clone();
        if let Some(score_diff) = Self::account_reputation_diff_by_account((follower.clone(), author.clone(), ScoringAction::FollowSpace)) {
          space.score = space.score.checked_sub(score_diff as i32).ok_or(MSG_OUT_OF_BOUNDS_UPDATING_SPACE_SCORE)?;
          Self::change_social_account_reputation(author.clone(), follower.clone(), score_diff * -1, ScoringAction::FollowSpace)?;
        }
      }

      <SpacesFollowedByAccount<T>>::mutate(follower.clone(), |space_ids| Self::vec_remove_on(space_ids, space_id));
      <SpaceFollowers<T>>::mutate(space_id, |account_ids| Self::vec_remove_on(account_ids, follower.clone()));
      <SpaceFollowedByAccount<T>>::remove((follower.clone(), space_id));
      <SocialAccountById<T>>::insert(follower.clone(), social_account);
      <SpaceById<T>>::insert(space_id, space);

      Self::deposit_event(RawEvent::SpaceUnfollowed(follower.clone(), space_id));
    }

    pub fn follow_account(origin, account: T::AccountId) {
      let follower = ensure_signed(origin)?;

      ensure!(follower != account, MSG_ACCOUNT_CANNOT_FOLLOW_ITSELF);
      ensure!(!<AccountFollowedByAccount<T>>::exists((follower.clone(), account.clone())), MSG_ACCOUNT_IS_ALREADY_FOLLOWED);

      let mut follower_account = Self::get_or_new_social_account(follower.clone());
      let mut followed_account = Self::get_or_new_social_account(account.clone());

      follower_account.following_accounts_count = follower_account.following_accounts_count
        .checked_add(1).ok_or(MSG_OVERFLOW_FOLLOWING_ACCOUNT)?;
      followed_account.followers_count = followed_account.followers_count
        .checked_add(1).ok_or(MSG_OVERFLOW_FOLLOWING_ACCOUNT)?;

      Self::change_social_account_reputation(account.clone(), follower.clone(),
        Self::get_score_diff(follower_account.reputation.clone(), ScoringAction::FollowAccount),
        ScoringAction::FollowAccount
      )?;

      <SocialAccountById<T>>::insert(follower.clone(), follower_account);
      <SocialAccountById<T>>::insert(account.clone(), followed_account);
      <AccountsFollowedByAccount<T>>::mutate(follower.clone(), |ids| ids.push(account.clone()));
      <AccountFollowers<T>>::mutate(account.clone(), |ids| ids.push(follower.clone()));
      <AccountFollowedByAccount<T>>::insert((follower.clone(), account.clone()), true);

      Self::deposit_event(RawEvent::AccountFollowed(follower, account));
    }

    pub fn unfollow_account(origin, account: T::AccountId) {
      let follower = ensure_signed(origin)?;

      ensure!(follower != account, MSG_ACCOUNT_CANNOT_UNFOLLOW_ITSELF);

      let mut follower_account = Self::social_account_by_id(follower.clone()).ok_or(MSG_FOLLOWER_ACCOUNT_NOT_FOUND)?;
      let mut followed_account = Self::social_account_by_id(account.clone()).ok_or(MSG_FOLLOWED_ACCOUNT_NOT_FOUND)?;

      ensure!(<AccountFollowedByAccount<T>>::exists((follower.clone(), account.clone())), MSG_ACCOUNT_IS_NOT_FOLLOWED);

      follower_account.following_accounts_count = follower_account.following_accounts_count
        .checked_sub(1).ok_or(MSG_UNDERFLOW_UNFOLLOWING_ACCOUNT)?;
      followed_account.followers_count = followed_account.followers_count
        .checked_sub(1).ok_or(MSG_UNDERFLOW_UNFOLLOWING_ACCOUNT)?;

      let reputation_diff = Self::account_reputation_diff_by_account(
        (follower.clone(), account.clone(), ScoringAction::FollowAccount)
      ).ok_or(MSG_REPUTATION_DIFF_NOT_FOUND)?;
      Self::change_social_account_reputation(account.clone(), follower.clone(),
        reputation_diff,
        ScoringAction::FollowAccount
      )?;

      <SocialAccountById<T>>::insert(follower.clone(), follower_account);
      <SocialAccountById<T>>::insert(account.clone(), followed_account);
      <AccountsFollowedByAccount<T>>::mutate(follower.clone(), |account_ids| Self::vec_remove_on(account_ids, account.clone()));
      <AccountFollowers<T>>::mutate(account.clone(), |account_ids| Self::vec_remove_on(account_ids, follower.clone()));
      <AccountFollowedByAccount<T>>::remove((follower.clone(), account.clone()));

      Self::deposit_event(RawEvent::AccountUnfollowed(follower, account));
    }

    // TODO use PostUpdate to pass data?
    pub fn create_post(origin, space_id: T::SpaceId, ipfs_hash: Vec<u8>, extension: PostExtension<T>) {
      let owner = ensure_signed(origin)?;

      let mut space = Self::space_by_id(space_id).ok_or(MSG_SPACE_NOT_FOUND)?;
      space.posts_count = space.posts_count.checked_add(1).ok_or(MSG_OVERFLOW_ADDING_POST_ON_SPACE)?;

      let new_post_id = Self::next_post_id();

      // Sharing functions contain check for post/comment existance
      match extension {
        PostExtension::RegularPost => {
          Self::is_ipfs_hash_valid(ipfs_hash.clone())?;
        },
        PostExtension::SharedPost(post_id) => {
          let post = Self::post_by_id(post_id).ok_or(MSG_ORIGINAL_POST_NOT_FOUND)?;
          ensure!(post.extension == PostExtension::RegularPost, MSG_CANNOT_SHARE_SHARED_POST);
          Self::share_post(owner.clone(), post_id, new_post_id)?;
        },
        PostExtension::SharedComment(comment_id) => {
          Self::share_comment(owner.clone(), comment_id, new_post_id)?;
        },
      }

      let new_post: Post<T> = Post {
        id: new_post_id,
        space_id,
        created: Self::new_change(owner.clone()),
        updated: None,
        extension,
        ipfs_hash,
        comments_count: 0,
        upvotes_count: 0,
        downvotes_count: 0,
        shares_count: 0,
        edit_history: vec![],
        score: 0,
      };

      <PostById<T>>::insert(new_post_id, new_post);
      <PostIdsBySpaceId<T>>::mutate(space_id, |ids| ids.push(new_post_id));
      <NextPostId<T>>::mutate(|n| { *n += T::PostId::sa(1); });
      <SpaceById<T>>::insert(space_id, space);

      Self::deposit_event(RawEvent::PostCreated(owner.clone(), new_post_id));
    }

    // TODO use CommentUpdate to pass data?
    pub fn create_comment(origin, post_id: T::PostId, parent_id: Option<T::CommentId>, ipfs_hash: Vec<u8>) {
      let owner = ensure_signed(origin)?;

      let ref mut post = Self::post_by_id(post_id).ok_or(MSG_POST_NOT_FOUND)?;
      Self::is_ipfs_hash_valid(ipfs_hash.clone())?;

      let comment_id = Self::next_comment_id();
      let new_comment: Comment<T> = Comment {
        id: comment_id,
        parent_id,
        post_id,
        created: Self::new_change(owner.clone()),
        updated: None,
        ipfs_hash,
        upvotes_count: 0,
        downvotes_count: 0,
        shares_count: 0,
        direct_replies_count: 0,
        edit_history: vec![],
        score: 0,
      };

      post.comments_count = post.comments_count.checked_add(1).ok_or(MSG_OVERFLOW_ADDING_COMMENT_ON_POST)?;

      Self::change_post_score(owner.clone(), post, ScoringAction::CreateComment)?;

      if let Some(id) = parent_id {
        let mut parent_comment = Self::comment_by_id(id).ok_or(MSG_UNKNOWN_PARENT_COMMENT)?;
        parent_comment.direct_replies_count = parent_comment.direct_replies_count.checked_add(1).ok_or(MSG_OVERFLOW_REPLYING_ON_COMMENT)?;
        <CommentById<T>>::insert(id, parent_comment);
      }

      <CommentById<T>>::insert(comment_id, new_comment);
      <CommentIdsByPostId<T>>::mutate(post_id, |ids| ids.push(comment_id));
      <NextCommentId<T>>::mutate(|n| { *n += T::CommentId::sa(1); });
      <PostById<T>>::insert(post_id, post);

      Self::deposit_event(RawEvent::CommentCreated(owner.clone(), comment_id));
    }

    pub fn create_post_reaction(origin, post_id: T::PostId, kind: ReactionKind) {
      let owner = ensure_signed(origin)?;

      ensure!(
        !<PostReactionIdByAccount<T>>::exists((owner.clone(), post_id)),
        MSG_ACCOUNT_ALREADY_REACTED_TO_POST
      );

      let ref mut post = Self::post_by_id(post_id).ok_or(MSG_POST_NOT_FOUND)?;
      let reaction_id = Self::new_reaction(owner.clone(), kind.clone());
      let action: ScoringAction;

      match kind {
        ReactionKind::Upvote => {
          post.upvotes_count = post.upvotes_count.checked_add(1).ok_or(MSG_OVERFLOW_UPVOTING_POST)?;
          action = ScoringAction::UpvotePost;
        },
        ReactionKind::Downvote => {
          post.downvotes_count = post.downvotes_count.checked_add(1).ok_or(MSG_OVERFLOW_DOWNVOTING_POST)?;
          action = ScoringAction::DownvotePost;
        },
      }

      if post.created.account != owner {
        Self::change_post_score(owner.clone(), post, action)?;
      }
      else {
        <PostById<T>>::insert(post_id, post);
      }

      <ReactionIdsByPostId<T>>::mutate(post_id, |ids| ids.push(reaction_id));
      <PostReactionIdByAccount<T>>::insert((owner.clone(), post_id), reaction_id);

      Self::deposit_event(RawEvent::PostReactionCreated(owner.clone(), post_id, reaction_id));
    }

    pub fn create_comment_reaction(origin, comment_id: T::CommentId, kind: ReactionKind) {
      let owner = ensure_signed(origin)?;

      ensure!(
        !<CommentReactionIdByAccount<T>>::exists((owner.clone(), comment_id)),
        MSG_ACCOUNT_ALREADY_REACTED_TO_COMMENT
      );

      let ref mut comment = Self::comment_by_id(comment_id).ok_or(MSG_COMMENT_NOT_FOUND)?;
      let reaction_id = Self::new_reaction(owner.clone(), kind.clone());
      let action: ScoringAction;

      match kind {
        ReactionKind::Upvote => {
          comment.upvotes_count = comment.upvotes_count.checked_add(1).ok_or(MSG_OVERFLOW_UPVOTING_COMMENT)?;
          action = ScoringAction::UpvoteComment;
        },
        ReactionKind::Downvote => {
          comment.downvotes_count = comment.downvotes_count.checked_add(1).ok_or(MSG_OVERFLOW_DOWNVOTING_COMMENT)?;
          action = ScoringAction::DownvoteComment;
        },
      }
      if comment.created.account != owner {
        Self::change_comment_score(owner.clone(), comment, action)?;
      }
      else {
        <CommentById<T>>::insert(comment_id, comment);
      }

      <ReactionIdsByCommentId<T>>::mutate(comment_id, |ids| ids.push(reaction_id));
      <CommentReactionIdByAccount<T>>::insert((owner.clone(), comment_id), reaction_id);

      Self::deposit_event(RawEvent::CommentReactionCreated(owner.clone(), comment_id, reaction_id));
    }

    pub fn create_profile(origin, username: Vec<u8>, ipfs_hash: Vec<u8>) {
      let owner = ensure_signed(origin)?;

      let mut social_account = Self::get_or_new_social_account(owner.clone());
      ensure!(social_account.profile.is_none(), MSG_PROFILE_ALREADY_EXISTS);
      Self::is_username_valid(username.clone())?;
      Self::is_ipfs_hash_valid(ipfs_hash.clone())?;

      social_account.profile = Some(
        Profile {
          created: Self::new_change(owner.clone()),
          updated: None,
          username: username.clone(),
          ipfs_hash,
          edit_history: vec![]
        }
      );
      <AccountByProfileUsername<T>>::insert(username.clone(), owner.clone());
      <SocialAccountById<T>>::insert(owner.clone(), social_account.clone());

      Self::deposit_event(RawEvent::ProfileCreated(owner.clone()));
    }

    pub fn update_profile(origin, update: ProfileUpdate) {
      let owner = ensure_signed(origin)?;

      let has_updates =
        update.username.is_some() ||
        update.ipfs_hash.is_some();
      
      ensure!(has_updates, MSG_NOTHING_TO_UPDATE_IN_PROFILE);

      let mut social_account = Self::social_account_by_id(owner.clone()).ok_or(MSG_SOCIAL_ACCOUNT_NOT_FOUND)?;
      let mut profile = social_account.profile.ok_or(MSG_PROFILE_DOESNT_EXIST)?;
      let mut is_update_applied = false;
      let mut new_history_record = ProfileHistoryRecord {
        edited: Self::new_change(owner.clone()),
        old_data: ProfileUpdate {username: None, ipfs_hash: None}
      };

      if let Some(ipfs_hash) = update.ipfs_hash {
        if ipfs_hash != profile.ipfs_hash {
          Self::is_ipfs_hash_valid(ipfs_hash.clone())?;
          new_history_record.old_data.ipfs_hash = Some(profile.ipfs_hash);
          profile.ipfs_hash = ipfs_hash;
          is_update_applied = true;
        }
      }

      if let Some(username) = update.username {
        if username != profile.username {
          Self::is_username_valid(username.clone())?;
          <AccountByProfileUsername<T>>::remove(profile.username.clone());
          <AccountByProfileUsername<T>>::insert(username.clone(), owner.clone());
          new_history_record.old_data.username = Some(profile.username);
          profile.username = username;
          is_update_applied = true;
        }
      }

      if is_update_applied {
        profile.updated = Some(Self::new_change(owner.clone()));
        profile.edit_history.push(new_history_record);
        social_account.profile = Some(profile);
        <SocialAccountById<T>>::insert(owner.clone(), social_account);

        Self::deposit_event(RawEvent::ProfileUpdated(owner.clone()));
      }
    }

    pub fn update_space(origin, space_id: T::SpaceId, update: SpaceUpdate) {
      let owner = ensure_signed(origin)?;
      
      let has_updates = 
        // update.writers.is_some() ||
        update.handle.is_some() ||
        update.ipfs_hash.is_some();

      ensure!(has_updates, MSG_NOTHING_TO_UPDATE_IN_SPACE);

      let mut space = Self::space_by_id(space_id).ok_or(MSG_SPACE_NOT_FOUND)?;

      // TODO ensure: space writers also should be able to edit this space:
      ensure!(owner == space.created.account, MSG_ONLY_SPACE_OWNER_CAN_UPDATE_SPACE);

      let mut fields_updated = 0;
      let mut new_history_record = SpaceHistoryRecord {
        edited: Self::new_change(owner.clone()),
        old_data: SpaceUpdate {/*writers: None, */handle: None, ipfs_hash: None}
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
        space.updated = Some(Self::new_change(owner.clone()));
        space.edit_history.push(new_history_record);
        <SpaceById<T>>::insert(space_id, space);
        Self::deposit_event(RawEvent::SpaceUpdated(owner.clone(), space_id));
      }
    }
    
    pub fn update_post(origin, post_id: T::PostId, update: PostUpdate<T>) {
      let owner = ensure_signed(origin)?;
      
      let has_updates = 
        update.space_id.is_some() ||
        update.ipfs_hash.is_some();

      ensure!(has_updates, MSG_NOTHING_TO_UPDATE_IN_POST);

      let mut post = Self::post_by_id(post_id).ok_or(MSG_POST_NOT_FOUND)?;

      // TODO ensure: space writers also should be able to edit this post:
      ensure!(owner == post.created.account, MSG_ONLY_POST_OWNER_CAN_UPDATE_POST);

      let mut fields_updated = 0;
      let mut new_history_record = PostHistoryRecord {
        edited: Self::new_change(owner.clone()),
        old_data: PostUpdate {space_id: None, ipfs_hash: None}
      };

      if let Some(ipfs_hash) = update.ipfs_hash {
        if ipfs_hash != post.ipfs_hash {
          Self::is_ipfs_hash_valid(ipfs_hash.clone())?;
          new_history_record.old_data.ipfs_hash = Some(post.ipfs_hash);
          post.ipfs_hash = ipfs_hash;
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
        post.updated = Some(Self::new_change(owner.clone()));
        post.edit_history.push(new_history_record);
        <PostById<T>>::insert(post_id, post);

        Self::deposit_event(RawEvent::PostUpdated(owner.clone(), post_id));
      }
    }
    
    pub fn update_comment(origin, comment_id: T::CommentId, update: CommentUpdate) {
      let owner = ensure_signed(origin)?;

      let mut comment = Self::comment_by_id(comment_id).ok_or(MSG_COMMENT_NOT_FOUND)?;
      ensure!(owner == comment.created.account, MSG_ONLY_COMMENT_AUTHOR_CAN_UPDATE_COMMENT);

      let ipfs_hash = update.ipfs_hash;
      ensure!(ipfs_hash != comment.ipfs_hash, MSG_NEW_COMMENT_HASH_DO_NOT_DIFFER);
      Self::is_ipfs_hash_valid(ipfs_hash.clone())?;

      let new_history_record = CommentHistoryRecord {
        edited: Self::new_change(owner.clone()),
        old_data: CommentUpdate {ipfs_hash: comment.ipfs_hash}
      };
      comment.edit_history.push(new_history_record);

      comment.ipfs_hash = ipfs_hash;
      comment.updated = Some(Self::new_change(owner.clone()));
      <CommentById<T>>::insert(comment_id, comment);

      Self::deposit_event(RawEvent::CommentUpdated(owner.clone(), comment_id));
    }

    pub fn update_post_reaction(origin, post_id: T::PostId, reaction_id: T::ReactionId, new_kind: ReactionKind) {
      let owner = ensure_signed(origin)?;

      ensure!(
        <PostReactionIdByAccount<T>>::exists((owner.clone(), post_id)),
        MSG_ACCOUNT_HAS_NOT_REACTED_TO_POST
      );

      let mut reaction = Self::reaction_by_id(reaction_id).ok_or(MSG_REACTION_NOT_FOUND)?;
      let ref mut post = Self::post_by_id(post_id).ok_or(MSG_POST_NOT_FOUND)?;

      ensure!(owner == reaction.created.account, MSG_ONLY_REACTION_OWNER_CAN_UPDATE_REACTION);
      ensure!(reaction.kind != new_kind, MSG_NEW_REACTION_KIND_DO_NOT_DIFFER);

      reaction.kind = new_kind;
      reaction.updated = Some(Self::new_change(owner.clone()));
      let action: ScoringAction;
      let action_to_cancel: ScoringAction;
      
      match new_kind {
        ReactionKind::Upvote => {
          post.upvotes_count += 1;
          post.downvotes_count -= 1;
          action_to_cancel = ScoringAction::DownvotePost;
          action = ScoringAction::UpvotePost;
        },
        ReactionKind::Downvote => {
          post.downvotes_count += 1;
          post.upvotes_count -= 1;
          action_to_cancel = ScoringAction::UpvotePost;
          action = ScoringAction::DownvotePost;
        },
      }
      Self::change_post_score(owner.clone(), post, action_to_cancel)?;
      Self::change_post_score(owner.clone(), post, action)?;

      <ReactionById<T>>::insert(reaction_id, reaction);
      <PostById<T>>::insert(post_id, post);

      Self::deposit_event(RawEvent::PostReactionUpdated(owner.clone(), post_id, reaction_id));
    }

    pub fn update_comment_reaction(origin, comment_id: T::CommentId, reaction_id: T::ReactionId, new_kind: ReactionKind) {
      let owner = ensure_signed(origin)?;

      ensure!(
        <CommentReactionIdByAccount<T>>::exists((owner.clone(), comment_id)),
        MSG_ACCOUNT_HAS_NOT_REACTED_TO_COMMENT
      );

      let mut reaction = Self::reaction_by_id(reaction_id).ok_or(MSG_REACTION_NOT_FOUND)?;
      let ref mut comment = Self::comment_by_id(comment_id).ok_or(MSG_COMMENT_NOT_FOUND)?;

      ensure!(owner == reaction.created.account, MSG_ONLY_REACTION_OWNER_CAN_UPDATE_REACTION);
      ensure!(reaction.kind != new_kind, MSG_NEW_REACTION_KIND_DO_NOT_DIFFER);

      reaction.kind = new_kind;
      reaction.updated = Some(Self::new_change(owner.clone()));
      let action: ScoringAction;
      let action_to_cancel: ScoringAction;
      
      match new_kind {
        ReactionKind::Upvote => {
          comment.upvotes_count += 1;
          comment.downvotes_count -= 1;
          action_to_cancel = ScoringAction::DownvoteComment;
          action = ScoringAction::UpvoteComment;
        },
        ReactionKind::Downvote => {
          comment.downvotes_count += 1;
          comment.upvotes_count -= 1;
          action_to_cancel = ScoringAction::UpvoteComment;
          action = ScoringAction::DownvoteComment;
        },
      }
      Self::change_comment_score(owner.clone(), comment, action_to_cancel)?;
      Self::change_comment_score(owner.clone(), comment, action)?;

      <ReactionById<T>>::insert(reaction_id, reaction);
      <CommentById<T>>::insert(comment_id, comment);

      Self::deposit_event(RawEvent::CommentReactionUpdated(owner.clone(), comment_id, reaction_id));
    }

    // TODO fn delete_space(origin, space_id: T::SpaceId) {
      // TODO only owner can delete
      // TODO unfollow all space followers
    // }
    
    // TODO fn delete_post(origin, post_id: T::PostId) {}
    
    // TODO fn delete_comment(origin, comment_id: T::CommentId) {}

    pub fn delete_post_reaction(origin, post_id: T::PostId, reaction_id: T::ReactionId) {
      let owner = ensure_signed(origin)?;

      ensure!(
        <PostReactionIdByAccount<T>>::exists((owner.clone(), post_id)),
        MSG_NO_POST_REACTION_BY_ACCOUNT_TO_DELETE
      );
      
      let action_to_cancel: ScoringAction;
      let reaction = Self::reaction_by_id(reaction_id).ok_or(MSG_REACTION_NOT_FOUND)?;
      let ref mut post = Self::post_by_id(post_id).ok_or(MSG_POST_NOT_FOUND)?;

      ensure!(owner == reaction.created.account, MSG_ONLY_REACTION_OWNER_CAN_UPDATE_REACTION);

      match reaction.kind {
        ReactionKind::Upvote => {
          post.upvotes_count -= 1;
          action_to_cancel = ScoringAction::UpvotePost;
        },
        ReactionKind::Downvote => {
          post.downvotes_count -= 1;
          action_to_cancel = ScoringAction::DownvotePost;

        },
      }
      Self::change_post_score(owner.clone(), post, action_to_cancel)?;

      <PostById<T>>::insert(post_id, post);
      <ReactionById<T>>::remove(reaction_id);
      <ReactionIdsByPostId<T>>::mutate(post_id, |ids| Self::vec_remove_on(ids, reaction_id));
      <PostReactionIdByAccount<T>>::remove((owner.clone(), post_id));

      Self::deposit_event(RawEvent::PostReactionDeleted(owner.clone(), post_id, reaction_id));
    }

    pub fn delete_comment_reaction(origin, comment_id: T::CommentId, reaction_id: T::ReactionId) {
      let owner = ensure_signed(origin)?;

      ensure!(
        <CommentReactionIdByAccount<T>>::exists((owner.clone(), comment_id)),
        MSG_NO_COMMENT_REACTION_BY_ACCOUNT_TO_DELETE
      );
      
      let action_to_cancel: ScoringAction;
      let reaction = Self::reaction_by_id(reaction_id).ok_or(MSG_REACTION_NOT_FOUND)?;
      let ref mut comment = Self::comment_by_id(comment_id).ok_or(MSG_COMMENT_NOT_FOUND)?;
      
      ensure!(owner == reaction.created.account, MSG_ONLY_REACTION_OWNER_CAN_UPDATE_REACTION);

      match reaction.kind {
        ReactionKind::Upvote => {
          comment.upvotes_count -= 1;
          action_to_cancel = ScoringAction::UpvoteComment
        },
        ReactionKind::Downvote => {
          comment.downvotes_count -= 1;
          action_to_cancel = ScoringAction::DownvoteComment
        },
      }
      Self::change_comment_score(owner.clone(), comment, action_to_cancel)?;

      <CommentById<T>>::insert(comment_id, comment);
      <ReactionIdsByCommentId<T>>::mutate(comment_id, |ids| Self::vec_remove_on(ids, reaction_id));
      <ReactionById<T>>::remove(reaction_id);
      <CommentReactionIdByAccount<T>>::remove((owner.clone(), comment_id));

      Self::deposit_event(RawEvent::CommentReactionDeleted(owner.clone(), comment_id, reaction_id));
    }

    // TODO spend some tokens on: create/update a space/post/comment.
  }
}
