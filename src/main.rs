use rusqlite::{Connection, Result};

#[derive(Debug)]
struct MonthlyBudget {
    is_valid: i32,
    rent: i32,
    overhead: i32,
    groceries: i32,
    health: i32,
    sport: i32,
    household: i32,
    car: i32,
    clothes: i32,
    restaurant: i32,
    courses: i32,
    travel: i32,
    pension: i32,
    rest: i32,
}

fn main() -> Result<()> {
    let mut conn = Connection::open("./finance.db")?;
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
                    170000,
                    50000,
                    80000,
                    25000,
                    30000,
                    15000,
                    40000,
                    20000,
                    60000,
                    5000,
                    50000,
                    25000,
                    30000
                )",
            [],
        )?;
    };
    let mut stmt = conn.prepare("SELECT * FROM monthly_budget;")?;
    let budget_iter = stmt.query_map([], |row| {
        Ok(MonthlyBudget {
            is_valid: row.get(0)?,
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
        println!("Found budget {:?}", budget);
    }

    Ok(())
}
