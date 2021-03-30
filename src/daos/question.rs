use deadpool_postgres::{Pool};
use tokio_postgres::{Row};
use std::time::{SystemTime};

pub struct QuestionEntity {
    pub id: i32,
    pub rating: i32,
    pub title: String,
    pub content: String,
    pub created_at: SystemTime,
}

impl QuestionEntity {
    pub fn from_row(row: Row) -> QuestionEntity {
        QuestionEntity{
            id: row.get(0),
            rating: row.get(1),
            title: row.get(2),
            content: row.get(3),
            created_at: row.get(4),
        }
    }
}

#[derive(Clone)]
pub struct QuestionDao {
    db_pool: Pool
}

impl QuestionDao {
    pub fn new(db_pool: Pool) -> QuestionDao {
        QuestionDao { db_pool }
    }

    pub async fn load(&self) -> Vec<QuestionEntity> {
        let client = self.db_pool.get().await.unwrap();
        let stmt = client.prepare("
            SELECT q.id, q.rating, q.title, q.content, q.created_at
            FROM questions q
        ").await.unwrap();
        let rows = client.query(&stmt, &[]).await.unwrap();
        let mut result = Vec::new();

        for row in rows {
            result.push(QuestionEntity::from_row(row));
        }

        result
    }
}