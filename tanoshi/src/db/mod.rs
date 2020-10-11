use crate::catalogue::{Manga, Chapter, Page};
use anyhow::Result;
use sqlx::{Row, Done};
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use tokio::stream::StreamExt;

#[derive(Debug, Clone)]
pub struct Db {
    pool: SqlitePool,
}

impl Db {
    pub async fn establish_connection(database_path: String) -> Db {
        let pool = SqlitePoolOptions::new()
            .max_connections(25)
            .connect(&database_path)
            .await
            .unwrap();
        Db { pool }
    }

    pub async fn get_manga_by_id(&self, id: i64) -> Option<Manga> {
        let stream = sqlx::query(
            r#"SELECT * FROM manga WHERE id = ?"#
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .ok();

        if let Some(row) = stream {
            Some(Manga {
                id: row.get(0),
                source_id: row.get(1),
                title: row.get(2),
                author: row.get::<String, _>(3).split(",").map(|a| a.to_string()).collect(),
                genre: row.get::<String, _>(4).split(",").map(|a| a.to_string()).collect(),
                status: row.get(5),
                description: row.get(6),
                path: row.get(7),
                cover_url: row.get(8),
                last_read_chapter: row.get(9),
                is_favorite: row.get(10),
                date_added: row.get(11),
                
            })
        } else {
            None
        }
    }

    pub async fn get_manga_by_source_path(&self, source_id: i64, path: &String) -> Option<Manga> {
        let stream = sqlx::query(
            r#"SELECT * FROM manga WHERE source_id = ? AND path = ?"#
        )
        .bind(source_id)
        .bind(path)
        .fetch_one(&self.pool)
        .await
        .ok();

        if let Some(row) = stream {
            Some(Manga {
                id: row.get(0),
                source_id: row.get(1),
                title: row.get(2),
                author: row.get::<String, _>(3).split(",").map(|a| a.to_string()).collect(),
                genre: row.get::<String, _>(4).split(",").map(|a| a.to_string()).collect(),
                status: row.get(5),
                description: row.get(6),
                path: row.get(7),
                cover_url: row.get(8),
                last_read_chapter: row.get(9),
                is_favorite: row.get(10),
                date_added: row.get(11),
            })
        } else {
            None
        }
    }

    pub async fn get_mangas(&self, source_id: Vec<u64>, path: &String) -> Vec<Manga> {
        todo!()
    }

    pub async fn insert_manga(&self, manga: &Manga) -> Result<i64> {
        let row_id = sqlx::query(
            r#"INSERT INTO manga(
                source_id, 
                title, 
                author, 
                genre, 
                status, 
                description, 
                path, 
                cover_url, 
                date_added
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"#)
        .bind(manga.source_id)
        .bind(&manga.title)
        .bind(manga.author.join(","))
        .bind(manga.genre.join(","))
        .bind(&manga.status)
        .bind(&manga.description)
        .bind(&manga.path)
        .bind(&manga.cover_url)
        .bind(chrono::NaiveDateTime::from_timestamp(chrono::Local::now().timestamp(), 0))
        .execute(&self.pool)
        .await?
        .last_insert_rowid();

        Ok(row_id)
    }

    pub async fn insert_mangas(&self, manga: Vec<Manga>) -> Result<()> {
        todo!()
    }

    pub async fn update_manga_info(&self, manga: &Manga) -> Result<u64> {
        let rows_affected = sqlx::query(
            r#"UPDATE manga SET
                source_id = ?, 
                title = ?, 
                author = ?, 
                genre = ?, 
                status = ?, 
                description = ?, 
                path = ?, 
                cover_url = ?
                WHERE id = ?"#)
        .bind(manga.source_id)
        .bind(&manga.title)
        .bind(manga.author.join(","))
        .bind(manga.genre.join(","))
        .bind(&manga.status)
        .bind(&manga.description)
        .bind(&manga.path)
        .bind(&manga.cover_url)
        .bind(manga.id)
        .execute(&self.pool)
        .await?
        .rows_affected();

        Ok(rows_affected)
    }

    pub async fn get_chapter_by_id(&self, id: i64) -> Option<Chapter> {
        let stream = sqlx::query(
            r#"
            SELECT *, 
            (SELECT c.id FROM chapter c WHERE c.manga_id = 21 AND c.rank = chapter.rank - 1) prev,
            (SELECT c.id FROM chapter c WHERE c.manga_id = 21 AND c.rank = chapter.rank + 1) next 
            FROM chapter WHERE id = ?"#
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .ok();

        if let Some(row) = stream {
            Some(Chapter {
                id: row.get(0),
                source_id: row.get(1),
                manga_id: row.get(2),
                title: row.get(3),
                path: row.get(4),
                rank: row.get(5),
                read_at: row.get(6),
                uploaded: row.get(7),
                date_added: row.get(8),
                prev: row.get(9),
                next: row.get(10),
            })
        } else {
            None
        }
    }

    pub async fn get_chapter_by_source_path(&self, source_id: i64, path: &String) -> Option<Chapter> {
        let stream = sqlx::query(
            r#"
            SELECT *,
            (SELECT c.id FROM chapter c WHERE c.manga_id = 21 AND c.rank = chapter.rank - 1) prev,
            (SELECT c.id FROM chapter c WHERE c.manga_id = 21 AND c.rank = chapter.rank + 1) next 
            FROM chapter WHERE source_id = ? AND path = ?"#
        )
        .bind(source_id)
        .bind(path)
        .fetch_one(&self.pool)
        .await
        .ok();

        if let Some(row) = stream {
            Some(Chapter {
                id: row.get(0),
                source_id: row.get(1),
                manga_id: row.get(2),
                title: row.get(3),
                path: row.get(4),
                rank: row.get(5),
                read_at: row.get(6),
                uploaded: row.get(7),
                date_added: row.get(8),
                prev: row.get(9),
                next: row.get(10),
            })
        } else {
            None
        }
    }
    
    pub async fn get_chapters_by_manga_id(&self, manga_id: i64) -> Result<Vec<Chapter>> {
        let mut stream = sqlx::query(
            r#"
            SELECT *,
            (SELECT c.id FROM chapter c WHERE c.manga_id = chapter.manga_id AND c.rank = chapter.rank - 1) prev,
            (SELECT c.id FROM chapter c WHERE c.manga_id = chapter.manga_id AND c.rank = chapter.rank + 1) next 
            FROM chapter WHERE manga_id = ?"#
        )
        .bind(manga_id)
        .fetch(&self.pool);

        let mut chapters = vec![];
        while let Some(row) = stream.try_next().await? {
            chapters.push(Chapter {
                id: row.get(0),
                source_id: row.get(1),
                manga_id: row.get(2),
                title: row.get(3),
                path: row.get(4),
                rank: row.get(5),
                read_at: row.get(6),
                uploaded: row.get(7),
                date_added: row.get(8),
                prev: row.get(9),
                next: row.get(10),
            });
        }
        if chapters.len() == 0 {
            Err(anyhow::anyhow!("Chapters not found"))
        } else {
            Ok(chapters)
        }
    }

    pub async fn insert_chapter(&self, chapter: &Chapter) -> Result<i64> {
        let row_id = sqlx::query(
            r#"INSERT INTO chapter(
                source_id,
                manga_id,
                title, 
                path, 
                rank, 
                uploaded, 
                date_added
            ) VALUES (?, ?, ?, ?, ?, ?, ?)"#)
        .bind(chapter.source_id)
        .bind(chapter.manga_id)
        .bind(&chapter.title)
        .bind(&chapter.path)
        .bind(chapter.rank)
        .bind(chapter.uploaded)
        .bind(chrono::NaiveDateTime::from_timestamp(chrono::Local::now().timestamp(), 0))
        .execute(&self.pool)
        .await?
        .last_insert_rowid();

        Ok(row_id)
    }

    pub async fn get_page_by_source_url(&self, source_id: i64, url: &String) -> Option<Page> {
        let stream = sqlx::query(
            r#"SELECT * FROM page WHERE source_id = ? AND url = ?"#
        )
        .bind(source_id)
        .bind(url)
        .fetch_one(&self.pool)
        .await
        .ok();

        if let Some(row) = stream {
            Some(Page{
                id: row.get(0),
                source_id: row.get(1),
                manga_id: row.get(2),
                chapter_id: row.get(3),
                rank: row.get(4),
                url: row.get(5),
                read_at: row.get(6),
                date_added: row.get(7),
            })
        } else {
            None
        }
    }

    pub async fn insert_page(&self, page: &Page) -> Result<i64> {
        let row_id = sqlx::query(
            r#"INSERT INTO page(
                source_id,
                manga_id,
                chapter_id,
                rank, 
                url,
                date_added
            ) VALUES (?, ?, ?, ?, ?, ?)"#)
        .bind(page.source_id)
        .bind(page.manga_id)
        .bind(page.chapter_id)
        .bind(page.rank)
        .bind(&page.url)
        .bind(chrono::NaiveDateTime::from_timestamp(chrono::Local::now().timestamp(), 0))
        .execute(&self.pool)
        .await?
        .last_insert_rowid();

        Ok(row_id)
    }

    pub async fn get_pages_by_chapter_id(&self, chapter_id: i64) -> Result<Vec<Page>> {
        let mut stream = sqlx::query(
            r#"SELECT * FROM page WHERE chapter_id = ?"#
        )
        .bind(chapter_id)
        .fetch(&self.pool);

        let mut pages = vec![];
        while let Some(row) = stream.try_next().await? {
            pages.push(Page{
                id: row.get(0),
                source_id: row.get(1),
                manga_id: row.get(2),
                chapter_id: row.get(3),
                rank: row.get(4),
                url: row.get(5),
                read_at: row.get(6),
                date_added: row.get(7),
            });
        }
        if pages.len() == 0 {
            Err(anyhow::anyhow!("Pages not found"))
        } else {
            Ok(pages)
        }
    }
}
