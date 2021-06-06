use mysql::{PooledConn, params, prelude::Queryable};
use super::SqlEntry;

/// 旅客信息表项
#[derive(Debug, PartialEq, Eq)]
pub struct Passenger {
    /// 身份证
    id_card: usize,
    /// 姓名
    name: String,
    /// 密码
    password: String,
}

impl SqlEntry for Passenger {
    fn insert(&self, db: &mut PooledConn, tb_name: &str) -> mysql::Result<()> {
        let stmt = format!(
            r#"insert into {} (id_card, name, password)
            values (:id_card, :name, :password)"#,
            tb_name
        );
        db.exec_drop(stmt, params! {
            "id_card" => self.id_card,
            "name" => self.name.as_str(),
            "password" => self.password.as_str()
        })?;
        Ok(())
    }

    fn select(db: &mut PooledConn, tb_name: &str, id: usize) -> mysql::Result<Self> {
        let query = format!(
            r#"select * from {}
            where id_card = {}"#,
            tb_name,
            id
        );
        let mut select_ret = db.query_map(
            query,
            |(id_card, name, password)| {
                Self {
                    id_card,
                    name,
                    password
                }
            }
        )?;
        let ret = select_ret.pop().expect("[backend] select empty");
        Ok(ret)
    }
}

/// 航班预定信息
#[derive(PartialEq, Eq, Debug)]
pub struct BookedRecord {
    /// 记录编号
    id: usize,
    /// 旅客身份证
    pid_card: usize,
    /// 航班编号
    flight_id: usize,
    /// 预定状态
    state: BookdedState
}

impl SqlEntry for BookedRecord {
    fn insert(&self, db: &mut PooledConn, tb_name: &str) -> mysql::Result<()> {
        let stmt = format!(
            r#"insert into {} (id, pid_card, flight_id, state)
            values (:id, :pid_card, :flight_id, :state)"#,
            tb_name
        );
        db.exec_drop(stmt, params! {
            "id" => self.id,
            "pid_card" => self.pid_card,
            "flight_id" => self.flight_id,
            "state" => self.state as usize
        })?;
        Ok(())
    }

    fn select(db: &mut PooledConn, tb_name: &str, id: usize) -> mysql::Result<Self> {
        let query = format!(
            r#"select * from {}
            where id = {}"#,
            tb_name,
            id
        );
        let mut select_ret = db.query_map(
            query,
            |(id, pid_card, flight_id, state)| {
                let state = match state {
                    0 => BookdedState::NotPaied,
                    1 => BookdedState::PaiedNotFinished,
                    2 => BookdedState::Finished,
                    _ => panic!("unknown state value!")
                };
                Self {
                    id,
                    pid_card,
                    flight_id,
                    state
                }
            }
        )?;
        let ret = select_ret.pop().expect("[backend] select empty");
        Ok(ret)
    }
}

/// 航班预定状态有三种
/// 一个是已经预定了但没有付款
/// 二是付了款但是订单没完成（没到目的地）
/// 三是已经完成（旅客已经顺利到达目的地）
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BookdedState {
    NotPaied = 0,
    PaiedNotFinished = 1,
    Finished = 2
}

#[test]
fn booked_record_test() -> mysql::Result<()> {
    use mysql::Pool;
    let url = "mysql://sktt1ryze:WXZFwxzf123@localhost/test_db";
    let pool = Pool::new(url)?;
    let mut conn = pool.get_conn()?;
    conn.query_drop(
        r"
            create temporary table booked_records (
                id int not null,
                pid_card int not null,
                flight_id int not null,
                state int not null
            )
        "
    )?;
    let booked_record = BookedRecord {
        id: 0,
        pid_card: 1,
        flight_id: 2,
        state: BookdedState::NotPaied
    };
    booked_record.insert(&mut conn, "booked_records")?;
    let new_booked_record = BookedRecord::select(&mut conn, "booked_records", 0)?;
    assert_eq!(booked_record, new_booked_record);
    Ok(())
}

#[test]
fn passenger_test() -> mysql::Result<()> {
    use mysql::Pool;
    let url = "mysql://sktt1ryze:WXZFwxzf123@localhost/test_db";
    let pool = Pool::new(url)?;
    let mut conn = pool.get_conn()?;
    conn.query_drop(
        r"
            create temporary table passengers (
                id_card int not null,
                name char(20) not null,
                password char(20) not null
            )
        "
    )?;
    let passenger = Passenger {
        id_card: 0,
        name: "ccc".to_string(),
        password: "testpassword".to_string()
    };
    passenger.insert(&mut conn, "passengers")?;
    let new_passenger = Passenger::select(&mut conn, "passengers", 0)?;
    assert_eq!(passenger, new_passenger);
    Ok(())
}