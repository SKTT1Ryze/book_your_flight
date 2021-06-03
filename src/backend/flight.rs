use mysql::{PooledConn, params, prelude::Queryable};
use super::SqlEntry;
/// 航班信息表项
#[derive(Debug)]
pub struct Flight {
    /// 航班编号
    id: usize,
    /// 机器型号
    mtype: String,
    /// 起飞时间
    stime: String,
    /// 预计飞行时间(单位为分钟)
    ftime: u32,
    /// 客容量
    capacity: u32,
    /// 价格
    price: u32
}

impl SqlEntry for Flight {
    fn insert(&self, db: &mut PooledConn, tb_name: &str) -> mysql::Result<()> {
        // 航班的座位信息存放在座位信息表中
        let stmt = format!(
            r#"insert into {} (id, mtype, stime, ftime, capacity, price)
            values  (:id, :mtype, :stime, :ftime, :capacity, :price)"#,
            tb_name
        );
        db.exec_drop(stmt, params! {
            "id" => self.id,
            "mtype" => self.mtype.as_str(),
            "stime" => self.stime.as_str(),
            "ftime" => self.ftime,
            "capacity" => self.capacity,
            "price" => self.price
        })?;
        Ok(())
    }

    fn select(db: &mut PooledConn, tb_name: &str, id: usize) -> mysql::Result<Self> {
        let query = format!(
            r#"select * from {}
            where id = {}
            "#,
            tb_name,
            id
        );
        let mut select_ret = db.query_map(
            query,
            |(id, mtype, stime, ftime, capacity, price)| {
                Self {
                    id,
                    mtype,
                    stime,
                    ftime,
                    capacity,
                    price
                }
            }
        )?;
        let ret = select_ret.pop().expect("[backend] select empty");
        Ok(ret)
    }
}

/// 座位信息
#[derive(PartialEq, Eq, Debug)]
pub struct SeatInfo {
    /// 座位编号
    id: usize,
    /// 所属航班编号
    flight_id: usize,
    /// 排号，座位号，比如 11A
    location: (usize, String),
    /// 是否已经被预定
    is_booked: bool
}


impl SqlEntry for SeatInfo {
    fn insert(&self, db: &mut PooledConn, tb_name: &str) -> mysql::Result<()> {
        let stmt = format!(
            r#"insert into {} (id, flight_id, srow, scolumn, is_booked)
            values  (:id, :flight_id, :srow, :scolumn, :is_booked)"#,
            tb_name
        );
        // println!("[backend] insert stmt: {}", stmt);
        db.exec_drop(stmt, params! {
            "id" => self.id,
            "flight_id" => self.flight_id,
            "srow" => self.location.0,
            "scolumn" => self.location.1.as_str(),
            "is_booked" => self.is_booked
        })?;
        Ok(())
    }

    fn select(db: &mut PooledConn, tb_name: &str, id: usize) -> mysql::Result<Self> {
        let query = format!(
            r#"select id, flight_id, srow, scolumn, is_booked from {}
            where id = {}
            "#,
            tb_name,
            id
        );
        // println!("[backend] select query: {}", query);
        let mut select_ret = db.query_map(
            query,
            |(id, flight_id, srow, scolumn, is_booked)| {
                Self {
                    id,
                    flight_id,
                    location: (srow, scolumn),
                    is_booked
                }
            }
        )?;
        let ret = select_ret.pop().expect("[backend] select empty");
        Ok(ret)
    }
}


#[test]
fn seatinfo_test() -> mysql::Result<()> {
    use mysql::Pool;
    let url = "mysql://sktt1ryze:WXZFwxzf123@localhost/test_db";
    let pool = Pool::new(url)?;
    let mut conn = pool.get_conn()?;
    conn.query_drop(
        r"
            create temporary table seats (
                id int not null,
                flight_id int not null,
                srow int not null,
                scolumn char(20) not null,
                is_booked bool
            )
        "
    )?;
    let seatinfo = SeatInfo {
        id: 0,
        flight_id: 0,
        location: (11, "A".to_string()),
        is_booked: false
    };
    seatinfo.insert(&mut conn, "seats")?;
    let new_seatinfo = SeatInfo::select(&mut conn, "seats", 0)?;
    assert_eq!(seatinfo, new_seatinfo);
    Ok(())
}

#[test]
fn flight_test() -> mysql::Result<()> {
    use mysql::Pool;
    use super::time::Datetime;
    let url = "mysql://sktt1ryze:WXZFwxzf123@localhost/test_db";
    let pool = Pool::new(url)?;
    let mut conn = pool.get_conn()?;
    conn.query_drop(
        r"
            create temporary table flights (
                id int not null,
                mtype char(20) not null,
                stime datetime not null,
                ftime int not null,
                capacity int not null,
                price int not null
            )
        "
    )?;
    let stime = Datetime::new(2021, 6, 3, 17, 40, 0, 0).as_sql();
    let flight = Flight {
        id: 0,
        mtype: "Boeding".to_string(),
        stime,
        ftime: 60,
        capacity: 100,
        price: 1000
    };
    flight.insert(&mut conn, "flights")?;
    let new_flight = Flight::select(&mut conn, "flights", 0)?;
    assert_eq!(flight.stime, new_flight.stime);
    Ok(())
}