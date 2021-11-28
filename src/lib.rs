pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }
    pub fn add_text(&mut self, text: &str) {
        // This line works, because I guess if you're taking the mutable child reference of a mutable parent reference
        // then that's still technically "1 mutable reference"
        self.state
            .as_mut()
            .unwrap()
            .add_text(&mut self.content, text)
    }
    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }
    pub fn request_review(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.request_review());
        }
    }
    pub fn approve(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.approve());
        }
    }
}

impl Default for Post {
    fn default() -> Self {
        Self::new()
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    /// Basically, realize that `&mut self` is synctactic sugar for `self: &mut Self`
    /// So `self: Box<Self>` basically specifies that self exists inside a box, and
    /// therefore takes ownership of both the box and the things inside it?
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn reject(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, _: &'a Post) -> &'a str {
        ""
    }
    fn add_text(&mut self, _: &mut String, _: &str) {}
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview::new())
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn add_text(&mut self, content: &mut String, text: &str) {
        content.push_str(text)
    }
}

struct PendingReview {
    approve_count: u32,
}

impl PendingReview {
    pub fn new() -> Self {
        Self { approve_count: 0 }
    }
}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(mut self: Box<Self>) -> Box<dyn State> {
        self.approve_count += 1;
        if self.approve_count >= 2 {
            Box::new(Published {})
        } else {
            self
        }
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
}
