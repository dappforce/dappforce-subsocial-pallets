use super::messages::*;

use rstd::prelude::*;
use parity_codec::Codec;
use parity_codec_derive::{Encode, Decode};
use srml_support::{StorageMap, StorageValue, decl_module, decl_storage, decl_event, dispatch::Result, ensure, Parameter};
use runtime_primitives::traits::{SimpleArithmetic, As, Member, MaybeDebug, MaybeSerializeDebug};
use system::{self, ensure_signed};
use {timestamp};

pub const DEFAULT_SLUG_MAX_LEN: u32 = 50;
pub const DEFAULT_SLUG_MIN_LEN: u32 = 5;

pub const DEFAULT_IPFS_HASH_LEN: u32 = 46;

pub const DEFAULT_USERNAME_MAX_LEN: u32 = 24;
pub const DEFAULT_USERNAME_MIN_LEN: u32 = 3;

pub const DEFAULT_BLOG_MAX_LEN: u32 = 1_000;
pub const DEFAULT_POST_MAX_LEN: u32 = 10_000;
pub const DEFAULT_COMMENT_MAX_LEN: u32 = 1_000;

pub const DEFAULT_UPVOTE_POST_ACTION_WEIGHT: i16 = 5;
pub const DEFAULT_DOWNVOTE_POST_ACTION_WEIGHT: i16 = -3;
pub const DEFAULT_SHARE_POST_ACTION_WEIGHT: i16 = 5;
pub const DEFAULT_CREATE_COMMENT_ACTION_WEIGHT: i16 = 5;
pub const DEFAULT_UPVOTE_COMMENT_ACTION_WEIGHT: i16 = 4;
pub const DEFAULT_DOWNVOTE_COMMENT_ACTION_WEIGHT: i16 = -2;
pub const DEFAULT_SHARE_COMMENT_ACTION_WEIGHT: i16 = 3;
pub const DEFAULT_FOLLOW_BLOG_ACTION_WEIGHT: i16 = 7;
pub const DEFAULT_FOLLOW_ACCOUNT_ACTION_WEIGHT: i16 = 3;

pub trait Trait: system::Trait + timestamp::Trait + MaybeDebug {

  type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;

  type BlogId: Parameter + Member + SimpleArithmetic + Codec + Default + Copy
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
  block: T::BlockNumber,
  time: T::Moment,
}

// TODO add a schema along w/ JSON, maybe create a struct?

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Clone, Encode, Decode, PartialEq)]
pub struct Blog<T: Trait> {
  id: T::BlogId,
  pub created: Change<T>,
  updated: Option<Change<T>>,

  // Can be updated by the owner:
  pub writers: Vec<T::AccountId>,
  pub slug: Vec<u8>,
  pub ipfs_hash: Vec<u8>,

  pub posts_count: u16,
  pub followers_count: u32,

  pub edit_history: Vec<BlogHistoryRecord<T>>,

  pub score: i32,
}

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Clone, Encode, Decode, PartialEq)]
pub struct BlogUpdate<T: Trait> {
  pub writers: Option<Vec<T::AccountId>>,
  pub slug: Option<Vec<u8>>,
  pub ipfs_hash: Option<Vec<u8>>,
}

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Clone, Encode, Decode, PartialEq)]
pub struct BlogHistoryRecord<T: Trait> {
  edited: Change<T>,
  pub old_data: BlogUpdate<T>,
}

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Clone, Encode, Decode, PartialEq)]
pub struct Post<T: Trait> {
  id: T::PostId,
  pub blog_id: T::BlogId,
  pub created: Change<T>,
  updated: Option<Change<T>>,
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
  pub blog_id: Option<T::BlogId>,
  pub ipfs_hash: Option<Vec<u8>>,
}

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Clone, Encode, Decode, PartialEq)]
pub struct PostHistoryRecord<T: Trait> {
  edited: Change<T>,
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
  id: T::CommentId,
  pub parent_id: Option<T::CommentId>,
  pub post_id: T::PostId,
  pub created: Change<T>,
  updated: Option<Change<T>>,

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
  edited: Change<T>,
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
  id: T::ReactionId,
  pub created: Change<T>,
  updated: Option<Change<T>>,
  pub kind: ReactionKind,
}

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Clone, Encode, Decode, PartialEq)]
pub struct SocialAccount<T: Trait> {
  followers_count: u32,
  following_accounts_count: u16,
  following_blogs_count: u16,
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
  edited: Change<T>,
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
  FollowBlog,
  FollowAccount,
}

impl Default for ScoringAction {
  fn default() -> Self {
    ScoringAction::FollowAccount
  }
}

decl_storage! {
  trait Store for Module<T: Trait> as Blogs {

    SlugMinLen get(slug_min_len): u32 = DEFAULT_SLUG_MIN_LEN;
    SlugMaxLen get(slug_max_len): u32 = DEFAULT_SLUG_MAX_LEN;

    IpfsHashLen get(ipfs_hash_len): u32 = DEFAULT_IPFS_HASH_LEN;

    UsernameMinLen get(username_min_len): u32 = DEFAULT_USERNAME_MIN_LEN;
    UsernameMaxLen get(username_max_len): u32 = DEFAULT_USERNAME_MAX_LEN;

    BlogMaxLen get(blog_max_len): u32 = DEFAULT_BLOG_MAX_LEN;
    PostMaxLen get(post_max_len): u32 = DEFAULT_POST_MAX_LEN;
    CommentMaxLen get(comment_max_len): u32 = DEFAULT_COMMENT_MAX_LEN;

    UpvotePostActionWeight get (upvote_post_action_weight): i16 = DEFAULT_UPVOTE_POST_ACTION_WEIGHT;
    DownvotePostActionWeight get (downvote_post_action_weight): i16 = DEFAULT_DOWNVOTE_POST_ACTION_WEIGHT;
    SharePostActionWeight get (share_post_action_weight): i16 = DEFAULT_SHARE_POST_ACTION_WEIGHT;
    CreateCommentActionWeight get (create_comment_action_weight): i16 = DEFAULT_CREATE_COMMENT_ACTION_WEIGHT;
    UpvoteCommentActionWeight get (upvote_comment_action_weight): i16 = DEFAULT_UPVOTE_COMMENT_ACTION_WEIGHT;
    DownvoteCommentActionWeight get (downvote_comment_action_weight): i16 = DEFAULT_DOWNVOTE_COMMENT_ACTION_WEIGHT;
    ShareCommentActionWeight get (share_comment_action_weight): i16 = DEFAULT_SHARE_COMMENT_ACTION_WEIGHT;
    FollowBlogActionWeight get (follow_blog_action_weight): i16 = DEFAULT_FOLLOW_BLOG_ACTION_WEIGHT;
    FollowAccountActionWeight get (follow_account_action_weight): i16 = DEFAULT_FOLLOW_ACCOUNT_ACTION_WEIGHT;

    BlogById get(blog_by_id): map T::BlogId => Option<Blog<T>>;
    PostById get(post_by_id): map T::PostId => Option<Post<T>>;
    CommentById get(comment_by_id): map T::CommentId => Option<Comment<T>>;
    ReactionById get(reaction_by_id): map T::ReactionId => Option<Reaction<T>>;
    SocialAccountById get(social_account_by_id): map T::AccountId => Option<SocialAccount<T>>;

    BlogIdsByOwner get(blog_ids_by_owner): map T::AccountId => Vec<T::BlogId>;
    PostIdsByBlogId get(post_ids_by_blog_id): map T::BlogId => Vec<T::PostId>;
    CommentIdsByPostId get(comment_ids_by_post_id): map T::PostId => Vec<T::CommentId>;

    ReactionIdsByPostId get(reaction_ids_by_post_id): map T::PostId => Vec<T::ReactionId>;
    ReactionIdsByCommentId get(reaction_ids_by_comment_id): map T::CommentId => Vec<T::ReactionId>;
    PostReactionIdByAccount get(post_reaction_id_by_account): map (T::AccountId, T::PostId) => T::ReactionId;
    CommentReactionIdByAccount get(comment_reaction_id_by_account): map (T::AccountId, T::CommentId) => T::ReactionId;

    BlogIdBySlug get(blog_id_by_slug): map Vec<u8> => Option<T::BlogId>;

    BlogsFollowedByAccount get(blogs_followed_by_account): map T::AccountId => Vec<T::BlogId>;
    BlogFollowers get(blog_followers): map T::BlogId => Vec<T::AccountId>;
    BlogFollowedByAccount get(blog_followed_by_account): map (T::AccountId, T::BlogId) => bool;

    AccountFollowedByAccount get(account_followed_by_account): map (T::AccountId, T::AccountId) => bool;
    AccountsFollowedByAccount get(accounts_followed_by_account): map T::AccountId => Vec<T::AccountId>;
    AccountFollowers get(account_followers): map T::AccountId => Vec<T::AccountId>;

    NextBlogId get(next_blog_id): T::BlogId = T::BlogId::sa(1);
    NextPostId get(next_post_id): T::PostId = T::PostId::sa(1);
    NextCommentId get(next_comment_id): T::CommentId = T::CommentId::sa(1);
    NextReactionId get(next_reaction_id): T::ReactionId = T::ReactionId::sa(1);

    AccountReputationDiffByAccount get(account_reputation_diff_by_account): map (T::AccountId, T::AccountId, ScoringAction) => Option<i16>; // TODO shorten name (?refactor)
    PostScoreByAccount get(post_score_by_account): map (T::AccountId, T::PostId, ScoringAction) => Option<i16>;
    CommentScoreByAccount get(comment_score_by_account): map (T::AccountId, T::CommentId, ScoringAction) => Option<i16>;

    PostSharesByAccount get(post_shares_by_account): map (T::AccountId, T::PostId) => u16;
    SharedPostIdsByOriginalPostId get(shared_post_ids_by_original_post_id): map T::PostId => Vec<T::PostId>;

    CommentSharesByAccount get(comment_shares_by_account): map (T::AccountId, T::CommentId) => u16;
    SharedPostIdsByOriginalCommentId get(shared_post_ids_by_original_comment_id): map T::CommentId => Vec<T::PostId>;

    AccountByProfileUsername get(account_by_profile_username): map Vec<u8> => Option<T::AccountId>;
  }
}

decl_event! {
  pub enum Event<T> where
    <T as system::Trait>::AccountId,
    <T as Trait>::BlogId,
    <T as Trait>::PostId,
    <T as Trait>::CommentId,
    <T as Trait>::ReactionId
  {
    BlogCreated(AccountId, BlogId),
    BlogUpdated(AccountId, BlogId),
    BlogDeleted(AccountId, BlogId),

    BlogFollowed(AccountId, BlogId),
    BlogUnfollowed(AccountId, BlogId),

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

    fn deposit_event<T>() = default;

    fn on_initialize(_now: T::BlockNumber) {
      // Stub
    }

    fn on_finalize(_now: T::BlockNumber) {
      // Stub
    }

    // TODO use BlogUpdate to pass data
    pub fn create_blog(origin, slug: Vec<u8>, ipfs_hash: Vec<u8>) {
      let owner = ensure_signed(origin)?;

      ensure!(slug.len() >= Self::slug_min_len() as usize, MSG_BLOG_SLUG_IS_TOO_SHORT);
      ensure!(slug.len() <= Self::slug_max_len() as usize, MSG_BLOG_SLUG_IS_TOO_LONG);
      ensure!(!<BlogIdBySlug<T>>::exists(slug.clone()), MSG_BLOG_SLUG_IS_NOT_UNIQUE);
      Self::is_ipfs_hash_valid(ipfs_hash.clone())?;

      let blog_id = Self::next_blog_id();
      let ref mut new_blog: Blog<T> = Blog {
        id: blog_id,
        created: Self::new_change(owner.clone()),
        updated: None,
        writers: vec![],
        slug: slug.clone(),
        ipfs_hash,
        posts_count: 0,
        followers_count: 0,
        edit_history: vec![],
        score: 0
      };

      // Blog creator automatically follows their blog:
      Self::add_blog_follower_and_insert_blog(owner.clone(), new_blog, true)?;

      <BlogIdsByOwner<T>>::mutate(owner.clone(), |ids| ids.push(blog_id));
      <BlogIdBySlug<T>>::insert(slug, blog_id);
      <NextBlogId<T>>::mutate(|n| { *n += T::BlogId::sa(1); });
    }

    pub fn follow_blog(origin, blog_id: T::BlogId) {
      let follower = ensure_signed(origin)?;

      let ref mut blog = Self::blog_by_id(blog_id).ok_or(MSG_BLOG_NOT_FOUND)?;
      ensure!(!Self::blog_followed_by_account((follower.clone(), blog_id)), MSG_ACCOUNT_IS_FOLLOWING_BLOG);

      Self::add_blog_follower_and_insert_blog(follower.clone(), blog, false)?;
    }

    pub fn unfollow_blog(origin, blog_id: T::BlogId) {
      let follower = ensure_signed(origin)?;

      let ref mut blog = Self::blog_by_id(blog_id).ok_or(MSG_BLOG_NOT_FOUND)?;
      ensure!(Self::blog_followed_by_account((follower.clone(), blog_id)), MSG_ACCOUNT_IS_NOT_FOLLOWING_BLOG);

      let mut social_account = Self::social_account_by_id(follower.clone()).ok_or(MSG_SOCIAL_ACCOUNT_NOT_FOUND)?;
      social_account.following_blogs_count = social_account.following_blogs_count
        .checked_sub(1)
        .ok_or(MSG_UNDERFLOW_UNFOLLOWING_BLOG)?;
      blog.followers_count = blog.followers_count.checked_sub(1).ok_or(MSG_UNDERFLOW_UNFOLLOWING_BLOG)?;

      if blog.created.account != follower {
        let author = blog.created.account.clone();
        if let Some(score_diff) = Self::account_reputation_diff_by_account((follower.clone(), author.clone(), ScoringAction::FollowBlog)) {
          blog.score = blog.score.checked_sub(score_diff as i32).ok_or(MSG_OUT_OF_BOUNDS_UPDATING_BLOG_SCORE)?;
          Self::change_social_account_reputation(author.clone(), follower.clone(), score_diff * -1, ScoringAction::FollowBlog)?;
        }
      }

      <BlogsFollowedByAccount<T>>::mutate(follower.clone(), |blog_ids| Self::vec_remove_on(blog_ids, blog_id));
      <BlogFollowers<T>>::mutate(blog_id, |account_ids| Self::vec_remove_on(account_ids, follower.clone()));
      <BlogFollowedByAccount<T>>::remove((follower.clone(), blog_id));
      <SocialAccountById<T>>::insert(follower.clone(), social_account);
      <BlogById<T>>::insert(blog_id, blog);

      Self::deposit_event(RawEvent::BlogUnfollowed(follower.clone(), blog_id));
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
    pub fn create_post(origin, blog_id: T::BlogId, ipfs_hash: Vec<u8>, extension: PostExtension<T>) {
      let owner = ensure_signed(origin)?;

      let mut blog = Self::blog_by_id(blog_id).ok_or(MSG_BLOG_NOT_FOUND)?;
      blog.posts_count = blog.posts_count.checked_add(1).ok_or(MSG_OVERFLOW_ADDING_POST_ON_BLOG)?;

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
        blog_id,
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
      <PostIdsByBlogId<T>>::mutate(blog_id, |ids| ids.push(new_post_id));
      <NextPostId<T>>::mutate(|n| { *n += T::PostId::sa(1); });
      <BlogById<T>>::insert(blog_id, blog);

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

    pub fn update_blog(origin, blog_id: T::BlogId, update: BlogUpdate<T>) {
      let owner = ensure_signed(origin)?;
      
      let has_updates = 
        update.writers.is_some() ||
        update.slug.is_some() ||
        update.ipfs_hash.is_some();

      ensure!(has_updates, MSG_NOTHING_TO_UPDATE_IN_BLOG);

      let mut blog = Self::blog_by_id(blog_id).ok_or(MSG_BLOG_NOT_FOUND)?;

      // TODO ensure: blog writers also should be able to edit this blog:
      ensure!(owner == blog.created.account, MSG_ONLY_BLOG_OWNER_CAN_UPDATE_BLOG);

      let mut fields_updated = 0;
      let mut new_history_record = BlogHistoryRecord {
        edited: Self::new_change(owner.clone()),
        old_data: BlogUpdate {writers: None, slug: None, ipfs_hash: None}
      };

      if let Some(writers) = update.writers {
        if writers != blog.writers {
          // TODO validate writers.
          // TODO update BlogIdsByWriter: insert new, delete removed, update only changed writers.
          new_history_record.old_data.writers = Some(blog.writers);
          blog.writers = writers;
          fields_updated += 1;
        }
      }

      if let Some(ipfs_hash) = update.ipfs_hash {
        if ipfs_hash != blog.ipfs_hash {
          Self::is_ipfs_hash_valid(ipfs_hash.clone())?;
          new_history_record.old_data.ipfs_hash = Some(blog.ipfs_hash);
          blog.ipfs_hash = ipfs_hash;
          fields_updated += 1;
        }
      }

      if let Some(slug) = update.slug {
        if slug != blog.slug {
          let slug_len = slug.len();
          ensure!(slug_len >= Self::slug_min_len() as usize, MSG_BLOG_SLUG_IS_TOO_SHORT);
          ensure!(slug_len <= Self::slug_max_len() as usize, MSG_BLOG_SLUG_IS_TOO_LONG);
          ensure!(!<BlogIdBySlug<T>>::exists(slug.clone()), MSG_BLOG_SLUG_IS_NOT_UNIQUE);

          <BlogIdBySlug<T>>::remove(blog.slug.clone());
          <BlogIdBySlug<T>>::insert(slug.clone(), blog_id);
          new_history_record.old_data.slug = Some(blog.slug);
          blog.slug = slug;
          fields_updated += 1;
        }
      }

      // Update this blog only if at least one field should be updated:
      if fields_updated > 0 {
        blog.updated = Some(Self::new_change(owner.clone()));
        blog.edit_history.push(new_history_record);
        <BlogById<T>>::insert(blog_id, blog);
        Self::deposit_event(RawEvent::BlogUpdated(owner.clone(), blog_id));
      }
    }
    
    pub fn update_post(origin, post_id: T::PostId, update: PostUpdate<T>) {
      let owner = ensure_signed(origin)?;
      
      let has_updates = 
        update.blog_id.is_some() ||
        update.ipfs_hash.is_some();

      ensure!(has_updates, MSG_NOTHING_TO_UPDATE_IN_POST);

      let mut post = Self::post_by_id(post_id).ok_or(MSG_POST_NOT_FOUND)?;

      // TODO ensure: blog writers also should be able to edit this post:
      ensure!(owner == post.created.account, MSG_ONLY_POST_OWNER_CAN_UPDATE_POST);

      let mut fields_updated = 0;
      let mut new_history_record = PostHistoryRecord {
        edited: Self::new_change(owner.clone()),
        old_data: PostUpdate {blog_id: None, ipfs_hash: None}
      };

      if let Some(ipfs_hash) = update.ipfs_hash {
        if ipfs_hash != post.ipfs_hash {
          Self::is_ipfs_hash_valid(ipfs_hash.clone())?;
          new_history_record.old_data.ipfs_hash = Some(post.ipfs_hash);
          post.ipfs_hash = ipfs_hash;
          fields_updated += 1;
        }
      }

      // Move this post to another blog:
      if let Some(blog_id) = update.blog_id {
        if blog_id != post.blog_id {
          Self::ensure_blog_exists(blog_id)?;
          
          // Remove post_id from its old blog:
          <PostIdsByBlogId<T>>::mutate(post.blog_id, |post_ids| Self::vec_remove_on(post_ids, post_id));
          
          // Add post_id to its new blog:
          <PostIdsByBlogId<T>>::mutate(blog_id.clone(), |ids| ids.push(post_id));
          new_history_record.old_data.blog_id = Some(post.blog_id);
          post.blog_id = blog_id;
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
      let mut post = Self::post_by_id(post_id).ok_or(MSG_POST_NOT_FOUND)?;

      ensure!(owner == reaction.created.account, MSG_ONLY_REACTION_OWNER_CAN_UPDATE_REACTION);
      ensure!(reaction.kind != new_kind, MSG_NEW_REACTION_KIND_DO_NOT_DIFFER);

      reaction.kind = new_kind;
      reaction.updated = Some(Self::new_change(owner.clone()));
      
      match new_kind {
        ReactionKind::Upvote => {
          post.upvotes_count += 1;
          post.downvotes_count -= 1;
        },
        ReactionKind::Downvote => {
          post.downvotes_count += 1;
          post.upvotes_count -= 1;
        },
      }

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
      let mut comment = Self::comment_by_id(comment_id).ok_or(MSG_COMMENT_NOT_FOUND)?;

      ensure!(owner == reaction.created.account, MSG_ONLY_REACTION_OWNER_CAN_UPDATE_REACTION);
      ensure!(reaction.kind != new_kind, MSG_NEW_REACTION_KIND_DO_NOT_DIFFER);

      reaction.kind = new_kind;
      reaction.updated = Some(Self::new_change(owner.clone()));

      match new_kind {
        ReactionKind::Upvote => {
          comment.upvotes_count += 1;
          comment.downvotes_count -= 1;
        },
        ReactionKind::Downvote => {
          comment.downvotes_count += 1;
          comment.upvotes_count -= 1;
        },
      }

      <ReactionById<T>>::insert(reaction_id, reaction);
      <CommentById<T>>::insert(comment_id, comment);

      Self::deposit_event(RawEvent::CommentReactionUpdated(owner.clone(), comment_id, reaction_id));
    }

    // TODO fn delete_blog(origin, blog_id: T::BlogId) {
      // TODO only owner can delete
      // TODO unfollow all blog followers
    // }
    
    // TODO fn delete_post(origin, post_id: T::PostId) {}
    
    // TODO fn delete_comment(origin, comment_id: T::CommentId) {}

    pub fn delete_post_reaction(origin, post_id: T::PostId, reaction_id: T::ReactionId) {
      let owner = ensure_signed(origin)?;

      ensure!(
        <PostReactionIdByAccount<T>>::exists((owner.clone(), post_id)),
        MSG_NO_POST_REACTION_BY_ACCOUNT_TO_DELETE
      );
      
      let reaction = Self::reaction_by_id(reaction_id).ok_or(MSG_REACTION_NOT_FOUND)?;
      let mut post = Self::post_by_id(post_id).ok_or(MSG_POST_NOT_FOUND)?;

      ensure!(owner == reaction.created.account, MSG_ONLY_REACTION_OWNER_CAN_UPDATE_REACTION);

      match reaction.kind {
        ReactionKind::Upvote => post.upvotes_count -= 1,
        ReactionKind::Downvote => post.downvotes_count -= 1,
      }

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
      
      let reaction = Self::reaction_by_id(reaction_id).ok_or(MSG_REACTION_NOT_FOUND)?;
      let mut comment = Self::comment_by_id(comment_id).ok_or(MSG_COMMENT_NOT_FOUND)?;
      
      ensure!(owner == reaction.created.account, MSG_ONLY_REACTION_OWNER_CAN_UPDATE_REACTION);

      match reaction.kind {
        ReactionKind::Upvote => comment.upvotes_count -= 1,
        ReactionKind::Downvote => comment.downvotes_count -= 1,
      }

      <CommentById<T>>::insert(comment_id, comment);
      <ReactionIdsByCommentId<T>>::mutate(comment_id, |ids| Self::vec_remove_on(ids, reaction_id));
      <ReactionById<T>>::remove(reaction_id);
      <CommentReactionIdByAccount<T>>::remove((owner.clone(), comment_id));

      Self::deposit_event(RawEvent::CommentReactionDeleted(owner.clone(), comment_id, reaction_id));
    }

    // TODO spend some tokens on: create/update a blog/post/comment.
  }
}

impl<T: Trait> Module<T> {

  fn ensure_blog_exists(blog_id: T::BlogId) -> Result {
    ensure!(<BlogById<T>>::exists(blog_id), MSG_BLOG_NOT_FOUND);
    Ok(())
  }

  fn new_change(account: T::AccountId) -> Change<T> {
    Change {
      account,
      block: <system::Module<T>>::block_number(),
      time: <timestamp::Module<T>>::now(),
    }
  }

  // TODO: maybe don't add reaction in storage before checks in 'create_reaction' are done?
  fn new_reaction(account: T::AccountId, kind: ReactionKind) -> T::ReactionId {
    let reaction_id = Self::next_reaction_id();
    let new_reaction: Reaction<T> = Reaction {
      id: reaction_id,
      created: Self::new_change(account),
      updated: None,
      kind
    };

    <ReactionById<T>>::insert(reaction_id, new_reaction);
    <NextReactionId<T>>::mutate(|n| { *n += T::ReactionId::sa(1); });

    reaction_id
  }

  fn add_blog_follower_and_insert_blog(
    follower: T::AccountId,
    blog: &mut Blog<T>,
    is_new_blog: bool
  ) -> Result {

    let blog_id = blog.id;
    let mut social_account = Self::get_or_new_social_account(follower.clone());
    social_account.following_blogs_count = social_account.following_blogs_count
      .checked_add(1)
      .ok_or(MSG_OVERFLOW_FOLLOWING_BLOG)?;

    blog.followers_count = blog.followers_count.checked_add(1).ok_or(MSG_OVERFLOW_FOLLOWING_BLOG)?;
    if blog.created.account != follower {
      let author = blog.created.account.clone();
      let score_diff = Self::get_score_diff(social_account.reputation, ScoringAction::FollowBlog);
      blog.score = blog.score.checked_add(score_diff as i32).ok_or(MSG_OUT_OF_BOUNDS_UPDATING_BLOG_SCORE)?;
      Self::change_social_account_reputation(author.clone(), follower.clone(), score_diff, ScoringAction::FollowBlog)?;
    }

    <BlogById<T>>::insert(blog_id, blog);
    <SocialAccountById<T>>::insert(follower.clone(), social_account.clone());
    <BlogsFollowedByAccount<T>>::mutate(follower.clone(), |ids| ids.push(blog_id));
    <BlogFollowers<T>>::mutate(blog_id, |ids| ids.push(follower.clone()));
    <BlogFollowedByAccount<T>>::insert((follower.clone(), blog_id), true);

    if is_new_blog {
      Self::deposit_event(RawEvent::BlogCreated(follower.clone(), blog_id));
    }

    Self::deposit_event(RawEvent::BlogFollowed(follower, blog_id));
    
    Ok(())
  }

  fn get_or_new_social_account(account: T::AccountId) -> SocialAccount<T> {
    if let Some(social_account) = Self::social_account_by_id(account) {
      social_account
    } else {
      SocialAccount {
        followers_count: 0,
        following_accounts_count: 0,
        following_blogs_count: 0,
        reputation: 1,
        profile: None
      }
    }
  }

  fn vec_remove_on<F: PartialEq>(vector: &mut Vec<F>, element: F) {
    if let Some(index) = vector.iter().position(|x| *x == element) {
      vector.swap_remove(index);
    }
  }

  pub fn change_post_score(account: T::AccountId, post: &mut Post<T>, action: ScoringAction) -> Result {
    let social_account = Self::get_or_new_social_account(account.clone());
    let post_id = post.id;
    let mut blog = Self::blog_by_id(post.blog_id).ok_or(MSG_BLOG_NOT_FOUND)?;
    
    if post.created.account != account {
      if let Some(score_diff) = Self::post_score_by_account((account.clone(), post_id, action)) {
        let reputation_diff = Self::account_reputation_diff_by_account((account.clone(), post.created.account.clone(), action)).ok_or(MSG_REPUTATION_DIFF_NOT_FOUND)?;
        post.score = post.score.checked_add(score_diff as i32 * -1).ok_or(MSG_OUT_OF_BOUNDS_REVERTING_POST_SCORE)?;
        blog.score = blog.score.checked_add(score_diff as i32 * -1).ok_or(MSG_OUT_OF_BOUNDS_REVERTING_BLOG_SCORE)?;
        Self::change_social_account_reputation(post.created.account.clone(), account.clone(), reputation_diff * -1, action)?;
        <PostScoreByAccount<T>>::remove((account.clone(), post_id, action));
      } else {
        match action {
          ScoringAction::UpvotePost => {
            if Self::post_score_by_account((account.clone(), post_id, ScoringAction::DownvotePost)).is_some() {
              Self::change_post_score(account.clone(), post, ScoringAction::DownvotePost)?;
            }
          },
          ScoringAction::DownvotePost => {
            if Self::post_score_by_account((account.clone(), post_id, ScoringAction::UpvotePost)).is_some() {
              Self::change_post_score(account.clone(), post, ScoringAction::UpvotePost)?;
            }
          },
          _ => (),
        }
        let score_diff = Self::get_score_diff(social_account.reputation, action);
        post.score = post.score.checked_add(score_diff as i32).ok_or(MSG_OUT_OF_BOUNDS_UPDATING_POST_SCORE)?;
        blog.score = blog.score.checked_add(score_diff as i32).ok_or(MSG_OUT_OF_BOUNDS_UPDATING_BLOG_SCORE)?;
        Self::change_social_account_reputation(post.created.account.clone(), account.clone(), score_diff, action)?;
        <PostScoreByAccount<T>>::insert((account.clone(), post_id, action), score_diff);
      }

      <PostById<T>>::insert(post_id, post.clone());
      <BlogById<T>>::insert(post.blog_id, blog.clone());
    }

    Ok(())
  }

  pub fn change_comment_score(account: T::AccountId, comment: &mut Comment<T>, action: ScoringAction) -> Result {
    let social_account = Self::get_or_new_social_account(account.clone());
    let comment_id = comment.id;

    if comment.created.account != account {
      if let Some(score_diff) = Self::comment_score_by_account((account.clone(), comment_id, action)) {
        let reputation_diff = Self::account_reputation_diff_by_account((account.clone(), comment.created.account.clone(), action)).ok_or(MSG_REPUTATION_DIFF_NOT_FOUND)?;
        comment.score = comment.score.checked_add(score_diff as i32 * -1).ok_or(MSG_OUT_OF_BOUNDS_REVERTING_COMMENT_SCORE)?;
        Self::change_social_account_reputation(comment.created.account.clone(), account.clone(), reputation_diff * -1, action)?;
        <CommentScoreByAccount<T>>::remove((account.clone(), comment_id, action));
      } else {
        match action {
          ScoringAction::UpvoteComment => {
            if Self::comment_score_by_account((account.clone(), comment_id, ScoringAction::DownvoteComment)).is_some() {
              Self::change_comment_score(account.clone(), comment, ScoringAction::DownvoteComment)?;
            }
          },
          ScoringAction::DownvoteComment => {
            if Self::comment_score_by_account((account.clone(), comment_id, ScoringAction::UpvoteComment)).is_some() {
              Self::change_comment_score(account.clone(), comment, ScoringAction::UpvoteComment)?;
            }
          },
          ScoringAction::CreateComment => {
            let ref mut post = Self::post_by_id(comment.post_id).ok_or(MSG_POST_NOT_FOUND)?;
            Self::change_post_score(account.clone(), post, action)?;
          }
          _ => (),
        }
        let score_diff = Self::get_score_diff(social_account.reputation, action);
        comment.score = comment.score.checked_add(score_diff as i32).ok_or(MSG_OUT_OF_BOUNDS_UPDATING_COMMENT_SCORE)?;
        Self::change_social_account_reputation(comment.created.account.clone(), account.clone(), score_diff, action)?;
        <CommentScoreByAccount<T>>::insert((account, comment_id, action), score_diff);
      }
      <CommentById<T>>::insert(comment_id, comment.clone());
    }

    Ok(())
  }

  pub fn change_social_account_reputation(account: T::AccountId, scorer: T::AccountId, mut score_diff: i16, action: ScoringAction) -> Result {
    let mut social_account = Self::get_or_new_social_account(account.clone());

    if social_account.reputation as i64 + score_diff as i64 <= 1 {
      social_account.reputation = 1;
      score_diff = 0;
    }

    if score_diff < 0 {
      social_account.reputation = social_account.reputation.checked_sub((score_diff * -1) as u32).ok_or(MSG_OUT_OF_BOUNDS_UPDATING_ACCOUNT_REPUTATION)?;
    } else {
      social_account.reputation = social_account.reputation.checked_add(score_diff as u32).ok_or(MSG_OUT_OF_BOUNDS_UPDATING_ACCOUNT_REPUTATION)?;
    }
    
    if Self::account_reputation_diff_by_account((scorer.clone(), account.clone(), action)).is_some() {
      <AccountReputationDiffByAccount<T>>::remove((scorer.clone(), account.clone(), action));
    } else {
      <AccountReputationDiffByAccount<T>>::insert((scorer.clone(), account.clone(), action), score_diff);
    }

    <SocialAccountById<T>>::insert(account.clone(), social_account.clone());

    Self::deposit_event(RawEvent::AccountReputationChanged(account, action, social_account.reputation));

    Ok(())
  }

  pub fn get_score_diff(reputation: u32, action: ScoringAction) -> i16 {
    let r = Self::log_2(reputation);
    let d = (reputation - (2 as u32).pow(r)) * 100 / (2 as u32).pow(r);
    let score_diff = ((r + 1) * 100 + d) / 100;
    
    score_diff as i16 * Self::weight_of_scoring_action(action)
  }

  fn weight_of_scoring_action(action: ScoringAction) -> i16 {
    match action {
      ScoringAction::UpvotePost => Self::upvote_post_action_weight(),
      ScoringAction::DownvotePost => Self::downvote_post_action_weight(),
      ScoringAction::SharePost => Self::share_post_action_weight(),
      ScoringAction::CreateComment => Self::create_comment_action_weight(),
      ScoringAction::UpvoteComment => Self::upvote_comment_action_weight(),
      ScoringAction::DownvoteComment => Self::downvote_comment_action_weight(),
      ScoringAction::ShareComment => Self::share_comment_action_weight(),
      ScoringAction::FollowBlog => Self::follow_blog_action_weight(),
      ScoringAction::FollowAccount => Self::follow_account_action_weight(),
    }
  }

  fn num_bits<P>() -> usize { rstd::mem::size_of::<P>() * 8 }

  pub fn log_2(x: u32) -> u32 {
    assert!(x > 0);
    Self::num_bits::<u32>() as u32 - x.leading_zeros() - 1
  }

  fn is_username_valid(username: Vec<u8>) -> Result {
    ensure!(Self::account_by_profile_username(username.clone()).is_none(), MSG_USERNAME_IS_BUSY);
    ensure!(username.len() >= Self::username_min_len() as usize, MSG_USERNAME_TOO_SHORT);
    ensure!(username.len() <= Self::username_max_len() as usize, MSG_USERNAME_TOO_LONG);
    ensure!(username.iter().all(|&x| x.is_ascii_alphanumeric()), MSG_USERNAME_NOT_ALPHANUMERIC);

    Ok(())
  }

  fn is_ipfs_hash_valid(ipfs_hash: Vec<u8>) -> Result {
    ensure!(ipfs_hash.len() == Self::ipfs_hash_len() as usize, MSG_IPFS_IS_INCORRECT);

    Ok(())
  }

  fn share_post(account: T::AccountId, original_post_id: T::PostId, shared_post_id: T::PostId) -> Result {
    let ref mut original_post = Self::post_by_id(original_post_id).ok_or(MSG_ORIGINAL_POST_NOT_FOUND)?;
    original_post.shares_count = original_post.shares_count.checked_add(1)
      .ok_or(MSG_OVERFLOW_TOTAL_SHARES_SHARING_POST)?;

    let mut shares_by_account = Self::post_shares_by_account((account.clone(), original_post_id));
    shares_by_account = shares_by_account.checked_add(1).ok_or(MSG_OVERFLOW_POST_SHARES_BY_ACCOUNT)?;

    if shares_by_account == 1 {
      Self::change_post_score(account.clone(), original_post, ScoringAction::SharePost)?;
    }

    <PostById<T>>::insert(original_post_id, original_post);
    <PostSharesByAccount<T>>::insert((account.clone(), original_post_id), shares_by_account); // TODO Maybe use mutate instead?
    <SharedPostIdsByOriginalPostId<T>>::mutate(original_post_id, |ids| ids.push(shared_post_id));

    Self::deposit_event(RawEvent::PostShared(account, original_post_id));

    Ok(())
  }

  fn share_comment(account: T::AccountId, original_comment_id: T::CommentId, shared_post_id: T::PostId) -> Result {
    let ref mut original_comment = Self::comment_by_id(original_comment_id).ok_or(MSG_ORIGINAL_COMMENT_NOT_FOUND)?;
    original_comment.shares_count = original_comment.shares_count.checked_add(1)
      .ok_or(MSG_OVERFLOW_TOTAL_SHARES_SHARING_COMMENT)?;

    let mut shares_count = Self::comment_shares_by_account((account.clone(), original_comment_id));
    shares_count = shares_count.checked_add(1).ok_or(MSG_OVERFLOW_COMMENT_SHARES_BY_ACCOUNT)?;

    if shares_count == 1 {
      Self::change_comment_score(account.clone(), original_comment, ScoringAction::ShareComment)?;
    }

    <CommentSharesByAccount<T>>::insert((account.clone(), original_comment_id), shares_count); // TODO Maybe use mutate instead?
    <SharedPostIdsByOriginalCommentId<T>>::mutate(original_comment_id, |ids| ids.push(shared_post_id));

    Self::deposit_event(RawEvent::CommentShared(account, original_comment_id));

    Ok(())
  }
}
