use oracle::{Connector, Row}; // Row를 추가로 import 하세요.

mod queries;

fn main() -> Result<(), oracle::Error> {
    let conn = Connector::new("docker", "docker123", "127.0.0.1:1521/ORCL")
        .connect()?;

    // 1. INSERT (Create)
    println!("--- INSERT ---");
    let id = 101;
    let title = "Rust Oracle Test";
    let content = "This is a test content inserted by Rust.";

    conn.execute(queries::INSERT_BOARD, &[&id, &title, &content])?;
    conn.commit()?; // 커밋 필수
    println!("Inserted ID: {}", id);

    // 2. SELECT (Read)
    println!("\n--- SELECT ---");
    let sql = queries::SELECT_BOARD;
    let rows = conn.query(sql, &[])?;

    for row_result in rows {
        let row: Row = row_result?;
        let r_id: i64 = row.get("ID")?;
        let r_title: Option<String> = row.get("TITLE")?;
        let r_content: Option<String> = row.get("CONTENT")?;
        let r_created_at: Option<oracle::sql_type::Timestamp> = row.get("CREATED_AT")?;

        println!("ID: {}, Title: {:?}, Content: {:?}, Date: {:?}", r_id, r_title, r_content, r_created_at);
    }

    // 3. UPDATE
    println!("\n--- UPDATE ---");
    let new_title = "Updated Title";
    let new_content = "Updated Content";
    conn.execute(queries::UPDATE_BOARD, &[&new_title, &new_content, &id])?;
    conn.commit()?;
    println!("Updated ID: {}", id);

    // 4. DELETE
    println!("\n--- DELETE ---");
    conn.execute(queries::DELETE_BOARD, &[&id])?;
    conn.commit()?;
    println!("Deleted ID: {}", id);

    Ok(())
}