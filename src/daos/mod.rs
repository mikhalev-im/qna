use deadpool_postgres::{ Config };
use tokio_postgres::NoTls;

mod answer;
mod question;

#[derive(Clone)]
pub struct Daos {
    pub questions: question::QuestionDao,
    pub answers: answer::AnswerDao,
}

pub fn new() -> Daos {
    let mut cfg = Config::new();
    cfg.dbname = Some("qna".to_string());
    cfg.user = Some("postgres".to_string());
    cfg.password = Some("postgres".to_string());

    let pool = cfg.create_pool(NoTls).unwrap();

    Daos {
        questions: question::QuestionDao::new(pool.clone()),
        answers: answer::AnswerDao::new(pool.clone()),
    }
}
