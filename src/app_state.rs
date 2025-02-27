#[derive(Clone, Debug)]
pub struct AppState{
    user_id: u64
}

// Constructor
impl AppState{
    pub fn new(user_id: u64) -> Self {
        Self { user_id }
    }
}

// Property Accessors
impl AppState {
    pub fn user_id(&self) -> u64 {
        self.user_id
    }
}