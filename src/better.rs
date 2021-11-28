struct Post {
    content: String,
}

impl Post {
    fn init() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }
    fn content(&self) -> &str {
        &self.content
    }
}

struct DraftPost {
    content: String,
}

impl DraftPost {
    fn add_text(&mut self, text: &str) {
        self.content.push_str(text)
    }
    fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
            approve_count: 0,
        }
    }
}

struct PendingReviewPost {
    approve_count: u32,
    content: String,
}

enum ReviewResult {
    Post(Post),
    Draft(PendingReviewPost),
}

impl PendingReviewPost {
    fn approve(mut self) -> ReviewResult {
        self.approve_count += 1;
        if self.approve_count >= 2 {
            ReviewResult::Post(Post {
                content: self.content,
            })
        } else {
            ReviewResult::Draft(self)
        }
    }
    fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn run() {
        let mut post = Post::init();

        post.add_text("I ate a salad for lunch today. ");
        // This assertion statement no longer becomes needed, since we cannot view the contents of drafts
        // assert_eq!("", post.content());

        // The fact that we have to set `let post = post.(...)` makes this implementation not quite OOP anymore, since the methods are *not* encapsulated
        // Invalid states are impossible, and we don't have jank `"" str` returns, which means `rust-analyzer` and the language itself becomes more useful
        // We also get rid of quite a lot of the testing that we have to do
        let post = post.request_review();
        // These assertions also aren't needed because a draft in review *has no* `add_text` function
        // post.add_text("I ate a salad for lunch today");
        // assert_eq!("", post.content());

        let mut post = post.reject();
        post.add_text("I ate ice cream for dinner today.");

        let post = post.request_review();

        // This "works" but doesn't make much sense
        // If we wanted to do *any loop*, OOP + trait objects would probably make more sense
        let post = match post.approve() {
            ReviewResult::Post(p) => p,
            ReviewResult::Draft(e) => {
                match e.approve() {
                    ReviewResult::Post(p) => p,
                    ReviewResult::Draft(_) => unreachable!(),
                }
            },
        };
        assert_eq!(
            "I ate a salad for lunch today. I ate ice cream for dinner today.",
            post.content()
        );
    }
}
