use rusqlite::{Connection, Result};

pub fn init_budget(category_limits_huf: crate::read_config::CategoryLimitsHuf) -> Result<()> {
    let conn = Connection::open("./finance.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS monthly_budget (
            is_valid INTEGER,
            rent INTEGER, 
            overhead INTEGER,
            groceries INTEGER,
            health INTEGER,
            sport INTEGER,
            household INTEGER,
            car INTEGER,
            clothes INTEGER,
            restaurant INTEGER,
            courses INTEGER,
            travel INTEGER,
            pension INTEGER,
            rest INTEGER
        )",
        [],
    )?;
    let mut query = conn.prepare("SELECT count(*) FROM monthly_budget")?;
    let rows = query.query_map([], |row| row.get(0))?;
    let mut count = 0;
    for row in rows {
        count = row?;
    }
    if count == 0 {
        conn.execute(
            "INSERT INTO monthly_budget (
                is_valid,
                rent,
                overhead,
                groceries,
                health,
                sport,
                household,
                car,
                clothes,
                restaurant,
                courses,
                travel,
                pension,
                rest
            )
                VALUES (
                    1,
                    ?2,
                    ?3,
                    ?4,
                    ?5,
                    ?6,
                    ?7,
                    ?8,
                    ?9,
                    ?10,
                    ?11,
                    ?12,
                    ?13
                )",
            [
                &category_limits_huf.rent,
                &category_limits_huf.overhead,
                &category_limits_huf.groceries,
                &category_limits_huf.health,
                &category_limits_huf.sport,
                &category_limits_huf.household,
                &category_limits_huf.car,
                &category_limits_huf.clothes,
                &category_limits_huf.restaurant,
                &category_limits_huf.courses,
                &category_limits_huf.travel,
                &category_limits_huf.pension,
                &category_limits_huf.rest,
            ],
        )?;
    };
    let mut stmt = conn.prepare("SELECT * FROM monthly_budget;")?;
    let budget_iter = stmt.query_map([], |row| {
        Ok(crate::read_config::CategoryLimitsHuf {
            rent: row.get(1)?,
            overhead: row.get(2)?,
            groceries: row.get(3)?,
            health: row.get(4)?,
            sport: row.get(5)?,
            household: row.get(6)?,
            car: row.get(7)?,
            clothes: row.get(8)?,
            restaurant: row.get(9)?,
            courses: row.get(10)?,
            travel: row.get(11)?,
            pension: row.get(12)?,
            rest: row.get(13)?,
        })
    })?;

    for budget in budget_iter {
        println!("Found budget {:#?}", budget);
    }

    Ok(())
}
