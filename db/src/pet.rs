pub const TABLE: &str = "pet";
const SELECT_FIELDS: &str = "id, owner_id, name, animal_type, color";

//fetches all pets belonging to a given owner_id
pub async fn fetch(db_pool: &DBPool, owner_id: i32) -> Result<Vec<Pet>> {
    let con = get_db_con(db_pool).await?;
    let query = format!("SELECT {} FROM {} WHERE owner_id = $1", SELECT_FIELDS, TABLE);
    let rows = con.query(query.as_str(), &[&owner_id]).await.map_err(DBQueryError)?;

    Ok(rows.iter().map(|r| row_to_pet(&r)).collect())
}

//creates a new pet for the given owner_id
pub async fn create(db_pool: &DBPool, owner_id: i32, body: PetRequest) -> Result<Pet> {
    let con = get_db_con(db_pool).await?;
    let query = format!(
        "INSERT INTO {} (name, owner_id, animal_type, color) VALUES ($1, $2, $3, $4) RETURNING *",
        TABLE
    );
    let row = con
        .query_one(
            query.as_str(),
            &[&body.name, &body.owner_id, &body.animal_type, &body.color],
        )
        .await
        .map_err(DBQueryError)?;
        Ok(row_to_pet(&row))
}

//deletes the pet with the given id and owner_id
pub async fn delete(db_pool: &DBPool, owner_id: i32, id: i32) -> Result<u64> {
    let con = get_db_con(db_pool).await?;
    let query = format!("DELETE FROM {} WHERE id = $1 AND owner_id = $2", TABLE);
    con.execute(query.as_str(), &[&id, &owner_id]).await.map_err(DBQueryError)
}

//constructs a Pet object from query 'row' result
fn row_to_pet(row: &Row) -> Pet {
    let id: i32 =  row.get(0);
    let owner_id: i32 = row.get(1);
    let name: String = row.get(2);
    let animal_type: String = row.get(3);
    let color: Option<String> = row.get(4);

    Pet {
        id,
        name,
        owner_id,
        animal_type,
        color,
    }
}



