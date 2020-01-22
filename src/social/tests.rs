#![cfg(test)]
use super::mock::*;

use super::spaces::*;
use super::defaults::*;
use super::messages::*;

use runtime_io::with_externalities;
use srml_support::*;

const ACCOUNT1 : AccountId = 1;
const ACCOUNT2 : AccountId = 2;

const SPACE1 : SpaceId = 1;
const SPACE2 : SpaceId = 2;

fn space_handle() -> Vec<u8> {
  b"space_handle".to_vec()
}

fn space_ipfs_hash() -> Option<Vec<u8>> {
  Some(b"QmRAQB6YaCyidP37UdDnjFY5vQuiBrcqdyoW1CuDgwxkD4".to_vec())
}

fn space_update(handle: Option<Vec<u8>>, ipfs_hash: Option<Vec<u8>>, hidden: Option<bool>) -> SpaceUpdate {
  SpaceUpdate {
    handle,
    ipfs_hash,
    hidden
  }
}

fn post_ipfs_hash() -> Vec<u8> {
  b"QmRAQB6YaCyidP37UdDnjFY5vQuiBrcqdyoW2CuDgwxkD4".to_vec()
}

fn post_update(space_id: Option<SpaceId>, ipfs_hash: Option<Vec<u8>>, hidden: Option<bool>) -> PostUpdate<Test> {
  PostUpdate {
    space_id,
    ipfs_hash,
    hidden
  }
}

fn comment_ipfs_hash() -> Vec<u8> {
  b"QmRAQB6YaCyidP37UdDnjFY5vQuiBrcqdyoW1CuDgwxkD4".to_vec()
}

fn subcomment_ipfs_hash() -> Vec<u8> {
  b"QmYA2fn8cMbVWo4v95RwcwJVyQsNtnEwHerfWR8UNtEwoE".to_vec()
}

fn comment_update(ipfs_hash: Option<Vec<u8>>, hidden: Option<bool>) -> CommentUpdate {
  CommentUpdate {
    ipfs_hash,
    hidden
  }
}

fn alice_username() -> Vec<u8> {
  b"Alice".to_vec()
}
fn bob_username() -> Vec<u8> {
  b"Bob".to_vec()
}

fn reaction_upvote() -> ReactionKind {
  ReactionKind::Upvote
}
fn reaction_downvote() -> ReactionKind {
  ReactionKind::Downvote
}

fn extension_regular_post() -> PostExtension<Test> {
  PostExtension::RegularPost
}
fn extension_shared_post(post_id: PostId) -> PostExtension<Test> {
  PostExtension::SharedPost(post_id)
}
fn extension_shared_comment(comment_id: CommentId) -> PostExtension<Test> {
  PostExtension::SharedComment(comment_id)
}

fn _create_default_space() -> dispatch::Result {
  _create_space(None, None, None, None)
}

fn _create_space(origin: Option<Origin>, on_behalf: Option<Option<SpaceId>>, handle: Option<Vec<u8>>, ipfs_hash: Option<Option<Vec<u8>>>) -> dispatch::Result {
  Spaces::create_space(
    origin.unwrap_or(Origin::signed(ACCOUNT1)),
    on_behalf.unwrap_or(None),
    handle.unwrap_or(self::space_handle()),
    ipfs_hash.unwrap_or(self::space_ipfs_hash())
  )
}

fn _update_space(origin: Option<Origin>, on_behalf: Option<Option<SpaceId>>, space_id: Option<u32>, update: Option<SpaceUpdate>) -> dispatch::Result {
  Spaces::update_space(
    origin.unwrap_or(Origin::signed(ACCOUNT1)),
    on_behalf.unwrap_or(None),
    space_id.unwrap_or(1),
    update.unwrap_or(self::space_update(None, None, None))
  )
}

fn _default_follow_space() -> dispatch::Result {
  _follow_space(None, None, None)
}

fn _follow_space(origin: Option<Origin>, on_behalf: Option<SpaceId>, space_id: Option<SpaceId>) -> dispatch::Result {
  Spaces::follow_space(
    origin.unwrap_or(Origin::signed(ACCOUNT2)),
    on_behalf.unwrap_or(SPACE2),
    space_id.unwrap_or(1)
  )
}

fn _default_unfollow_space() -> dispatch::Result {
  _unfollow_space(None, None, None)
}

fn _unfollow_space(origin: Option<Origin>, on_behalf: Option<SpaceId>, space_id: Option<SpaceId>) -> dispatch::Result {
  Spaces::unfollow_space(
    origin.unwrap_or(Origin::signed(ACCOUNT2)),
    on_behalf.unwrap_or(SPACE2),
    space_id.unwrap_or(1)
  )
}

fn _create_default_post() -> dispatch::Result {
  _create_post(None, None, None, None, None)
}

fn _create_post(origin: Option<Origin>, on_behalf: Option<Option<SpaceId>>, space_id: Option<SpaceId>, ipfs_hash: Option<Vec<u8>>, extension: Option<PostExtension<Test>>) -> dispatch::Result {
  Spaces::create_post(
    origin.unwrap_or(Origin::signed(ACCOUNT1)),
    on_behalf.unwrap_or(Some(SPACE1)),
    space_id.unwrap_or(1),
    ipfs_hash.unwrap_or(self::post_ipfs_hash()),
    extension.unwrap_or(self::extension_regular_post())
  )
}

fn _update_post(origin: Option<Origin>, on_behalf: Option<Option<SpaceId>>, post_id: Option<PostId>, update: Option<PostUpdate<Test>>) -> dispatch::Result {
  Spaces::update_post(
    origin.unwrap_or(Origin::signed(ACCOUNT1)),
    on_behalf.unwrap_or(Some(SPACE1)),
    post_id.unwrap_or(1),
    update.unwrap_or(self::post_update(None, None, None))
  )
}

fn _create_default_comment() -> dispatch::Result {
  _create_comment(None, None, None, None, None)
}

fn _create_comment(origin: Option<Origin>, on_behalf: Option<Option<SpaceId>>, post_id: Option<PostId>, parent_id: Option<CommentId>, ipfs_hash: Option<Vec<u8>>) -> dispatch::Result {
  Spaces::create_comment(
    origin.unwrap_or(Origin::signed(ACCOUNT1)),
    on_behalf.unwrap_or(Some(SPACE1)),
    post_id.unwrap_or(1),
    parent_id,
    ipfs_hash.unwrap_or(self::comment_ipfs_hash())
  )
}

fn _update_comment(origin: Option<Origin>, on_behalf: Option<Option<SpaceId>>, comment_id: Option<CommentId>, update: Option<CommentUpdate>) -> dispatch::Result {
  Spaces::update_comment(
    origin.unwrap_or(Origin::signed(ACCOUNT1)),
    on_behalf.unwrap_or(Some(SPACE1)),
    comment_id.unwrap_or(1),
    update.unwrap_or(self::comment_update(None, None))
  )
}

fn _create_default_post_reaction() -> dispatch::Result {
  _create_post_reaction(None, None, None, None)
}

fn _create_default_comment_reaction() -> dispatch::Result {
  _create_comment_reaction(None, None, None, None)
}

fn _create_post_reaction(origin: Option<Origin>, on_behalf: Option<Option<SpaceId>>, post_id: Option<PostId>, kind: Option<ReactionKind>) -> dispatch::Result {
  Spaces::create_post_reaction(
    origin.unwrap_or(Origin::signed(ACCOUNT1)),
    on_behalf.unwrap_or(Some(SPACE2)),
    post_id.unwrap_or(1),
    kind.unwrap_or(self::reaction_upvote())
  )
}

fn _create_comment_reaction(origin: Option<Origin>, on_behalf: Option<Option<SpaceId>>, comment_id: Option<CommentId>, kind: Option<ReactionKind>) -> dispatch::Result {
  Spaces::create_comment_reaction(
    origin.unwrap_or(Origin::signed(ACCOUNT1)),
    on_behalf.unwrap_or(Some(SPACE2)),
    comment_id.unwrap_or(1),
    kind.unwrap_or(self::reaction_upvote())
  )
}

// TODO: unused function
fn _update_post_reaction(origin: Option<Origin>, on_behalf: Option<Option<SpaceId>>, post_id: Option<PostId>, reaction_id: ReactionId, kind: Option<ReactionKind>) -> dispatch::Result {
  Spaces::update_post_reaction(
    origin.unwrap_or(Origin::signed(ACCOUNT1)),
    on_behalf.unwrap_or(Some(SPACE2)),
    post_id.unwrap_or(1),
    reaction_id,
    kind.unwrap_or(self::reaction_upvote())
  )
}

// TODO: unused function
fn _update_comment_reaction(origin: Option<Origin>, on_behalf: Option<Option<SpaceId>>, comment_id: Option<CommentId>, reaction_id: ReactionId, kind: Option<ReactionKind>) -> dispatch::Result {
  Spaces::update_comment_reaction(
    origin.unwrap_or(Origin::signed(ACCOUNT1)),
    on_behalf.unwrap_or(Some(SPACE2)),
    comment_id.unwrap_or(1),
    reaction_id,
    kind.unwrap_or(self::reaction_upvote())
  )
}

// TODO: unused function
fn _delete_post_reaction(origin: Option<Origin>, on_behalf: Option<Option<SpaceId>>, post_id: Option<PostId>, reaction_id: ReactionId) -> dispatch::Result {
  Spaces::delete_post_reaction(
    origin.unwrap_or(Origin::signed(ACCOUNT1)),
    on_behalf.unwrap_or(Some(SPACE2)),
    post_id.unwrap_or(1),
    reaction_id
  )
}

// TODO: unused function
fn _delete_comment_reaction(origin: Option<Origin>, on_behalf: Option<Option<SpaceId>>, comment_id: Option<CommentId>, reaction_id: ReactionId) -> dispatch::Result {
  Spaces::delete_comment_reaction(
    origin.unwrap_or(Origin::signed(ACCOUNT1)),
    on_behalf.unwrap_or(Some(SPACE2)),
    comment_id.unwrap_or(1),
    reaction_id
  )
}

// Space tests
#[test]
fn create_space_should_work() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1

    // Check storages
    assert_eq!(Spaces::space_ids_by_owner(ACCOUNT1), vec![1]);
    assert_eq!(Spaces::space_id_by_handle(self::space_handle()), Some(1));
    assert_eq!(Spaces::next_space_id(), 2);

    // Check whether data stored correctly
    let space = Spaces::space_by_id(1).unwrap();

    assert_eq!(space.created.on_behalf.account, ACCOUNT1);
    assert_eq!(space.handle, self::space_handle());
    assert_eq!(space.ipfs_hash, self::space_ipfs_hash());
    assert_eq!(space.posts_count, 0);
    assert_eq!(space.followers_count, 1);
    assert!(space.edit_history.is_empty());
  });
}

#[test]
fn create_space_should_fail_short_handle() {
  let handle : Vec<u8> = vec![97; (DEFAULT_HANDLE_MIN_LEN - 1) as usize];

  with_externalities(&mut build_ext(), || {
    // Try to catch an error creating a space with too short handle
    assert_noop!(_create_space(None, None, Some(handle), None), MSG_SPACE_HANDLE_IS_TOO_SHORT);
  });
}

#[test]
fn create_space_should_fail_long_handle() {
  let handle : Vec<u8> = vec![97; (DEFAULT_HANDLE_MAX_LEN + 1) as usize];

  with_externalities(&mut build_ext(), || {
    // Try to catch an error creating a space with too long handle
    assert_noop!(_create_space(None, None, Some(handle), None), MSG_SPACE_HANDLE_IS_TOO_LONG);
  });
}

#[test]
fn create_space_should_fail_not_unique_handle() {

  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1
    // Try to catch an error creating a space with not unique handle
    assert_noop!(_create_default_space(), MSG_SPACE_HANDLE_IS_NOT_UNIQUE);
  });
}

#[test]
fn create_space_should_fail_invalid_ipfs_hash() {
  let ipfs_hash : Option<Vec<u8>> = Some(b"QmV9tSDx9UiPeWExXEeH6aoDvmihvx6j".to_vec());

  with_externalities(&mut build_ext(), || {
    // Try to catch an error creating a space with invalid ipfs_hash
    assert_noop!(_create_space(None, None, None, Some(ipfs_hash)), MSG_IPFS_IS_INCORRECT);
  });
}

#[test]
fn update_space_should_work() {
  let handle : Vec<u8> = b"new_handle".to_vec();
  let ipfs_hash : Option<Vec<u8>> = Some(b"QmRAQB6YaCyidP37UdDnjFY5vQuiBrcqdyoW2CuDgwxkD4".to_vec());

  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1

    // Space update with ID 1 should be fine
    assert_ok!(_update_space(None, None, None,
      Some(
        self::space_update(
          Some(handle.clone()),
          ipfs_hash.clone(),
          Some(true)
        )
      )
    ));

    // Check whether space updates correctly
    let space = Spaces::space_by_id(1).unwrap();
    assert_eq!(space.handle, handle);
    assert_eq!(space.ipfs_hash, ipfs_hash);
    assert_eq!(space.hidden, true);

    // Check whether history recorded correctly
    assert_eq!(space.edit_history[0].old_data.handle, Some(self::space_handle()));
    assert_eq!(space.edit_history[0].old_data.ipfs_hash, self::space_ipfs_hash());
    assert_eq!(space.edit_history[0].old_data.hidden, Some(false));
  });
}

#[test]
fn update_space_should_fail_nothing_to_update() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1
  
    // Try to catch an error updating a space with no changes
    assert_noop!(_update_space(None, None, None, None), MSG_NOTHING_TO_UPDATE_IN_SPACE);
  });
}

#[test]
fn update_space_should_fail_space_not_found() {
  let handle : Vec<u8> = b"new_handle".to_vec();

  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1
  
    // Try to catch an error updating a space with wrong space ID
    assert_noop!(_update_space(None, None, Some(2),
      Some(
        self::space_update(
          Some(handle),
          None,
          None
        )
      )
    ), MSG_SPACE_NOT_FOUND);
  });
}

#[test]
fn update_space_should_fail_not_an_owner() {
  let handle : Vec<u8> = b"new_handle".to_vec();

  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1
  
    // Try to catch an error updating a space with different account
    assert_noop!(_update_space(Some(Origin::signed(ACCOUNT2)), None, None,
      Some(
        self::space_update(
          Some(handle),
          None,
          None
        )
      )
    ), MSG_ONLY_SPACE_OWNER_CAN_UPDATE_SPACE);
  });
}

#[test]
fn update_space_should_fail_short_handle() {
  let handle : Vec<u8> = vec![97; (DEFAULT_HANDLE_MIN_LEN - 1) as usize];

  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1
  
    // Try to catch an error updating a space with too short handle
    assert_noop!(_update_space(None, None, None,
      Some(
        self::space_update(
          Some(handle),
          None,
          None
        )
      )
    ), MSG_SPACE_HANDLE_IS_TOO_SHORT);
  });
}

#[test]
fn update_space_should_fail_long_handle() {
  let handle : Vec<u8> = vec![97; (DEFAULT_HANDLE_MAX_LEN + 1) as usize];

  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1
  
    // Try to catch an error updating a space with too long handle
    assert_noop!(_update_space(None, None, None,
      Some(
        self::space_update(
          Some(handle),
          None,
          None
        )
      )
    ), MSG_SPACE_HANDLE_IS_TOO_LONG);
  });
}

#[test]
fn update_space_should_fail_not_unique_handle() {
  let handle : Vec<u8> = b"unique_handle".to_vec();

  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1
    
    assert_ok!(_create_space(
      None,
      None,
      Some(handle.clone()),
      None
    )); // SpaceId 2 with a custom handle
  
    // Try to catch an error updating a space on ID 1 with a handle of space on ID 2
    assert_noop!(_update_space(None, None, Some(1),
      Some(
        self::space_update(
          Some(handle),
          None,
          None
        )
      )
    ), MSG_SPACE_HANDLE_IS_NOT_UNIQUE);
  });
}

#[test]
fn update_space_should_fail_invalid_ipfs_hash() {
  let ipfs_hash : Option<Vec<u8>> = Some(b"QmV9tSDx9UiPeWExXEeH6aoDvmihvx6j".to_vec());

  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1
  
    // Try to catch an error updating a space with invalid ipfs_hash
    assert_noop!(_update_space(None, None, None,
      Some(
        self::space_update(
          None,
          ipfs_hash,
          None
        )
      )
    ), MSG_IPFS_IS_INCORRECT);
  });
}

// Post tests
#[test]
fn create_post_should_work() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1
    assert_ok!(_create_default_post()); // PostId 1

    // Check storages
    assert_eq!(Spaces::post_ids_by_space_id(1), vec![1]);
    assert_eq!(Spaces::next_post_id(), 2);

    // Check whether data stored correctly
    let post = Spaces::post_by_id(1).unwrap();

    assert_eq!(post.space_id, 1);
    assert_eq!(post.created.on_behalf.account, ACCOUNT1);
    assert_eq!(post.ipfs_hash, self::post_ipfs_hash());
    assert_eq!(post.comments_count, 0);
    assert_eq!(post.upvotes_count, 0);
    assert_eq!(post.downvotes_count, 0);
    assert_eq!(post.shares_count, 0);
    assert_eq!(post.extension, self::extension_regular_post());
    assert!(post.edit_history.is_empty());
  });
}

#[test]
fn create_post_should_fail_space_not_found() {
  with_externalities(&mut build_ext(), || {
    assert_noop!(_create_default_post(), MSG_SPACE_NOT_FOUND);
  });
}

#[test]
fn create_post_should_fail_invalid_ipfs_hash() {
  let ipfs_hash : Option<Vec<u8>> = Some(b"QmV9tSDx9UiPeWExXEeH6aoDvmihvx6j".to_vec());

  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1

    // Try to catch an error creating a regular post with invalid ipfs_hash
    assert_noop!(_create_post(None, None, None, ipfs_hash, None), MSG_IPFS_IS_INCORRECT);
  });
}

#[test]
fn update_post_should_work() {
  let ipfs_hash : Vec<u8> = b"QmRAQB6YaCyidP37UdDnjFY5vQuiBrcqdyoW1CuDgwxkD4".to_vec();

  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1
    assert_ok!(_create_default_post()); // PostId 1

    // Post update with ID 1 should be fine
    assert_ok!(_update_post(None, None, None,
      Some(
        self::post_update(
          None,
          Some(ipfs_hash.clone()),
          Some(true)
        )
      )
    ));

    
    // Check whether post updates correctly
    let post = Spaces::post_by_id(1).unwrap();
    assert_eq!(post.space_id, 1);
    assert_eq!(post.ipfs_hash, ipfs_hash);
    assert_eq!(post.hidden, true);

    // Check whether history recorded correctly
    assert_eq!(post.edit_history[0].old_data.space_id, None);
    assert_eq!(post.edit_history[0].old_data.ipfs_hash, Some(self::post_ipfs_hash()));
    assert_eq!(post.edit_history[0].old_data.hidden, Some(false));
  });
}

#[test]
fn update_post_should_fail_nothing_to_update() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1
    assert_ok!(_create_default_post()); // PostId 1
  
    // Try to catch an error updating a post with no changes
    assert_noop!(_update_post(None, None, None, None), MSG_NOTHING_TO_UPDATE_IN_POST);
  });
}

#[test]
fn update_post_should_fail_post_not_found() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1
    assert_ok!(_create_space(None, None, Some(b"space2_handle".to_vec()), None)); // SpaceId 2
    assert_ok!(_create_default_post()); // PostId 1
  
    // Try to catch an error updating a post with wrong post ID
    assert_noop!(_update_post(None, None, Some(2),
      Some(
        self::post_update(
          Some(2), 
          None,
          None
        )
      )
    ), MSG_POST_NOT_FOUND);
  });
}

#[test]
fn update_post_should_fail_not_an_owner() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1
    assert_ok!(_create_space(None, None, Some(b"space2_handle".to_vec()), None)); // SpaceId 2
    assert_ok!(_create_default_post()); // PostId 1
  
    // Try to catch an error updating a post with different account
    assert_noop!(_update_post(Some(Origin::signed(ACCOUNT2)), None, None,
      Some(
        self::post_update(
          Some(2), 
          None,
          None
        )
      )
    ), MSG_ONLY_POST_OWNER_CAN_UPDATE_POST);
  });
}

#[test]
fn update_post_should_fail_invalid_ipfs_hash() {
  let ipfs_hash : Vec<u8> = b"QmV9tSDx9UiPeWExXEeH6aoDvmihvx6j".to_vec();

  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1
    assert_ok!(_create_default_post()); // PostId 1
  
    // Try to catch an error updating a post with invalid ipfs_hash
    assert_noop!(_update_post(None, None, None,
      Some(
        self::post_update(
          None,
          Some(ipfs_hash),
          None
        )
      )
    ), MSG_IPFS_IS_INCORRECT);
  });
}

// Comment tests
#[test]
fn create_comment_should_work() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1
    assert_ok!(_create_default_post()); // PostId 1
    assert_ok!(_create_default_comment()); // CommentId 1

    // Check storages
    assert_eq!(Spaces::comment_ids_by_post_id(1), vec![1]);
    assert_eq!(Spaces::next_comment_id(), 2);
    assert_eq!(Spaces::post_by_id(1).unwrap().comments_count, 1);

    // Check whether data stored correctly
    let comment = Spaces::comment_by_id(1).unwrap();

    assert_eq!(comment.parent_id, None);
    assert_eq!(comment.post_id, 1);
    assert_eq!(comment.created.on_behalf.account, ACCOUNT1);
    assert_eq!(comment.ipfs_hash, self::comment_ipfs_hash());
    assert_eq!(comment.upvotes_count, 0);
    assert_eq!(comment.downvotes_count, 0);
    assert_eq!(comment.shares_count, 0);
    assert_eq!(comment.direct_replies_count, 0);
    assert!(comment.edit_history.is_empty());
  });
}

#[test]
fn create_comment_should_work_with_parent() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1
    assert_ok!(_create_default_post()); // PostId 1
    assert_ok!(_create_default_comment()); // CommentId 1
    assert_ok!(_create_comment(None, None, None, Some(1), None)); // CommentId 2 with parent CommentId 1

    // Check storages
    assert_eq!(Spaces::comment_ids_by_post_id(1), vec![1, 2]);
    assert_eq!(Spaces::next_comment_id(), 3);
    assert_eq!(Spaces::post_by_id(1).unwrap().comments_count, 2);

    // Check whether data stored correctly
    assert_eq!(Spaces::comment_by_id(2).unwrap().parent_id, Some(1));
    assert_eq!(Spaces::comment_by_id(1).unwrap().direct_replies_count, 1);
  });
}

#[test]
fn create_comment_should_fail_post_not_found() {
  with_externalities(&mut build_ext(), || {
    // Try to catch an error creating a comment with wrong post
    assert_noop!(_create_default_comment(), MSG_POST_NOT_FOUND);
  });
}

#[test]
fn create_comment_should_fail_parent_not_found() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1
    assert_ok!(_create_default_post()); // PostId 1

    // Try to catch an error creating a comment with wrong parent
    assert_noop!(_create_comment(None, None, None, Some(1), None), MSG_UNKNOWN_PARENT_COMMENT);
  });
}

#[test]
fn create_comment_should_fail_invalid_ipfs_hash() {
  let ipfs_hash : Vec<u8> = b"QmV9tSDx9UiPeWExXEeH6aoDvmihvx6j".to_vec();

  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1
    assert_ok!(_create_default_post()); // PostId 1

    // Try to catch an error creating a comment with wrong parent
    assert_noop!(_create_comment(None, None, None, None, Some(ipfs_hash)), MSG_IPFS_IS_INCORRECT);
  });
}

#[test]
fn update_comment_should_work() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1
    assert_ok!(_create_default_post()); // PostId 1
    assert_ok!(_create_default_comment()); // CommentId 1

    // Post update with ID 1 should be fine
    assert_ok!(_update_comment(
      None,
      None,
      None,
      Some(self::comment_update(Some(self::subcomment_ipfs_hash()), Some(true)))
    ));

    // Check whether post updates correctly
    let comment = Spaces::comment_by_id(1).unwrap();
    assert_eq!(comment.ipfs_hash, self::subcomment_ipfs_hash());
    assert_eq!(comment.hidden, true);

    // Check whether history recorded correctly
    assert_eq!(comment.edit_history[0].old_data.ipfs_hash, Some(self::comment_ipfs_hash()));
    assert_eq!(comment.edit_history[0].old_data.hidden, Some(false));
  });
}

#[test]
fn update_comment_should_fail_comment_not_found() {
  with_externalities(&mut build_ext(), || {
    // Try to catch an error updating a comment with wrong CommentId
    assert_noop!(_update_comment(
      None,
      None,
      None,
      Some(self::comment_update(Some(self::subcomment_ipfs_hash()), None))
    ),
    MSG_COMMENT_NOT_FOUND);
  });
}

#[test]
fn update_comment_should_fail_not_an_owner() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1
    assert_ok!(_create_default_post()); // PostId 1
    assert_ok!(_create_default_comment()); // CommentId 1

    // Try to catch an error updating a comment with wrong Account
    assert_noop!(_update_comment(
      Some(Origin::signed(2)),
      None,
      None,
      Some(self::comment_update(Some(self::subcomment_ipfs_hash()), None))
    ),
    MSG_ONLY_COMMENT_AUTHOR_CAN_UPDATE_COMMENT);
  });
}

#[test]
fn update_comment_should_fail_invalid_ipfs_hash() {
  let ipfs_hash : Option<Vec<u8>> = Some(b"QmV9tSDx9UiPeWExXEeH6aoDvmihvx6j".to_vec());

  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1
    assert_ok!(_create_default_post()); // PostId 1
    assert_ok!(_create_default_comment()); // CommentId 1

    // Try to catch an error updating a comment with invalid ipfs_hash
    assert_noop!(_update_comment(
      None,
      None,
      None,
      Some(self::comment_update(ipfs_hash, None))
    ),
    MSG_IPFS_IS_INCORRECT);
  });
}

// Reaction tests
#[test]
fn create_post_reaction_should_work_upvote() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1
    assert_ok!(_create_default_post()); // PostId 1

    assert_ok!(_create_post_reaction(Some(Origin::signed(ACCOUNT2)), None, None, None)); // ReactionId 1 by ACCOUNT2

    // Check storages
    assert_eq!(Spaces::reaction_ids_by_post_id(1), vec![1]);
    assert_eq!(Spaces::next_reaction_id(), 2);

    // Check post reaction counters
    let post = Spaces::post_by_id(1).unwrap();
    assert_eq!(post.upvotes_count, 1);
    assert_eq!(post.downvotes_count, 0);

    // Check whether data stored correctly
    let reaction = Spaces::reaction_by_id(1).unwrap();
    assert_eq!(reaction.created.on_behalf.account, ACCOUNT2);
    assert_eq!(reaction.kind, self::reaction_upvote());
  });
}

#[test]
fn create_post_reaction_should_work_downvote() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1
    assert_ok!(_create_default_post()); // PostId 1

    assert_ok!(_create_post_reaction(Some(Origin::signed(ACCOUNT2)), None, None, Some(self::reaction_downvote()))); // ReactionId 1 by ACCOUNT2

    // Check storages
    assert_eq!(Spaces::reaction_ids_by_post_id(1), vec![1]);
    assert_eq!(Spaces::next_reaction_id(), 2);

    // Check post reaction counters
    let post = Spaces::post_by_id(1).unwrap();
    assert_eq!(post.upvotes_count, 0);
    assert_eq!(post.downvotes_count, 1);

    // Check whether data stored correctly
    let reaction = Spaces::reaction_by_id(1).unwrap();
    assert_eq!(reaction.created.on_behalf.account, ACCOUNT2);
    assert_eq!(reaction.kind, self::reaction_downvote());
  });
}

#[test]
fn create_post_reaction_should_fail_already_reacted() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1
    assert_ok!(_create_default_post()); // PostId 1
    assert_ok!(_create_default_post_reaction()); // ReactionId1 

    // Try to catch an error creating reaction by the same account
    assert_noop!(_create_default_post_reaction(), MSG_ACCOUNT_ALREADY_REACTED_TO_POST);
  });
}

#[test]
fn create_post_reaction_should_fail_post_not_found() {
  with_externalities(&mut build_ext(), || {
    // Try to catch an error creating reaction by the same account
    assert_noop!(_create_default_post_reaction(), MSG_POST_NOT_FOUND);
  });
}

#[test]
fn create_comment_reaction_should_work_upvote() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1
    assert_ok!(_create_default_post()); // PostId 1
    assert_ok!(_create_default_comment()); // CommentId 1
    assert_ok!(_create_comment_reaction(Some(Origin::signed(ACCOUNT2)), None, None, None)); // ReactionId 1 by ACCOUNT2

    // Check storages
    assert_eq!(Spaces::reaction_ids_by_comment_id(1), vec![1]);
    assert_eq!(Spaces::next_reaction_id(), 2);

    // Check comment reaction counters
    let comment = Spaces::comment_by_id(1).unwrap();
    assert_eq!(comment.upvotes_count, 1);
    assert_eq!(comment.downvotes_count, 0);

    // Check whether data stored correctly
    let reaction = Spaces::reaction_by_id(1).unwrap();
    assert_eq!(reaction.created.on_behalf.account, ACCOUNT2);
    assert_eq!(reaction.kind, self::reaction_upvote());
  });
}

#[test]
fn create_comment_reaction_should_work_downvote() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1
    assert_ok!(_create_default_post()); // PostId 1
    assert_ok!(_create_default_comment()); // CommentId 1
    assert_ok!(_create_comment_reaction(Some(Origin::signed(ACCOUNT2)), None, None, Some(self::reaction_downvote()))); // ReactionId 1 by ACCOUNT2

    // Check storages
    assert_eq!(Spaces::reaction_ids_by_comment_id(1), vec![1]);
    assert_eq!(Spaces::next_reaction_id(), 2);

    // Check comment reaction counters
    let comment = Spaces::comment_by_id(1).unwrap();
    assert_eq!(comment.upvotes_count, 0);
    assert_eq!(comment.downvotes_count, 1);

    // Check whether data stored correctly
    let reaction = Spaces::reaction_by_id(1).unwrap();
    assert_eq!(reaction.created.on_behalf.account, ACCOUNT2);
    assert_eq!(reaction.kind, self::reaction_downvote());
  });
}

#[test]
fn create_comment_reaction_should_fail_already_reacted() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1
    assert_ok!(_create_default_post()); // PostId 1
    assert_ok!(_create_default_comment()); // CommentId 1
    assert_ok!(_create_default_comment_reaction()); // ReactionId 1

    // Try to catch an error creating reaction by the same account
    assert_noop!(_create_default_comment_reaction(), MSG_ACCOUNT_ALREADY_REACTED_TO_COMMENT);
  });
}

#[test]
fn create_comment_reaction_should_fail_comment_not_found() {
  with_externalities(&mut build_ext(), || {
    // Try to catch an error creating reaction by the same account
    assert_noop!(_create_default_comment_reaction(), MSG_COMMENT_NOT_FOUND);
  });
}

// Shares tests

#[test]
fn share_post_should_work() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1
    assert_ok!(_create_space(Some(Origin::signed(ACCOUNT2)), None, Some(b"space2_handle".to_vec()), None)); // SpaceId 2 by ACCOUNT2
    assert_ok!(_create_default_post()); // PostId 1
    assert_ok!(_create_post(
      Some(Origin::signed(ACCOUNT2)),
      None,
      Some(2),
      Some(vec![]),
      Some(self::extension_shared_post(1))
    )); // Share PostId 1 on SpaceId 2 by ACCOUNT2

    // Check storages
    assert_eq!(Spaces::post_ids_by_space_id(1), vec![1]);
    assert_eq!(Spaces::post_ids_by_space_id(2), vec![2]);
    assert_eq!(Spaces::next_post_id(), 3);

    assert_eq!(Spaces::post_shares_by_account((ACCOUNT2, 1)), 1);
    assert_eq!(Spaces::shared_post_ids_by_original_post_id(1), vec![2]);

    // Check whether data stored correctly
    assert_eq!(Spaces::post_by_id(1).unwrap().shares_count, 1);

    let shared_post = Spaces::post_by_id(2).unwrap();

    assert_eq!(shared_post.space_id, 2);
    assert_eq!(shared_post.created.on_behalf.account, ACCOUNT2);
    assert!(shared_post.ipfs_hash.is_empty());
    assert_eq!(shared_post.extension, self::extension_shared_post(1));
  });
}

#[test]
fn share_post_should_work_share_own_post() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1
    assert_ok!(_create_default_post()); // PostId 1
    assert_ok!(_create_post(
      Some(Origin::signed(ACCOUNT1)),
      None,
      Some(1),
      Some(vec![]),
      Some(self::extension_shared_post(1))
    )); // Share PostId 1

    // Check storages
    assert_eq!(Spaces::post_ids_by_space_id(1), vec![1, 2]);
    assert_eq!(Spaces::next_post_id(), 3);

    assert_eq!(Spaces::post_shares_by_account((ACCOUNT1, 1)), 1);
    assert_eq!(Spaces::shared_post_ids_by_original_post_id(1), vec![2]);

    // Check whether data stored correctly
    assert_eq!(Spaces::post_by_id(1).unwrap().shares_count, 1);

    let shared_post = Spaces::post_by_id(2).unwrap();
    assert_eq!(shared_post.space_id, 1);
    assert_eq!(shared_post.created.on_behalf.account, ACCOUNT1);
    assert!(shared_post.ipfs_hash.is_empty());
    assert_eq!(shared_post.extension, self::extension_shared_post(1));
  });
}

#[test]
fn share_post_should_fail_original_post_not_found() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1
    assert_ok!(_create_space(Some(Origin::signed(ACCOUNT2)), None, Some(b"space2_handle".to_vec()), None)); // SpaceId 2 by ACCOUNT2
    // Skipped creating PostId 1
    assert_noop!(_create_post(
      Some(Origin::signed(ACCOUNT2)),
      None,
      Some(2),
      Some(vec![]),
      Some(self::extension_shared_post(1))),
      
    MSG_ORIGINAL_POST_NOT_FOUND);
  });
}

#[test]
fn share_post_should_fail_share_shared_post() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1
    assert_ok!(_create_space(Some(Origin::signed(ACCOUNT2)), None, Some(b"space2_handle".to_vec()), None)); // SpaceId 2 by ACCOUNT2
    assert_ok!(_create_default_post());
    assert_ok!(_create_post(
      Some(Origin::signed(ACCOUNT2)),
      None,
      Some(2),
      Some(vec![]),
      Some(self::extension_shared_post(1)))
    );

    // Try to share post with extension SharedPost
    assert_noop!(_create_post(
      Some(Origin::signed(ACCOUNT1)),
      None,
      Some(1),
      Some(vec![]),
      Some(self::extension_shared_post(2))),
      
    MSG_CANNOT_SHARE_SHARED_POST);
  });
}

#[test]
fn share_comment_should_work() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1
    assert_ok!(_create_space(Some(Origin::signed(ACCOUNT2)), None, Some(b"space2_handle".to_vec()), None)); // SpaceId 2 by ACCOUNT2
    assert_ok!(_create_default_post()); // PostId 1
    assert_ok!(_create_default_comment()); // CommentId 1
    assert_ok!(_create_post(
      Some(Origin::signed(ACCOUNT2)),
      None,
      Some(2),
      Some(vec![]),
      Some(self::extension_shared_comment(1))
    )); // Share CommentId 1 on SpaceId 2 by ACCOUNT2

    // Check storages
    assert_eq!(Spaces::post_ids_by_space_id(1), vec![1]);
    assert_eq!(Spaces::post_ids_by_space_id(2), vec![2]);
    assert_eq!(Spaces::next_post_id(), 3);

    assert_eq!(Spaces::comment_shares_by_account((ACCOUNT2, 1)), 1);
    assert_eq!(Spaces::shared_post_ids_by_original_comment_id(1), vec![2]);

    // Check whether data stored correctly
    assert_eq!(Spaces::comment_by_id(1).unwrap().shares_count, 1);

    let shared_post = Spaces::post_by_id(2).unwrap();

    assert_eq!(shared_post.space_id, 2);
    assert_eq!(shared_post.created.on_behalf.account, ACCOUNT2);
    assert!(shared_post.ipfs_hash.is_empty());
    assert_eq!(shared_post.extension, self::extension_shared_comment(1));
  });
}

#[test]
fn share_comment_should_fail_original_comment_not_found() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1
    assert_ok!(_create_space(Some(Origin::signed(ACCOUNT2)), None, Some(b"space2_handle".to_vec()), None)); // SpaceId 2 by ACCOUNT2
    assert_ok!(_create_default_post()); // PostId 1
    // Skipped creating CommentId 1
    assert_noop!(_create_post(
      Some(Origin::signed(ACCOUNT2)),
      None,
      Some(2),
      Some(vec![]),
      Some(self::extension_shared_comment(1))),
      
    MSG_ORIGINAL_COMMENT_NOT_FOUND);
  });
}

// Space following tests

#[test]
fn follow_space_should_work() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1

    assert_ok!(_default_follow_space()); // Follow SpaceId 1 by ACCOUNT2

    assert_eq!(Spaces::space_by_id(1).unwrap().followers_count, 2);
    assert_eq!(Spaces::space_followers(1), vec![SPACE1, SPACE2]);
    assert_eq!(Spaces::space_followed_by_space((SPACE2, SPACE1)), true);
  });
}

#[test]
fn follow_space_should_fail_space_not_found() {
  with_externalities(&mut build_ext(), || {
    assert_noop!(_default_follow_space(), MSG_SPACE_NOT_FOUND);
  });
}

#[test]
fn follow_space_should_fail_already_following() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1
    assert_ok!(_default_follow_space()); // Follow SpaceId 1 by ACCOUNT2

    assert_noop!(_default_follow_space(), MSG_ACCOUNT_IS_FOLLOWING_SPACE);
  });
}

#[test]
fn unfollow_space_should_work() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1

    assert_ok!(_default_follow_space()); // Follow SpaceId 1 by ACCOUNT2
    assert_ok!(_default_unfollow_space());

    assert_eq!(Spaces::space_by_id(1).unwrap().followers_count, 1);
    assert_eq!(Spaces::space_followers(1), vec![SPACE1]);
  });
}

#[test]
fn unfollow_space_should_fail_space_not_found() {
  with_externalities(&mut build_ext(), || {
    assert_noop!(_default_unfollow_space(), MSG_SPACE_NOT_FOUND);
  });
}

#[test]
fn unfollow_space_should_fail_already_following() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1

    assert_noop!(_default_unfollow_space(), MSG_ACCOUNT_IS_NOT_FOLLOWING_SPACE);
  });
}
