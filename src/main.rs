use async_recursion::async_recursion;

struct Comment {
    id: u64,
    children: Vec<Comment>,
}

impl Comment {
    #[async_recursion(?Send)]
    async fn walk(&self) {
        let content = get_content(self.id).await;
        println!("{}", content);

        for child in &self.children {
            child.walk().await;
        }
    }
}

async fn get_content(comment_id: u64) -> String {
    format!("Content of {}", comment_id)
}

struct Comments(Vec<Comment>);

impl Comments {
    async fn walk(&self) {
        for comment in &self.0 {
            comment.walk().await
        }
    }
}

#[async_std::main]
async fn main() {
    let comments = Comments(vec![
        Comment { id: 0, children: vec![Comment { id: 1, children: vec![] }, Comment { id: 2, children: vec![] }] },
        Comment { id: 3, children: vec![Comment { id: 4, children: vec![Comment { id: 5, children: vec![] }] }] },
    ]);
    
    comments.walk().await;
}
