pub const MSG_SPACE_NOT_FOUND: &str = "Space was not found by id";
pub const MSG_ON_BEHALF_SPACE_NOT_FOUND: &str = "The space on whose behalf you are acting was not found by id";
pub const MSG_SPACE_HANDLE_IS_TOO_SHORT: &str = "Space handle is too short";
pub const MSG_SPACE_HANDLE_IS_TOO_LONG: &str = "Space handle is too long";
pub const MSG_SPACE_HANDLE_IS_NOT_UNIQUE: &str = "Space handle is not unique";
pub const MSG_SPACE_HANDLE_CONTAINS_INVALID_CHAR: &str = "Space handle is not alphanumeric";
pub const MSG_NOTHING_TO_UPDATE_IN_SPACE: &str = "Nothing to update in a space";
pub const MSG_ONLY_SPACE_OWNER_CAN_UPDATE_SPACE: &str = "Only a space owner can update their space";
pub const MSG_ACCOUNT_IS_NOT_A_SPACE_OWNER: &str = "Account is not a space owner";

pub const MSG_POST_NOT_FOUND: &str = "Post was not found by id";
pub const MSG_NOTHING_TO_UPDATE_IN_POST: &str = "Nothing to update in a post";
pub const MSG_ONLY_POST_OWNER_CAN_UPDATE_POST: &str = "Only post owner can update their post";
pub const MSG_OVERFLOW_ADDING_POST_ON_SPACE: &str = "Overflow adding post on space";

pub const MSG_COMMENT_NOT_FOUND: &str = "Comment was not found by id";
pub const MSG_NOTHING_TO_UPDATE_IN_COMMENT: &str = "Nothing to update in comment";
pub const MSG_UNKNOWN_PARENT_COMMENT: &str = "Unknown parent comment id";
pub const MSG_ONLY_COMMENT_AUTHOR_CAN_UPDATE_COMMENT: &str = "Only comment author can update their comment";
pub const MSG_OVERFLOW_ADDING_COMMENT_ON_POST: &str = "Overflow adding comment on post";
pub const MSG_OVERFLOW_REPLYING_ON_COMMENT: &str = "Overflow replying on comment";

pub const MSG_REACTION_NOT_FOUND: &str = "Reaction was not found by id";
pub const MSG_ACCOUNT_ALREADY_REACTED_TO_POST: &str = "Account has already reacted to this post. To change a kind of reaction call update_post_reaction()";
pub const MSG_ACCOUNT_HAS_NOT_REACTED_TO_POST: &str = "Account has not reacted to this post yet. Use create_post_reaction()";
pub const MSG_NO_POST_REACTION_BY_ACCOUNT_TO_DELETE: &str = "There is no post reaction by account that could be deleted";
pub const MSG_OVERFLOW_UPVOTING_POST: &str = "Overflow upvoting post";
pub const MSG_OVERFLOW_DOWNVOTING_POST: &str = "Overflow downvoting post";
pub const MSG_ACCOUNT_ALREADY_REACTED_TO_COMMENT: &str = "Account has already reacted to this comment. To change a kind of reaction call pub update_comment_reaction()";
pub const MSG_ACCOUNT_HAS_NOT_REACTED_TO_COMMENT: &str = "Account has not reacted to this comment yet. Use create_comment_reaction()";
pub const MSG_NO_COMMENT_REACTION_BY_ACCOUNT_TO_DELETE: &str = "There is no comment reaction by account that could be deleted";
pub const MSG_OVERFLOW_UPVOTING_COMMENT: &str = "Overflow upvoting comment";
pub const MSG_OVERFLOW_DOWNVOTING_COMMENT: &str = "Overflow downvoting comment";
pub const MSG_ONLY_REACTION_OWNER_CAN_UPDATE_REACTION: &str = "Only reaction owner can update their reaction";
pub const MSG_NEW_REACTION_KIND_DO_NOT_DIFFER: &str = "New reaction kind is the same as old one";

pub const MSG_ACCOUNT_IS_FOLLOWING_SPACE: &str = "Account is already following this space";
pub const MSG_ACCOUNT_IS_NOT_FOLLOWING_SPACE: &str = "Account is not following this space";
pub const MSG_ACCOUNT_CANNOT_FOLLOW_ITSELF: &str = "Account can not follow itself";
pub const MSG_ACCOUNT_CANNOT_UNFOLLOW_ITSELF: &str = "Account can not unfollow itself";
pub const MSG_ACCOUNT_IS_ALREADY_FOLLOWED: &str = "Account is already followed";
pub const MSG_ACCOUNT_IS_NOT_FOLLOWED: &str = "Account is not followed by follower";
pub const MSG_UNDERFLOW_UNFOLLOWING_SPACE: &str = "Underflow unfollowing space";
pub const MSG_OVERFLOW_FOLLOWING_SPACE: &str = "Overflow following space";
pub const MSG_OVERFLOW_FOLLOWING_ACCOUNT: &str = "Overflow following account";
pub const MSG_UNDERFLOW_UNFOLLOWING_ACCOUNT: &str = "Overflow following account";

pub const MSG_SOCIAL_ACCOUNT_NOT_FOUND: &str = "Social account was not found by id";
pub const MSG_FOLLOWER_ACCOUNT_NOT_FOUND: &str = "Follower social account was not found by id";
pub const MSG_FOLLOWED_ACCOUNT_NOT_FOUND: &str = "Followed social account was not found by id";

pub const MSG_IPFS_IS_INCORRECT: &str = "IPFS-hash is not correct";

/*
pub const MSG_OUT_OF_BOUNDS_UPDATING_SPACE_SCORE: &str = "Out of bounds updating space score";
pub const MSG_OUT_OF_BOUNDS_REVERTING_SPACE_SCORE: &str = "Out of bounds reverting space score";
pub const MSG_OUT_OF_BOUNDS_UPDATING_POST_SCORE: &str = "Out of bounds updating post score";
pub const MSG_OUT_OF_BOUNDS_REVERTING_POST_SCORE: &str = "Out of bounds reverting post score";
pub const MSG_OUT_OF_BOUNDS_UPDATING_COMMENT_SCORE: &str = "Out of bounds updating comment score";
pub const MSG_OUT_OF_BOUNDS_REVERTING_COMMENT_SCORE: &str = "Out of bounds reverting comment score";
pub const MSG_OUT_OF_BOUNDS_UPDATING_ACCOUNT_REPUTATION: &str = "Out of bounds updating social account reputation";
pub const MSG_REPUTATION_DIFF_NOT_FOUND: &str = "Scored account reputation difference by account and action not found";
*/

pub const MSG_ORIGINAL_POST_NOT_FOUND: &str = "Original post not found when sharing";
pub const MSG_OVERFLOW_TOTAL_SHARES_SHARING_POST: &str = "Overflow total shares counter when sharing post";
pub const MSG_OVERFLOW_POST_SHARES_BY_ACCOUNT: &str = "Overflow shares by account counter when sharing post";
pub const MSG_CANNOT_SHARE_SHARED_POST: &str = "Cannot share post that is not regular post";
pub const MSG_ORIGINAL_COMMENT_NOT_FOUND: &str = "Original comment not found when sharing";
pub const MSG_OVERFLOW_TOTAL_SHARES_SHARING_COMMENT: &str = "Overflow total shares counter when sharing comment";
pub const MSG_OVERFLOW_COMMENT_SHARES_BY_ACCOUNT: &str = "Overflow shares by account counter when sharing comment";
