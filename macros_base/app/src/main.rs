use my_macros::JsonPlan;

pub trait JsonPlan {
    fn output_plan(&self);
}

#[derive(JsonPlan)]
struct UserProfile {
    #[rename("user_id")] // Our custom helper attribute
    id: u32,
    
    username: String,
    
    #[rename("web_url")]
    website: String,
}

fn main() {
    let profile = UserProfile {
        id: 1,
        username: "Rustacean".into(),
        website: "https://rust-lang.org".into(),
    };

    profile.output_plan();
}