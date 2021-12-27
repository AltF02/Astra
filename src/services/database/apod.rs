use crate::models::apod::Apod;
use crate::services::DB;
use chrono::NaiveDate;
use sqlx::postgres::PgQueryResult;

#[derive(Debug, sqlx::FromRow)]
pub struct DBApod {
    pub id: i32,
    pub publish_date: NaiveDate,
    pub explanation: String,
    pub title: String,
    pub hdurl: String,
    pub copyright: Option<String>,
    pub dispatched: bool,
}

impl Default for DBApod {
    fn default() -> Self {
        DBApod {
            id: 0,
            publish_date: NaiveDate::from_ymd(2003, 12, 31),
            explanation: "".to_string(),
            title: "".to_string(),
            hdurl: "".to_string(),
            copyright: None,
            dispatched: false,
        }
    }
}

impl From<&Apod> for DBApod {
    fn from(a: &Apod) -> Self {
        DBApod {
            id: 0,
            publish_date: NaiveDate::parse_from_str(a.date.as_str(), "%Y-%m-%d").unwrap(),
            explanation: a.explanation.clone(),
            title: a.title.clone(),
            hdurl: a.hdurl.clone(),
            copyright: a.copyright.clone(),
            dispatched: false,
        }
    }
}

impl DB {
    pub async fn get_most_recent_apod(&self) -> DBApod {
        unimplemented!()
    }

    pub async fn get_apod_dispatched(&self, a: &mut DBApod) {
        let res: DBApod = sqlx::query_as("SELECT * FROM astra.apod WHERE publish_date = $1;")
            .bind(a.publish_date)
            .fetch_optional(&self.pool)
            .await
            .unwrap()
            .unwrap_or_default();

        a.dispatched = res.dispatched;
    }

    pub async fn set_apod(&self, a: &DBApod) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query(
            "INSERT INTO astra.apod (publish_date, explanation, \
                 title, hdurl, copyright, dispatched) \
                 VALUES ($1, $2, $3, $4, $5, $6);",
        )
        .bind(&a.publish_date)
        .bind(&a.explanation)
        .bind(&a.title)
        .bind(&a.hdurl)
        .bind(&a.copyright)
        .bind(&a.dispatched)
        .execute(&self.pool)
        .await
    }
}
