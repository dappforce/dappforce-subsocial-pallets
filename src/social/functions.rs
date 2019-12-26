use super::spaces::*;
use super::messages::*;

use rstd::prelude::*;
use srml_support::{StorageMap, StorageValue, dispatch::Result, ensure};
use runtime_primitives::traits::{As};
use system::{self};
use {timestamp};

impl<T: Trait> Module<T> {

  pub fn ensure_space_exists(space_id: T::SpaceId) -> Result {
    ensure!(<SpaceById<T>>::exists(space_id), MSG_SPACE_NOT_FOUND);
    Ok(())
  }

  pub fn new_change(account: T::AccountId) -> Change<T> {
    Change {
      account,
      block: <system::Module<T>>::block_number(),
      time: <timestamp::Module<T>>::now(),
    }
  }

  // TODO: maybe don't add reaction in storage before checks in 'create_reaction' are done?
  pub fn new_reaction(account: T::AccountId, kind: ReactionKind) -> T::ReactionId {
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

  pub fn add_space_follower_and_insert_space(
    follower: T::AccountId,
    space: &mut Space<T>,
    is_new_space: bool
  ) -> Result {

    let space_id = space.id;
    let mut social_account = Self::get_or_new_social_account(follower.clone());
    social_account.following_spaces_count = social_account.following_spaces_count
      .checked_add(1)
      .ok_or(MSG_OVERFLOW_FOLLOWING_SPACE)?;

    space.followers_count = space.followers_count.checked_add(1).ok_or(MSG_OVERFLOW_FOLLOWING_SPACE)?;
    if space.created.account != follower {
      let author = space.created.account.clone();
      let score_diff = Self::get_score_diff(social_account.reputation, ScoringAction::FollowSpace);
      space.score = space.score.checked_add(score_diff as i32).ok_or(MSG_OUT_OF_BOUNDS_UPDATING_SPACE_SCORE)?;
      Self::change_social_account_reputation(author.clone(), follower.clone(), score_diff, ScoringAction::FollowSpace)?;
    }

    <SpaceById<T>>::insert(space_id, space);
    <SocialAccountById<T>>::insert(follower.clone(), social_account.clone());
    <SpacesFollowedByAccount<T>>::mutate(follower.clone(), |ids| ids.push(space_id));
    <SpaceFollowers<T>>::mutate(space_id, |ids| ids.push(follower.clone()));
    <SpaceFollowedByAccount<T>>::insert((follower.clone(), space_id), true);

    if is_new_space {
      Self::deposit_event(RawEvent::SpaceCreated(follower.clone(), space_id));
    }

    Self::deposit_event(RawEvent::SpaceFollowed(follower, space_id));
    
    Ok(())
  }

  pub fn get_or_new_social_account(account: T::AccountId) -> SocialAccount<T> {
    if let Some(social_account) = Self::social_account_by_id(account) {
      social_account
    } else {
      SocialAccount {
        followers_count: 0,
        following_accounts_count: 0,
        following_spaces_count: 0,
        reputation: 1,
        profile: None
      }
    }
  }

  pub fn vec_remove_on<F: PartialEq>(vector: &mut Vec<F>, element: F) {
    if let Some(index) = vector.iter().position(|x| *x == element) {
      vector.swap_remove(index);
    }
  }

  pub fn change_post_score(account: T::AccountId, post: &mut Post<T>, action: ScoringAction) -> Result {
    let social_account = Self::get_or_new_social_account(account.clone());
    <SocialAccountById<T>>::insert(account.clone(), social_account.clone());

    let post_id = post.id;
    let mut space = Self::space_by_id(post.space_id).ok_or(MSG_SPACE_NOT_FOUND)?;
    
    if post.created.account != account {
      if let Some(score_diff) = Self::post_score_by_account((account.clone(), post_id, action)) {
        let reputation_diff = Self::account_reputation_diff_by_account((account.clone(), post.created.account.clone(), action)).ok_or(MSG_REPUTATION_DIFF_NOT_FOUND)?;
        post.score = post.score.checked_add(score_diff as i32 * -1).ok_or(MSG_OUT_OF_BOUNDS_REVERTING_POST_SCORE)?;
        space.score = space.score.checked_add(score_diff as i32 * -1).ok_or(MSG_OUT_OF_BOUNDS_REVERTING_SPACE_SCORE)?;
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
        space.score = space.score.checked_add(score_diff as i32).ok_or(MSG_OUT_OF_BOUNDS_UPDATING_SPACE_SCORE)?;
        Self::change_social_account_reputation(post.created.account.clone(), account.clone(), score_diff, action)?;
        <PostScoreByAccount<T>>::insert((account.clone(), post_id, action), score_diff);
      }

      <PostById<T>>::insert(post_id, post.clone());
      <SpaceById<T>>::insert(post.space_id, space.clone());
    }

    Ok(())
  }

  pub fn change_comment_score(account: T::AccountId, comment: &mut Comment<T>, action: ScoringAction) -> Result {
    let social_account = Self::get_or_new_social_account(account.clone());
    <SocialAccountById<T>>::insert(account.clone(), social_account.clone());

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

  // TODO write unit tests for this method.
  pub fn weight_of_scoring_action(action: ScoringAction) -> i16 {
    match action {
      ScoringAction::UpvotePost => Self::upvote_post_action_weight(),
      ScoringAction::DownvotePost => Self::downvote_post_action_weight(),
      ScoringAction::SharePost => Self::share_post_action_weight(),
      ScoringAction::CreateComment => Self::create_comment_action_weight(),
      ScoringAction::UpvoteComment => Self::upvote_comment_action_weight(),
      ScoringAction::DownvoteComment => Self::downvote_comment_action_weight(),
      ScoringAction::ShareComment => Self::share_comment_action_weight(),
      ScoringAction::FollowSpace => Self::follow_space_action_weight(),
      ScoringAction::FollowAccount => Self::follow_account_action_weight(),
    }
  }

  fn num_bits<P>() -> usize { rstd::mem::size_of::<P>() * 8 }

  pub fn log_2(x: u32) -> u32 {
    assert!(x > 0);
    Self::num_bits::<u32>() as u32 - x.leading_zeros() - 1
  }

  pub fn is_username_valid(username: Vec<u8>) -> Result {
    ensure!(Self::account_by_profile_username(username.clone()).is_none(), MSG_USERNAME_IS_BUSY);
    ensure!(username.len() >= Self::username_min_len() as usize, MSG_USERNAME_TOO_SHORT);
    ensure!(username.len() <= Self::username_max_len() as usize, MSG_USERNAME_TOO_LONG);
    ensure!(username.iter().all(|&x| x.is_ascii_alphanumeric()), MSG_USERNAME_NOT_ALPHANUMERIC);

    Ok(())
  }

  pub fn is_ipfs_hash_valid(ipfs_hash: Vec<u8>) -> Result {
    ensure!(ipfs_hash.len() == Self::ipfs_hash_len() as usize, MSG_IPFS_IS_INCORRECT);

    Ok(())
  }

  pub fn share_post(account: T::AccountId, original_post_id: T::PostId, shared_post_id: T::PostId) -> Result {
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

  pub fn share_comment(account: T::AccountId, original_comment_id: T::CommentId, shared_post_id: T::PostId) -> Result {
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
