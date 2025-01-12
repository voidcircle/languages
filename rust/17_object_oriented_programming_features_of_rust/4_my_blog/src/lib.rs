use rand::Rng;

pub fn review(posts: &mut [Post]) {
    let _ = posts
        .iter_mut()
        .filter(|current_post| current_post.status == PostStatus::WaitingForReview)
        .map(|current_post| {
            println!("REVIEWING: {}", current_post.title);

            current_post.status = if rand::thread_rng().gen::<bool>() {
                PostStatus::Published
            } else {
                PostStatus::Rejected
            }
        });
}

pub struct BlogService {
    posts: Vec<Post>,
}

pub struct Post {
    pub author: String,
    pub title: String,
    pub content: String,
    pub status: PostStatus,
    id: u16,
}

#[derive(PartialEq, Clone)]
pub enum PostStatus {
    Draft,
    Rejected,
    WaitingForReview,
    Published,
}

impl BlogService {
    pub fn new() -> BlogService {
        BlogService { posts: vec![] }
    }
    pub fn explore(&self) -> Vec<&Post> {
        self.posts
            .iter()
            .filter(|current_post| current_post.status == PostStatus::Published)
            .collect()
    }
    fn generate_new_id(&self) -> u16 {
        loop {
            let new_id: u16 = rand::thread_rng().gen::<u16>();

            match self
                .posts
                .iter()
                .find(|current_post| current_post.id == new_id)
            {
                None => return new_id,
                _ => continue,
            };
        }
    }
    // fn find_post(&self, id: u16) -> Option<&Post> {
    //     self.posts.iter().find(|current_post| current_post.id == id)
    // }
    fn find_post_mut(&mut self, id: u16) -> Option<&mut Post> {
        self.posts
            .iter_mut()
            .find(|current_post| current_post.id == id)
    }
    pub fn create_new_draft(&mut self, author: String, title: String, content: String) -> u16 {
        let new_id: u16 = self.generate_new_id();

        self.posts.push(Post {
            author,
            title,
            content,
            status: PostStatus::Draft,
            id: new_id,
        });

        new_id
    }
    pub fn edit_post(&mut self, post_id: u16, edited_content: String) -> Result<(), &str> {
        if let Some(selected_post) = self.find_post_mut(post_id) {
            if selected_post.status == PostStatus::Draft
                || selected_post.status == PostStatus::Rejected
            {
                selected_post.content = edited_content;

                return Ok(());
            }

            return Err("Post Not Editable.");
        }

        Err("Post Not Found.")
    }
    pub fn request_review(&mut self, post_id: u16) -> Result<(), &str> {
        if let Some(selected_post) = self.find_post_mut(post_id) {
            if selected_post.status == PostStatus::Draft
                || selected_post.status == PostStatus::Rejected
            {
                selected_post.status = PostStatus::WaitingForReview;
                return Ok(());
            }

            return Err("Post Not Editable.");
        }

        Err("Post Not Found.")
    }
}
