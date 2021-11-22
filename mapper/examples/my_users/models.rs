use types::ID;




use types::DateTime;


#[derive(Debug, Clone)]
pub struct MyUsers {
    
    id: ID,
    username: String,
    password: String,
    roles: Vec<String>,
    is_deleted: i16,
    created_at: DateTime,
}

