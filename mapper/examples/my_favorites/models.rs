use types::ID;



use types::DateTime;


#[derive(Debug, Clone)]
pub struct MyFavorites {
    
    id: ID,
    user_id: i64,
    adventure_id: i64,
    is_deleted: i16,
    created_at: DateTime,
}

