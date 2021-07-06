# 华中科技大学计算机学院数据库系统实验

## 课程任务概述
本实验课程建立在数据库系统原理理论课的基础上，从浅到深建立了若干个实践任务，旨在通过实践过程让我们体会数据库系统给开发环境带来的便利和功能，同时巩固在课堂上接收的理论知识。  
实验的第一部分是软件功能学习。  
实验的第二部分是 SQL 练习，该部分要求我们在 educoder 平台上编写一系列 SQL 语句，并通过脚本测试。  
实验的第三部分是运用所学到的数据库理论知识和 SQL 语句基础，设计并且实现出一个完整的数据库应用系统，笔者这里选择的是机票预定系统。   

## 软件功能学习部分
### 任务要求
* 练习 sql server 或者其他某个主流关系数据库管理系统软件的备份方式：数据文件和日志文件的脱机备份，系统的备份功能。
* 练习在新增的数据库上增加用户并配置权限的操作，通过用创建的用户登录数据库并且执行未经授权的 sql 语句验证自己的权限配置是否成功。


### 完成过程
### 任务总结

## SQL 练习
### 任务要求
创建用于管理新冠疫情防控的数据库，并创建一系列相应的表格，进行增删改查等操作。  
### 完成过程
#### 创建数据库和表
建表如下 sql 语句如下：  
```sql
create database if not exists covid19mon;
use covid19mon;

-- 请在以下适当的空白位置填写SQL语句完成任务书的要求。空白位置不够的话，可以通过回车换行增加。
-- 表1 人员表(person)
    create table if not exists person (
        id int not null,
        fullname char(20) not null,
        telephone char(11) not null,
        constraint pk_persion primary key (id)
    );
-- 表2 地点表(location)
    create table if not exists location (
        id int not null,
        location_name char(20) not null,
        constraint pk_location primary key (id)
    );
-- 表3 行程表（itinerary）
    create table if not exists itinerary (
        id int not null,
        p_id int,
        loc_id int,
        s_time datetime,
        e_time datetime,
        constraint pk_itinerary primary key (id),
	    constraint fk_itinerary_pid foreign key (p_id) references person(id),
	    constraint fk_itinerary_lid foreign key (loc_id) references location(id)
    );
-- 表4 诊断表（diagnose_record）
    create table if not exists diagnose_record (
        id int,
        p_id int,
        diagnose_date datetime,
        result int,
        CONSTRAINT pk_diagnose_record PRIMARY KEY (id),
        CONSTRAINT fk_diagnose_pid FOREIGN KEY (p_id) REFERENCES person(id)
    );
 
-- 表5 密切接触者表（close_contact）
    create table if not exists close_contact (
        id int,
        p_id int,
        contact_date datetime,
        loc_id int,
        case_p_id int,
        CONSTRAINT pk_close_contact PRIMARY KEY (id),
        CONSTRAINT fk_contact_pid FOREIGN KEY (p_id) REFERENCES person(id),
        CONSTRAINT fk_contact_lid FOREIGN KEY (loc_id) REFERENCES location(id),
        CONSTRAINT fk_contact_caseid FOREIGN KEY (case_p_id) REFERENCES person(id)
    );
-- 表6 隔离地点表（isolation_location）
    create table if not exists isolation_location (
        id int,
        location_name char(20),
        capacity int,
        CONSTRAINT pk_isolation_loc PRIMARY KEY (id)
    );
-- 表7 隔离表（isolation_record）
    create table if not exists isolation_record (
        id int,
        p_id int,
        s_date datetime,
        e_date datetime,
        isol_loc_id int,
        state int,
        CONSTRAINT pk_isolation PRIMARY KEY (id),
        CONSTRAINT fk_isolation_pid FOREIGN KEY (p_id) REFERENCES person(id),
        CONSTRAINT fk_isolation_lid FOREIGN KEY (isol_loc_id) REFERENCES isolation_location(id)
    );
-- 代码结束
/* *********************************************************** */
```
结果如下:  
![create_table](create_table.png)  

#### 表的更新
数据插入：  
```sql
-- 向人员表person插入以下3条数据:
-- ( 1, 张小敏,13907110001)
-- ( 2, 李大锤,18907110002)
-- ( 3, 孙二娘,13307100003)
-- 请在以下空白处写下完成任务要求的SQL语句：
use covid19mon;

insert into person (id, fullname, telephone) values (1, "张小敏", "13907110001");
insert into person (id, fullname, telephone) values (2, "李大锤", "18907110002");
insert into person (id, fullname, telephone) values (3, "孙二娘", "13307100003");
/* end of you code */
```

数据删除：  
```sql
-- 从人员表person中删除以下数据:
-- ( 2, 李大锤,18907110002)
-- 请在以下空白处写下完成任务要求的SQL语句：
use  covid19mon;

delete from person where id=2;
/* the end of your code */ 
```
修改数据：  
```sql
 -- 请将人员表person中1号人员的电话更改成13607176668
 -- 请在空白处写下能完成目标任务的SQL语句：
use covid19mon;
update person set telephone="13607176668" where id=1;

/* the end of your code */
```

结果：todo  

#### 数据查询
人流量大于 30 的地点：  
```sql
use covid19mon;

create table temp_tb (select loc_id, count(loc_id) as visitors from itinerary group by loc_id having count(loc_id) > 30);
-- select * from temp_tb;
select location.location_name, temp_tb.visitors from location, temp_tb where location.id = temp_tb.loc_id order by temp_tb.visitors desc, location.location_name asc;
drop table temp_tb;


/*  end  of  your code  */
 
```

每个隔离点正在进行隔离的人数：  
```sql
 use covid19mon;

-- 2) 查询每个隔离地及该地正在进行隔离的人数，以number为隔离人数的标题.
--    查询结果依隔离人数由多到少排序。人数相同时，依隔离地点名排序。
--    请用一条SQL语句实现该查询：

select
    isolation_location.location_name, count(isolation_record.p_id) as `number`
from isolation_location, isolation_record
where
    isolation_record.isol_loc_id = isolation_location.id and
    isolation_record.state = 1
group by isolation_record.isol_loc_id
order by count(isolation_record.p_id) desc, isolation_location.location_name asc;

/*  end  of  your code  */
```

连续行程：  
```sql
 use covid19mon;

-- 2) 查询行程表中所有属于同一个人的接续行程信息。输出内容包括：
-- 人员编号,姓名,重合时间,起始地点id,起始地点,结束地点id,结束地点。
-- 查询结果依人员编号排序。
-- 请用一条SQL语句实现该查询：

select person.id, person.fullname, person.telephone, I1.s_time as reclosing_time,
    L2.id as loc1, L2.location_name as address1, L1.id as loc2, L1.location_name as address2
from itinerary I1, itinerary I2, person, location L1, location L2
where I1.p_id = I2.p_id and I1.s_time = I2.e_time and I1.p_id = person.id and person.id > 30
    and I1.loc_id = L1.id and I2.loc_id = L2.id
    order by person.id, reclosing_time ASC;

/*  end  of  your code  */
```

充珉瑶和贾涵山的行程情况：  
```sql
-- 4) 查询充珉瑶和贾涵山的行程情况。查询结果包括：姓名、电话、到过什么地方（地名），何时到达，何时离开 。
--  列名原样列出，不必用别名。查询结果依人员编号降序排序。

--    请用一条SQL语句实现该查询：
use covid19mon;

select person.fullname, person.telephone, location.location_name, itinerary.s_time, itinerary.e_time
from itinerary
    right join location on itinerary.loc_id = location.id
    right join person on itinerary.p_id = person.id
where
    (person.fullname = '充珉瑶' or person.fullname = '贾涵山')
order by person.id desc, itinerary.s_time;

/*  end  of  your code  */
```

地名中带有‘店’字的地点名称:  
```sql
-- 5) 查询地名中带有‘店’字的地点编号和名称。查询结果按地点编号排序。
--    请用一条SQL语句实现该查询：
use covid19mon;

select location.id, location.location_name
from location
where
    location.location_name like '%店%';

/*  end  of  your code  */
```

确诊者的接触者：  
```sql
-- 6) 新发现一位确诊者，已知他在2021.2.2日20:05:40到21:25:40之间在“活动中心”逗留，
--    凡在此间在同一地点逗留过的，视为接触者，请查询接触者的姓名和电话。查询结果按姓名排序.
--    请用一条SQL语句实现该查询：
use covid19mon;

select person.fullname, person.telephone
from person, itinerary, location
where
    person.id = itinerary.p_id and
    location.id = itinerary.loc_id and
    location.location_name = "活动中心" and
    (
        (itinerary.s_time <= "2021-2-2 20:05:40" and itinerary.e_time >= "2021-2-2 20:05:40") or
        (itinerary.s_time >= "2021-2-2 20:05:40" and itinerary.s_time <= "2021-2-2 21:25:40")
    )
order by person.fullname

/*  end  of  your code  */
```

仍在使用的隔离点:  
```sql
-- 7) 查询正在使用的隔离点名,查询结果按隔离点的编号排序。
--    请用一条SQL语句实现该查询：
use covid19mon;

select isolation_location.location_name
from isolation_location
where
    isolation_location.id in (
        select distinct isolation_location.id
        from isolation_location, isolation_record
        where
            isolation_location.id = isolation_record.isol_loc_id and
            isolation_record.state = 1
    )
order by isolation_location.id

/*  end  of  your code  */
```

查询有出行记录的人员:  
```sql
-- 8) 用一条带exists关键字的SQL语句查询前30位有出行记录的人员姓名和电话。查询结果按照人员编号排序。
--    请用一条SQL语句实现该查询：
use covid19mon;

select person.fullname, person.telephone
from person
where exists (
    select * from itinerary
    where person.id = itinerary.p_id
)
order by person.id
limit 30

/*  end  of  your code  */
```

没有去过"Today便利店"的人数:  
```sql
-- 9) 写一条带NOT EXISTS 子查询的SQL语句实现下述查询要求：
--   查询人员表中没有去过地点“Today便利店”的人数。请给统计出的人数命名为number。
--   请用一条SQL语句实现该查询：
use covid19mon;

select count(*) as number
from person
where not exists (
    select * from location, itinerary
    where
        location.id = itinerary.loc_id and
        location.location_name = "Today便利店" and
        person.id = itinerary.p_id
)

/*  end  of  your code  */
```

去过所有地点的人员:  
```sql
-- 10) 查询人员表中去过所有地点的人员姓名。查询结果依人员姓名顺序排序。
--    请用一条SQL语句实现该查询：
use covid19mon;

select person.fullname
from person
where not exists (
    select * from location
    where not exists (
        select * from itinerary
        where itinerary.p_id = person.id and itinerary.loc_id = location.id
    )
)
order by person.fullname

/*  end  of  your code  */ 
```

隔离点的现状视图:  
```sql
-- 11) 建立反映所有隔离点现状的视图isolation_location_status。
-- 内容包括：地点编号，隔离地点名，房间容量，已占用量
-- 请保持原列名不变，已占用量由统计函籹计算得出，该列命名为occupied。 
-- 正在隔离的人占用着隔离点的位置，隔离结束或已转院的人不占用位置。

use covid19mon;
-- drop view if exists isolation_location_status;
create view isolation_location_status
as
select isolation_location.id, isolation_location.location_name, isolation_location.capacity, count(if(isolation_record.state = 1, isolation_record.id, null)) as occupied
from isolation_location
    right join isolation_record on isolation_location.id = isolation_record.isol_loc_id
-- where
    -- isolation_record.state = 1
group by isolation_location.id

/*  end  of  your code  */ 
```

各隔离点的剩余房间数:  
```sql
 -- 12) 从视图isolation_location_status中查询各隔离点的剩余空房间的数目。
--    需要列出的数据项：隔离点名称，剩余房间数。其中剩余房间数为计算得出，请给该列命名为available_rooms
--    查询结果依隔离点编号排序。
--    请用一条SQL语句实现该查询：
use covid19mon;

select location_name, (capacity - occupied) as available_rooms
from isolation_location_status;

/*  end  of  your code  */ 
```

与无症状感染者靳宛儿有过接触的人:  
```sql
-- 13) 筛查发现，靳宛儿为无症状感染者。现需查询其接触者姓名名单和电话，以便通知并安排隔离。查询结题按姓名排序。
--    凡行程表中，在同一地点逗留时间与靳宛儿有交集的，均视为接触者。
--    请用一条SQL语句实现该查询：
use covid19mon;

select P2.fullname, P2.telephone
from itinerary I1, location L1, person P1, person P2, itinerary I2
where
    I1.loc_id = L1.id and
    I1.p_id = P1.id and
    P1.fullname = '靳宛儿' and
    I2.loc_id = I1.loc_id and
    I2.p_id = P2.id and
    (
        (I2.s_time <= I1.s_time and I2.e_time >= I1.s_time) or
        (I2.s_time >= I1.s_time and I2.s_time <= I1.e_time)
    ) and
    P2.fullname <> '靳宛儿'
order by P2.fullname

/*  end  of  your code  */ 
```

每个地点发生的密切接触者人数:  
```sql
-- 14) 依据密切接触表的内容查询每个地点的密切接触者的数量，列出内容包括：地点名称，密接者人数。
--     人数由统计获得，列名命名为close_contact_number.查询结果依密接者人数降序排列。
--     密接者人数相同时，依地点名称排序。
--    请用一条SQL语句实现该查询：
use covid19mon;

select location.location_name, count(*) as close_contact_number
from close_contact, location, person
where
    close_contact.loc_id = location.id and
    close_contact.p_id = person.id
group by close_contact.loc_id
order by close_contact_number desc, location_name

/*  end  of  your code  */ 
```

感染人数最多的人：  
```sql
-- 15) 查询感染人数最多的人员编号，姓名，和被其感染的人数。
--     感染人数由统计所得，命名为infected_number.
-- 说-明：
--     由于数据集close_contact表中的被密接者大多无诊断记录（无法知晓其是否被感染）。
--     增补数据集会影响其它评测，更有同学此前已完成评测。所以，此题暂简化为被密接者就是感染者。
-- 请用一条SQL语句实现该查询：
use covid19mon;

select close_contact.case_p_id, P1.fullname, count(*) as infected_number
from close_contact, person P1, person P2
where
    close_contact.case_p_id = P1.id and
    close_contact.p_id = P2.id
group by close_contact.case_p_id
order by infected_number desc
limit 1

/*  end  of  your code  */
```

行程记录最频繁的3个人:  
```sql
-- 16) 查询2021-02-02 10:00:00到14:00:00期间，行程记录最频繁的3个人的姓名及行程记录条数。
--     记录条数命名为record_number. 记录数并列的，按姓名顺序排列。
--    请用一条SQL语句实现该查询：
use covid19mon;

select person.fullname, count(*) as record_number
from person, itinerary
where
    itinerary.p_id = person.id and
    (
        (
            itinerary.s_time <= '2021-02-02 10:00:00' and
            itinerary.e_time >= '2021-02-02 10:00:00'
        ) or
        (
            itinerary.s_time >= '2021-02-02 10:00:00' and
            itinerary.s_time <= '2021-02-02 14:00:00'
        )
    )
group by person.id
order by record_number desc, person.fullname
limit 3

/*  end  of  your code  */
```

房间数第2多的隔离点:  
```sql
-- 17 查询隔离点中，房间数第二多的隔离点名称和房间数。
--    请用一条SQL语句实现该查询：
use covid19mon;

select location_name, capacity
from isolation_location
where
    capacity in (
        select max(capacity) as x
        from isolation_location
        where capacity < (
            select max(capacity) from isolation_location
        )
    )

/*  end  of  your code  */
```

#### 创建触发器
隔离点的人员“确诊新冠”后，自动转院：  
```sql
-- 用create trigger语句创建符合任务要求的触发器（触发器名称自已命名）：
-- 当隔离表（isolation_record）中的某位隔离人员在诊断表（diagnose_record）中的
-- 诊断结果（result）为1(新冠确诊)时，自动将隔离表中的隔离状态（state)改成3（转入医院）。

-- 你需要考虑：
-- (1) 触发器应该建在哪个表上；
-- (2) 触发器执行的时机；
-- (3) (insert,update,delete)中的哪些(个）事件触发这个触发器运行。

-- 当前你处于MySQL环境下！
use covid19mon;

-- drop trigger if exists update_tirgger;
DELIMITER ;;
create trigger update_tirgger
after update
on diagnose_record for each row
begin
    if new.result = 1
    then
        update isolation_record
        set state = 3
        where isolation_record.p_id = new.p_id ;
    end if;
end;;
DELIMITER ;

-- drop trigger if exists insert_tirgger;
DELIMITER ;;
create trigger insert_tirgger
after insert
on diagnose_record for each row
begin
    if new.result = 1
    then
        update isolation_record
        set state = 3
        where isolation_record.p_id = new.p_id ;
    end if;
end;;
DELIMITER ;

```
结果：todo  
#### 创建函数
```sql
/*
(1)用create function语句创建符合以下要求的函数：
   依据人员编号计算其到达所有地点的次数(即行程表中的记录数)。
   函数名为：Count_Records。函数的参数名可以自己命名:*/
use covid19mon;
set global log_bin_trust_function_creators=1;


/*
(2) 利用创建的函数，仅用一条SQL语句查询在行程表中至少有3条行程记录的人员信息，查询结果依人员编号排序。*/
drop function if exists count_records;
DELIMITER ;;
create
    function count_records(p_id int)
    returns int
    begin
        declare ret int default 0;
        select count(*) into ret
        from itinerary
        where
            itinerary.p_id = p_id;
        return ret;
    end;;

DELIMITER ;

select person.id, person.fullname, person.telephone
from person
where
    (select count_records(id)) >= 3
order by id;
```

结果：todo  
### 任务总结
通过以上练习，我初步熟悉了 sql 语法，并结合实际例子了解了 sql 语句的应用场景，并知道了一般的 sql 查询语句的写法，为后面的数据库应用系统实现打好了基础。  

## 机票预定系统设计
### 系统设计目标
在以上实验练习的基础上，设计并实现一个机票预定系统，满足客户预定航班的基本需求。  
### 需求分析
+ 每个航班信息的输入
+ 每个航班的座位信息的输入
+ 当旅客进行机票预定时，输入旅客基本信息，系统为旅客安排航班，打印取票通知和账单
+ 旅客能够退订机票
+ 能够查询每个航班的预定情况，计算航班的满座率
+ 程序中需体现 sql 和编程语言的结合

### 总体设计
整个数据库系统均用 Rust 语言实现。  
总体设计分两个部分：一个是数据库部分，另一个是图形界面部分。  
数据库部分使用 Rust 社区里面的 `mysql` 库，可以在 Rust 语言里面实现对数据库的 sql 语句操作，比如：  
```Rust
use mysql::prelude::*;
use mysql::*;
#[derive(Debug, PartialEq, Eq)]
struct Entry {
    pub id: i32,
    pub foo: i32,
    pub bar: Option<String>
}

fn main() -> mysql::Result<()> {
    let url = "mysql://root:SKTT1Faker668@localhost/test_db";
    let pool = Pool::new(url)?;
    let mut conn = pool.get_conn()?;
    // 创建临时表
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
    // 插入元组
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

    // 查询
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
```

通过 `mysql` 这个库将 Rust 程序与 mysql 数据库联系起来。  

图形界面部分使用 Rust 社区里面的 `bevy` 游戏引擎，总所周知游戏引擎本身也是一个图形界面框架，这里通过 `bevy` 来实现图形界面的渲染。  

### 数据库设计
数据库设计涉及到四个表：航班信息表，座位信息表，旅客信息表，预定信息表，其中四个表的属性定义如下：  
* 航班信息表（航班编号，飞机型号，起飞时间，飞行时间，客容量，价格）
* 座位信息表（座位编号，航班编号，行，列，是否已经被预定）
* 旅客信息表（身份证，姓名，密码）
* 预定信息表（预定编号，身份证，航班编号，状态）

另外补充信息如下：  
* 座位信息表中的“行”和“列”分别是整数类型的字符串类型
* 旅客通过身份证和姓名还有密码注册相关账号，并通过这些信息登录
* 预定信息表中的“状态”属性有三种：
    + 一个是已经预定了但是没有付款
    + 二是付了款但是订单没有完成（没到达目的地）
    + 三是已经完成（旅客已经顺利到达目的地）

在系统的初始化过程中，将会通过以下代码创建相应的表：  
```Rust
/// 创建航班信息表
pub fn create_flights_table(db: &mut PooledConn, tb_name: &str) -> mysql::Result<()> {
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
```

详细设计将在后一节介绍。  

### 详细设计与实现
#### SqlEntry trait 的定义
为了统一系统中各种数据对象和 sql 语句转换的抽象，下面定义一个 SqlEntry trait:  
```Rust
/// Sql trait 统一抽象各种数据库数据
pub trait SqlEntry: Sized {
    /// 插入一行记录到数据库
    fn insert(&self, db: &mut PooledConn, tb_name: &str) -> mysql::Result<()>;
    /// 从查询的数据中构建结构体
    fn select(db: &mut PooledConn, tb_name: &str, id: usize) -> mysql::Result<Self>;
}
```
`trait` 是 Rust 语言中的概念，可以理解为 Java 中的接口。  
#### 航班信息的设计与实现
首先定义一个结构体：  
```Rust
/// 航班信息表项
#[derive(Debug)]
pub struct Flight {
    /// 航班编号
    pub(crate) id: usize,
    /// 机器型号
    pub(crate) mtype: String,
    /// 起飞时间
    pub(crate) stime: String,
    /// 预计飞行时间(单位为分钟)
    pub(crate) ftime: u32,
    /// 客容量
    pub(crate) capacity: u32,
    /// 价格
    pub(crate) price: u32
}
```
`Flight` 结构体中的成员对应之前提到的航班信息表的各个属性。  
然后为其实现 SqlEntry trait:  
```Rust
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
```
这样子就可以比较方便地对航班信息表进行插入和选择操作：  
```Rust
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
```

对其他表实现 SqlEntry 的理由与航班信息表信息表一致，都是为了代码的简洁，书写方便。  
#### 座位信息的设计与实现
定义 `SeatInfo` 结构体：  
```Rust
/// 座位信息
#[derive(PartialEq, Eq, Debug)]
pub struct SeatInfo {
    /// 座位编号
    pub id: usize,
    /// 所属航班编号
    pub flight_id: usize,
    /// 排号，座位号，比如 11A
    pub location: (usize, String),
    /// 是否已经被预定
    pub is_booked: bool
}
```

为其实现 SqlEntry trait:  
```Rust
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
```

#### 乘客信息的设计与实现
定义 `Passenger` 结构体：  
```Rust
#[derive(Debug, PartialEq, Eq)]
pub struct Passenger {
    /// 身份证
    pub id_card: usize,
    /// 姓名
    pub name: String,
    /// 密码
    pub password: String,
}
```

为其实现 SqlEntry:  
```Rust
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
```

#### 预定信息的设计与实现
定义结构体 `BookedRecord`：   
```Rust
/// 航班预定信息
#[derive(PartialEq, Eq, Debug)]
pub struct BookedRecord {
    /// 记录编号
    pub id: usize,
    /// 旅客身份证
    pub pid_card: usize,
    /// 航班编号
    pub flight_id: usize,
    /// 预定状态
    pub state: BookdedState
}
```

为其实现 SqlEntry trait:  
```Rust
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
```

### 系统测试
在本项目中的开发过程中，作者写了许多单元测试，这些单元测试起到一定的代码正确性检查作用：  
```Rust

```
### 系统设计与实现总结

## 课程总结