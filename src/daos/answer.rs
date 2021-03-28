use deadpool_postgres::{ Pool };

struct AnswerEntity {}

#[derive(Clone)]
pub struct AnswerDao {
    db_pool: Pool
}

impl AnswerDao {
    pub fn new(db_pool: Pool) -> AnswerDao {
        AnswerDao { db_pool }
    }
}