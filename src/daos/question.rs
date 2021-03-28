use deadpool_postgres::{ Pool };

struct QuestionEntity {}

#[derive(Clone)]
pub struct QuestionDao {
    db_pool: Pool
}

impl QuestionDao {
    pub fn new(db_pool: Pool) -> QuestionDao {
        QuestionDao { db_pool }
    }

    pub fn load(&self) {
        println!("WORKING!");
    }
}