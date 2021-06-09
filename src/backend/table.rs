use mysql::PooledConn;
use mysql::prelude::Queryable;
use crate::create_table;

/// 创建航班信息表
pub fn create_flights_table(db: &mut PooledConn, tb_name: &str) -> mysql::Result<()> {
    // let query = format!(
    //     r"create temporary table {} (
    //         id int not null,
    //         mtype char(20) not null,
    //         stime datetime not null,
    //         ftime int not null,
    //         capacity int not null,
    //         price int not null
    //     )
    //     ",
    //     tb_name
    // );
    // db.query_drop(query)?;
    create_table!(
        db,
        tb_name,
        r"create temporary table {} (
            id int not null,
            mtype char(20) not null,
            stime datetime not null,
            ftime int not null,
            capacity int not null,
            price int not null
        )
        "
    );
    Ok(())
}

/// 创建座位信息表
pub fn create_seats_table(db: &mut PooledConn, tb_name: &str) -> mysql::Result<()> {
    // let query = format!(
    //     r"create temporary table {} (
    //         id int not null,
    //         flight_id int not null,
    //         srow int not null,
    //         scolumn char(20) not null,
    //         is_booked bool
    //     )
    //     ",
    //     tb_name
    // );
    // db.query_drop(query)?;
    create_table!(
        db,
        tb_name,
        r"create temporary table {} (
            id int not null,
            flight_id int not null,
            srow int not null,
            scolumn char(20) not null,
            is_booked bool
        )
        "
    );
    Ok(())
}

/// 创建旅客信息表
pub fn create_passengers_table(db: &mut PooledConn, tb_name: &str) -> mysql::Result<()> {
    // let query = format!(
    //     r"create temporary table {} (
    //         id_card int not null,
    //         name char(20) not null
    //     )
    //     ",
    //     tb_name
    // );
    // db.query_drop(query)?;
    create_table!(
        db,
        tb_name,
        r"create temporary table {} (
            id_card int not null,
            name char(20) not null,
            password char(20) not null
        )
        "
    );
    Ok(())
}

/// 创建预定信息表
pub fn create_booked_records_table(db: &mut PooledConn, tb_name: &str) -> mysql::Result<()> {
    // let query = format!(
    //     r"create temporary table {} (
    //         id int not null,
    //         pid_card int not null,
    //         flight_id int not null,
    //         state int not null
    //     )
    //     ",
    //     tb_name
    // );
    // db.query_drop(query)?;
    create_table!(
        db,
        tb_name,
        r"create temporary table {} (
            id int not null,
            pid_card int not null,
            flight_id int not null,
            state int not null
        )
        "
    );
    Ok(())
}

#[macro_export]
macro_rules! create_table {
    ($db:expr, $tb_name:expr, $sql:expr) => {
        let query = format!($sql, $tb_name);
        $db.query_drop(query)?;
    };
}

#[test]
fn create_table_macro_test() -> mysql::Result<()> {
    use mysql::Pool;
    use super::time::Datetime;
    use super::flight::Flight;
    use super::SqlEntry;
    let url = "mysql://sktt1ryze:WXZFwxzf123@localhost/test_db";
    let pool = Pool::new(url)?;
    let mut conn = pool.get_conn()?;
    create_table!(
        conn,
        "macro_test_tb",
        r"
        create temporary table {} (
            id int not null,
            mtype char(20) not null,
            stime datetime not null,
            ftime int not null,
            capacity int not null,
            price int not null
        )
        "
    );
    let stime = Datetime::new(2021, 6, 3, 17, 40, 0, 0).as_sql();
    let flight = Flight {
        id: 0,
        mtype: "Boeding".to_string(),
        stime,
        ftime: 60,
        capacity: 100,
        price: 1000
    };
    flight.insert(&mut conn, "macro_test_tb")?;
    let new_flight = Flight::select(&mut conn, "macro_test_tb", 0)?;
    assert_eq!(flight.stime, new_flight.stime);
    Ok(())
}