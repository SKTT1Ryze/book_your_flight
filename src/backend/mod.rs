//! 数据库后端实现
//! 
//! + 每个航班信息的输入（插入）
//! + 每个航班的座位信息的输入（插入）
//! + 当旅客进行机票预定时，输入旅客基本信息，系统为旅客安排航班，打印取票通知和账单（查询）
//! + 旅客在飞机起飞前一天凭取票通知交款取票（查询，修改，删除）
//! + 旅客能够退订机票
//! + 能够查询每个航班的预定情况，计算航班的满座率
//! + 包含事务，存储过程/触发器，视图，函数
//! + 在程序中需要体现SQL和编程语言的结合
//! + 包含至少以下数据表
//!     - 航班信息表
//!     - 航班座位情况表
//!     - 旅客订票信息表
//!     - 取票通知表
//!     - 账单
//! 
//! 需要的表格:
//! + 记录航班信息的表格(id, plane, departure_time, flight_time, capacity, price)
//! + 记录航班座位信息的表格(flight_id, loc_row, loc_column, is_booked)
//! + 旅客信息表(id_card, name)
//! + 预定信息表(id, pid_card, flight_id, state)
//! + 记录航班状态的表格(flight_id, state)
//! + 记录已完成状态航班信息的表格(flight_id, carried_num(实际载客量))




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