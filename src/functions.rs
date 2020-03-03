use super::*;

use frame_support::{dispatch::DispatchResult};

impl<T: Trait> Module<T> {

    pub fn ensure_blog_exists(blog_id: BlogId) -> DispatchResult {
        ensure!(<BlogById<T>>::exists(blog_id), Error::<T>::BlogNotFound);
        Ok(())
    }

    pub fn new_change(account: T::AccountId) -> Change<T> {
        Change {
            account,
            block: <system::Module<T>>::block_number(),
            time: <pallet_timestamp::Module<T>>::now(),
        }
    }

    // TODO: maybe don't add reaction in storage before checks in 'create_reaction' are done?
    pub fn new_reaction(account: T::AccountId, kind: ReactionKind) -> ReactionId {
        let reaction_id = Self::next_reaction_id();
        let new_reaction: Reaction<T> = Reaction {
            id: reaction_id,
            created: Self::new_change(account),
            updated: None,
            kind
        };

        <ReactionById<T>>::insert(reaction_id, new_reaction);
        NextReactionId::mutate(|n| { *n += 1; });

        reaction_id
    }

    pub fn add_blog_follower_and_insert_blog(
        follower: T::AccountId,
        blog: &mut Blog<T>,
        is_new_blog: bool
    ) -> DispatchResult {

        let blog_id = blog.id;
        let mut social_account = Self::get_or_new_social_account(follower.clone());
        social_account.following_blogs_count = social_account.following_blogs_count
            .checked_add(1)
            .ok_or(Error::<T>::OverflowFollowingBlog)?;

        blog.followers_count = blog.followers_count.checked_add(1).ok_or(Error::<T>::OverflowFollowingBlog)?;
        if blog.created.account != follower {
            let author = blog.created.account.clone();
            let score_diff = Self::get_score_diff(social_account.reputation, ScoringAction::FollowBlog);
            blog.score = blog.score.checked_add(score_diff as i32).ok_or(Error::<T>::OutOfBoundsUpdatingBlogScore)?;
            Self::change_social_account_reputation(author, follower.clone(), score_diff, ScoringAction::FollowBlog)?;
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

    pub fn get_or_new_social_account(account: T::AccountId) -> SocialAccount<T> {
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

    pub fn vec_remove_on<F: PartialEq>(vector: &mut Vec<F>, element: F) {
        if let Some(index) = vector.iter().position(|x| *x == element) {
            vector.swap_remove(index);
        }
    }

    pub fn change_post_score(account: T::AccountId, post: &mut Post<T>, action: ScoringAction) -> DispatchResult {
        let social_account = Self::get_or_new_social_account(account.clone());
        <SocialAccountById<T>>::insert(account.clone(), social_account.clone());

        let post_id = post.id;
        let mut blog = Self::blog_by_id(post.blog_id).ok_or(Error::<T>::BlogNotFound)?;

        if post.created.account != account {
            if let Some(score_diff) = Self::post_score_by_account((account.clone(), post_id, action)) {
                let reputation_diff = Self::account_reputation_diff_by_account((account.clone(), post.created.account.clone(), action)).ok_or(Error::<T>::ReputationDiffNotFound)?;
                post.score = post.score.checked_add(score_diff as i32 * -1).ok_or(Error::<T>::OutOfBoundsRevertingPostScore)?;
                blog.score = blog.score.checked_add(score_diff as i32 * -1).ok_or(Error::<T>::OutOfBoundsRevertingBlogScore)?;
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
                post.score = post.score.checked_add(score_diff as i32).ok_or(Error::<T>::OutOfBoundsUpdatingPostScore)?;
                blog.score = blog.score.checked_add(score_diff as i32).ok_or(Error::<T>::OutOfBoundsUpdatingBlogScore)?;
                Self::change_social_account_reputation(post.created.account.clone(), account.clone(), score_diff, action)?;
                <PostScoreByAccount<T>>::insert((account.clone(), post_id, action), score_diff);
            }

            <PostById<T>>::insert(post_id, post.clone());
            <BlogById<T>>::insert(post.blog_id, blog.clone());
        }

        Ok(())
    }

    pub fn change_comment_score(account: T::AccountId, comment: &mut Comment<T>, action: ScoringAction) -> DispatchResult {
        let social_account = Self::get_or_new_social_account(account.clone());
        <SocialAccountById<T>>::insert(account.clone(), social_account.clone());

        let comment_id = comment.id;

        if comment.created.account != account {
            if let Some(score_diff) = Self::comment_score_by_account((account.clone(), comment_id, action)) {
                let reputation_diff = Self::account_reputation_diff_by_account((account.clone(), comment.created.account.clone(), action)).ok_or(Error::<T>::ReputationDiffNotFound)?;
                comment.score = comment.score.checked_add(score_diff as i32 * -1).ok_or(Error::<T>::OutOfBoundsRevertingCommentScore)?;
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
                        let ref mut post = Self::post_by_id(comment.post_id).ok_or(Error::<T>::PostNotFound)?;
                        Self::change_post_score(account.clone(), post, action)?;
                    }
                    _ => (),
                }
                let score_diff = Self::get_score_diff(social_account.reputation, action);
                comment.score = comment.score.checked_add(score_diff as i32).ok_or(Error::<T>::OutOfBoundsUpdatingCommentScore)?;
                Self::change_social_account_reputation(comment.created.account.clone(), account.clone(), score_diff, action)?;
                <CommentScoreByAccount<T>>::insert((account, comment_id, action), score_diff);
            }
            <CommentById<T>>::insert(comment_id, comment.clone());
        }

        Ok(())
    }

    pub fn change_social_account_reputation(account: T::AccountId, scorer: T::AccountId, mut score_diff: i16, action: ScoringAction) -> DispatchResult {
        let mut social_account = Self::get_or_new_social_account(account.clone());

        if social_account.reputation as i64 + score_diff as i64 <= 1 {
            social_account.reputation = 1;
            score_diff = 0;
        }

        if score_diff < 0 {
            social_account.reputation = social_account.reputation.checked_sub((score_diff * -1) as u32).ok_or(Error::<T>::OutOfBoundsUpdatingAccountReputation)?;
        } else {
            social_account.reputation = social_account.reputation.checked_add(score_diff as u32).ok_or(Error::<T>::OutOfBoundsUpdatingAccountReputation)?;
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
            ScoringAction::FollowBlog => Self::follow_blog_action_weight(),
            ScoringAction::FollowAccount => Self::follow_account_action_weight(),
        }
    }

    fn num_bits<P>() -> usize { sp_std::mem::size_of::<P>() * 8 }

    pub fn log_2(x: u32) -> u32 {
        assert!(x > 0);
        Self::num_bits::<u32>() as u32 - x.leading_zeros() - 1
    }

    pub fn is_username_valid(username: Vec<u8>) -> DispatchResult {
        ensure!(Self::account_by_profile_username(username.clone()).is_none(), Error::<T>::UsernameIsBusy);
        ensure!(username.len() >= Self::username_min_len() as usize, Error::<T>::UsernameIsTooShort);
        ensure!(username.len() <= Self::username_max_len() as usize, Error::<T>::UsernameIsTooLong);
        ensure!(username.iter().all(|&x| x.is_ascii_alphanumeric()), Error::<T>::UsernameIsNotAlphanumeric);

        Ok(())
    }

    pub fn is_ipfs_hash_valid(ipfs_hash: Vec<u8>) -> DispatchResult {
        ensure!(ipfs_hash.len() == Self::ipfs_hash_len() as usize, Error::<T>::IpfsIsIncorrect);

        Ok(())
    }

    pub fn share_post(account: T::AccountId, original_post_id: PostId, shared_post_id: PostId) -> DispatchResult {
        let ref mut original_post = Self::post_by_id(original_post_id).ok_or(Error::<T>::OriginalPostNotFound)?;
        original_post.shares_count = original_post.shares_count.checked_add(1)
            .ok_or(Error::<T>::OverflowTotalSharesSharingPost)?;

        let mut shares_by_account = Self::post_shares_by_account((account.clone(), original_post_id));
        shares_by_account = shares_by_account.checked_add(1).ok_or(Error::<T>::OverflowPostSharesSharingPost)?;

        if shares_by_account == 1 {
            Self::change_post_score(account.clone(), original_post, ScoringAction::SharePost)?;
        }

        <PostById<T>>::insert(original_post_id, original_post);
        <PostSharesByAccount<T>>::insert((account.clone(), original_post_id), shares_by_account); // TODO Maybe use mutate instead?
        SharedPostIdsByOriginalPostId::mutate(original_post_id, |ids| ids.push(shared_post_id));

        Self::deposit_event(RawEvent::PostShared(account, original_post_id));

        Ok(())
    }

    pub fn share_comment(account: T::AccountId, original_comment_id: CommentId, shared_post_id: PostId) -> DispatchResult {
        let ref mut original_comment = Self::comment_by_id(original_comment_id).ok_or(Error::<T>::OriginalCommentNotFound)?;
        original_comment.shares_count = original_comment.shares_count.checked_add(1)
            .ok_or(Error::<T>::OverflowTotalSharesSharingComment)?;

        let mut shares_count = Self::comment_shares_by_account((account.clone(), original_comment_id));
        shares_count = shares_count.checked_add(1).ok_or(Error::<T>::OverflowCommentSharesByAccount)?;

        if shares_count == 1 {
            Self::change_comment_score(account.clone(), original_comment, ScoringAction::ShareComment)?;
        }

        <CommentSharesByAccount<T>>::insert((account.clone(), original_comment_id), shares_count); // TODO Maybe use mutate instead?
        SharedPostIdsByOriginalCommentId::mutate(original_comment_id, |ids| ids.push(shared_post_id));

        Self::deposit_event(RawEvent::CommentShared(account, original_comment_id));

        Ok(())
    }
}
