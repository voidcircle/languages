use blog::{DraftPost, PendingReviewPost, Post, PublishedPost};

fn main() {
    let mut post: DraftPost = Post::new();

    post.add_text("I ate a salad for lunch today.");

    // we cannot get content in any ways since...
    // content itself is a private field,
    // and there is no methods that return the content of the content
    // post.content

    let reviewed_post: PendingReviewPost = post.request_review();

    let published_post: PublishedPost = reviewed_post.approve();

    // we cannot even accidently try to get the content of unpublished thing.
    // assert_eq!("", post.get_content())
    // assert_eq!("", reviewed_post.get_content())

    // but in the case of the published post, we can.
    assert_eq!("", published_post.get_content());

    // the disadvantage of using this kind of design pattern is that we cannot make anything that
    // accepts this and use the content when the post is NOT on the published status.
}
