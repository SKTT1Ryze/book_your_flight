//! 数据库后端实现

mod flight;
mod passenger;
mod time;
mod table;

use mysql::PooledConn;


/// Sql trait 统一抽象各种数据库数据
pub trait SqlEntry: Sized {
    /// 插入一行记录到数据库
    fn insert(&self, db: &mut PooledConn, tb_name: &str) -> mysql::Result<()>;
    /// 从查询的数据中构建结构体
    fn select(db: &mut PooledConn, tb_name: &str, id: usize) -> mysql::Result<Self>;
}