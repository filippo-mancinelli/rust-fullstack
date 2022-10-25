pub const TABLE: &str = "pet";
const SELECT_FIELDS: &str = "id, owner_id, name, animal_type, color";

pub async fn fetch(db_pool: &DBPool, owner_id: i32) -> Result<Vec<Pet>> {
    let con = get_db_con(db_pool).await?;
    let query = format!("SELECT {} FROM {} WHERE owner_id = $1", SELECT_FIELDS, TABLE);
    let rows = con.query(query.as_str(), &[&owner_id]).await.map_err(DBQueryError)?;

    Ok(rows.iter().map(|r| row_to_pet(&r)).collect())
}

pub async fn create(db_pool: &DBPool, owner_id: i32, body: PetRequest) -> Result<Pet> {
    let con = get_db_con(db_pool).await?;
    let query = format!(
        "INSERT INTO {} (name, owner_id, animal_type, color) VALUES ($1, $2, $3, $4) RETURNING *",
        TABLE
    );
    let row = con
        .query_one(
            query.as_str(),
            &[&body]
        )
}



