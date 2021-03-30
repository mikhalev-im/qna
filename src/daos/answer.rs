use deadpool_postgres::{ Pool };
use tokio_postgres::{Row};
use std::time::{SystemTime};

pub struct AnswerEntity {
    pub id: i32,
    pub question_id: i32,
    pub content: String,
    pub created_at: SystemTime,
}

impl AnswerEntity {
    pub fn from_row(row: Row) -> AnswerEntity {
        AnswerEntity{
            id: row.get(0),
            question_id: row.get(1),
            content: row.get(2),
            created_at: row.get(3),
        }
    }
}

#[derive(Clone)]
pub struct AnswerDao {
    db_pool: Pool
}

impl AnswerDao {
    pub fn new(db_pool: Pool) -> AnswerDao {
        AnswerDao { db_pool }
    }

    pub async fn load_by_question_id(&self, question_id: i32) -> Vec<AnswerEntity> {
        let client = self.db_pool.get().await.unwrap();
        let stmt = client.prepare("
            SELECT a.id, a.question_id, a.content, a.created_at
            FROM answers a
        ").await.unwrap();
        let rows = client.query(&stmt, &[]).await.unwrap();
        let mut result = Vec::new();

        for row in rows {
            result.push(AnswerEntity::from_row(row));
        }

        result
    }
}