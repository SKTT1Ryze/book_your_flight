/// MySql 中的 datetime
/// 年，月，日，时，分，秒，微秒
pub struct Datetime(u16, u8, u8, u8, u8, u8, u32);

impl Datetime {
    pub fn new(
        year: u16,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
        microsecond: u32
    ) -> Self {
        Self(year, month, day, hour, minute, second, microsecond)
    }
    pub fn as_sql(&self) -> String {
        match *self {
            // Datetime(y, m, d, 0, 0, 0, 0) => format!("{:04}-{:02}-{:02}", y, m, d),
            // Datetime(year, month, day, hour, minute, second, 0) => format!(
            //     "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
            //     year, month, day, hour, minute, second
            // ),
            Datetime(year, month, day, hour, minute, second, micros) => format!(
                "{:04}-{:02}-{:02} {:02}:{:02}:{:02}.{:06}",
                year, month, day, hour, minute, second, micros
            ),
            
        }
    }
    pub fn from_sql(sql: &str) -> Self {
        todo!()
    }
}