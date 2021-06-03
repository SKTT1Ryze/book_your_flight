use mysql::prelude::*;
use mysql::*;

#[derive(Debug, PartialEq, Eq)]
struct Entry {
    pub id: i32,
    pub foo: i32,
    pub bar: Option<String>
}

fn main() -> mysql::Result<()> {
    println!("book your flights!");
    println!("Hello, mysql!");
    let url = "mysql://root:SKTT1Faker668@localhost/test_db";
    let pool = Pool::new(url)?;
    let mut conn = pool.get_conn()?;
    conn.query_drop(
        r"
            create temporary table rust_tb (
                id int not null,
                foo int not null,
                bar char(20)
            )
        "
    )?;

    let entrys = vec![
        Entry { id: 0, foo: 0, bar: Some("abc".into())},
        Entry { id: 1, foo: 1, bar: Some("def".into())},
        Entry { id: 2, foo: 2, bar: Some("ghi".into())},
    ];
    conn.exec_batch(
        r"
            insert into rust_tb (id, foo, bar)
            values (:id, :foo, :bar)
        ",
        entrys.iter().map(|e| params! {
            "id" => e.id,
            "foo" => e.foo,
            "bar" => &e.bar
        })
    )?;

    let select_entry = conn
        .query_map(
            "select id, foo, bar from rust_tb",
            |(id, foo, bar)| {
                Entry {id, foo, bar}
            }
        )?;
    
    println!("select ret: {:?}", select_entry);
    Ok(())
}
