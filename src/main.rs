//! 华中科技大学数据库系统综合实验
//! 
//! 机票预定系统

mod backend;
mod frontend;
mod config;

use std::collections::HashMap;
use std::sync::{Mutex, Arc};
use bevy::{ecs::ResourceRefMut, prelude::*};
use bevy_egui::{EguiContext, EguiPlugin, EguiSettings, egui::{CentralPanel, CtxRef, InnerResponse, SidePanel, TopPanel}};
use frontend::scene::{Scenes, Scene, ShowF};
use frontend::state::StateMachine;
use backend::*;
use mysql::prelude::FromRow;
use mysql::{Pool, PooledConn, prelude::Queryable};
use config::*;

lazy_static::lazy_static! {
    static ref INPUTBOX_KEY: Vec<Vec<&'static str>> = vec![
        vec!["flight-id", "type", "flight-stime", "flight-ftime", "capacity", "price"],
        vec!["seat-id", "seat-flight-id", "row", "column", "is-booked"],
        vec!["id-card", "name", "password"],
        vec!["stime", "etime"],
        vec!["handled-id"],
        vec!["book-id"]
        ];
    static ref INPUTBOX: Arc<Mutex<HashMap<&'static str, String>>> = {
        let mut h = HashMap::new();
        INPUTBOX_KEY.iter().flat_map(|v| v.iter()).for_each(|s| {
            h.insert(*s, String::new());
        });
        Arc::new(Mutex::new(h))
    };
    static ref DB: Arc<Mutex<PooledConn>> = {
        let pool = Pool::new(DB_URL).expect("failed to get pool");
        let conn = pool.get_conn().expect("failed to get conn");
        Arc::new(Mutex::new(conn))
    };
}

static mut USER: Option<usize> = None;

fn main() {
    App::build()
        .add_resource(AppScenes::init())
        .add_resource(StateMachine::<usize, STATE_NUM>::init())
        .add_plugins(DefaultPlugins) // 添加默认插件
        .add_plugin(EguiPlugin) // 添加 egui 插件
        .add_startup_system(setup_system.system())
        .add_system(update_ui_scale_factor.system())
        .add_system(ui_menu.system())
        .run();
}

fn setup_system(_world: &mut World, res: &mut Resources) {
    let mut egui_ctx = res.get_mut::<EguiContext>().expect("failed to get egui context");
    let asset_server = res.get::<AssetServer>().expect("failed to get asset server");
    let handle = asset_server.load("branding/icon.png");
    egui_ctx.set_egui_texture(BEVY_TEXTURE_ID, handle);
    // 在数据库中建表
    let mut db = DB.lock().unwrap();
    create_flights_table(&mut db, "flights").expect("failed to create flights table");
    create_seats_table(&mut db, "seats").expect("failed to create seats table");
    create_passengers_table(&mut db, "passengers").expect("failed to create passengers table");
    create_booked_records_table(&mut db, "booked_records").expect("failed to create booked records table");
    let booked_record = passenger::BookedRecord {
        id: 0,
        pid_card: 1,
        flight_id: 2,
        state: passenger::BookdedState::NotPaied
    };
    booked_record.insert(&mut db, "booked_records").unwrap();
    let booked_record = passenger::BookedRecord {
        id: 1,
        pid_card: 1,
        flight_id: 3,
        state: passenger::BookdedState::NotPaied
    };
    booked_record.insert(&mut db, "booked_records").unwrap();
}

fn update_ui_scale_factor(mut egui_settings: ResMut<EguiSettings>, wins: Res<Windows>) {
    if let Some(win) = wins.get_primary() {
        egui_settings.scale_factor = 1.0 / win.scale_factor();
    }
}

fn ui_menu(
    _world: &mut World,
    res: &mut Resources,
) {
    let mut egui_ctx = res.get_mut::<EguiContext>().expect("faild to get egui context");
    let ctx = &mut egui_ctx.ctx;
    let scenes = res.get_mut::<AppScenes>().unwrap();
    let mut state_machine = res.get_mut::<StateMachine<usize, STATE_NUM>>().unwrap();
    let scene_id = state_machine.scene_id();
    scenes.inner[scene_id].show(ctx, &mut state_machine);
}

type AppScenes<'s> = Scenes<'s, (), STATE_NUM>;
type StateMachineRef<'r> = ResourceRefMut<'r, StateMachine<usize, STATE_NUM>>;
impl<'s> AppScenes<'s> {
    fn init() -> Self {
        show!(SidePanel, left_show_f, |ui| {
            ui.heading("menu");
            button!(ui, s, "flight message input", 0);
            button!(ui, s, "seats info input", 1);
            button!(ui, s, "passenger login", 2);
            
        }, s: &mut StateMachineRef);
        
        show!(TopPanel, top_show_f, |ui| {
            ui.heading("book your flight!");
        }, _s: &mut StateMachineRef);

        show!(CentralPanel, center_show_f0, |ui| {
            let mut b = INPUTBOX.lock().unwrap();
            for k in INPUTBOX_KEY[0].iter() {
                ui.horizontal(|ui| {
                    ui.label(*k);
                    ui.text_edit_singleline(
                        b.get_mut(k).unwrap()
                    );
                });
            }
            button_alpha!(ui, s, "confirm", 3, |db: &mut PooledConn| {
                let mut no_err = true;
                let flight_id = b.get("flight-id").unwrap();
                let flight_type = b.get("type").unwrap();
                let flight_stime = b.get("flight-stime").unwrap();
                let flight_ftime = b.get("flight-ftime").unwrap();
                let flight_capacity = b.get("capacity").unwrap();
                let flight_price = b.get("price").unwrap();
                let id = flight_id.parse::<usize>().unwrap_or_else(|_e| {
                    no_err = false;
                    0
                });
                let capacity = flight_capacity.parse::<u32>().unwrap_or_else(|_e| {
                    no_err = false;
                    0
                });
                let price = flight_price.parse::<u32>().unwrap_or_else(|_e| {
                    no_err = false;
                    0
                });
                let stime = str_to_time(flight_stime).unwrap_or_else(|_e| {
                    no_err = false;
                    (0, 0, 0, 0)
                });
                let ftime = flight_ftime.parse::<u32>().unwrap_or_else(|_e| {
                    no_err = false;
                    0
                });
                if flight_type.is_empty() {
                    println!("mtype is empty");
                    no_err = false;
                }
                if no_err {
                    let t = time::Datetime::new(2021, stime.0, stime.1, stime.2, stime.3, 0, 0);
                    // 输入一切正确
                    let flight = flight::Flight {
                        id,
                        mtype: flight_type.clone(),
                        stime: t.as_sql(),
                        ftime,
                        capacity,
                        price
                    };
                    // 插入数据库
                    flight.insert(db, "flights").expect("failed to insert to database");
                    println!("succeed in inserting to table flights");
                }
                no_err
            });
            
        }, s: &mut StateMachineRef);

        show!(CentralPanel, center_show_f1, |ui| {
            let mut b = INPUTBOX.lock().unwrap();
            for k in INPUTBOX_KEY[1].iter() {
                ui.horizontal(|ui| {
                    ui.label(*k);
                    ui.text_edit_singleline(
                        b.get_mut(k).unwrap()
                    );
                });   
            }
            button_alpha!(ui, s, "confirm", 3, |db: &mut PooledConn| {
                let seat_id = b.get("seat-id").unwrap();
                let seat_flight_id = b.get("seat-flight-id").unwrap();
                let row = b.get("row").unwrap();
                let column = b.get("column").unwrap();
                let is_booked = b.get("is-booked").unwrap();
                let mut no_err = true;
                let id = seat_id.parse::<usize>().unwrap_or_else(|_e| {
                    no_err = false;
                    0
                });
                let flight_id = seat_flight_id.parse::<usize>().unwrap_or_else(|_e| {
                    no_err = false;
                    0
                });
                let row = row.parse::<usize>().unwrap_or_else(|_e| {
                    no_err = false;
                    0
                });
                let column = column.chars().next().unwrap_or_else(|| {
                    no_err = false;
                    'A'
                });
                if !matches!(is_booked.as_str(), "yes" | "no") {
                    no_err = false;
                }
                if no_err {
                    let seat = flight::SeatInfo {
                        id,
                        flight_id,
                        location: (row, column.to_string()),
                        is_booked: matches!(is_booked.as_str(), "yes")       
                    };
                    seat.insert(db, "seats").expect("failed to insert to database");
                    println!("succeed in inserting to table seats");
                }
                no_err
            });
        }, s: &mut StateMachineRef);

        show!(CentralPanel, center_show_f2, |ui| {
            let mut b = INPUTBOX.lock().unwrap();
            for k in INPUTBOX_KEY[2].iter() {
                ui.horizontal(|ui| {
                    ui.label(*k);
                    ui.text_edit_singleline(
                        b.get_mut(k).unwrap()
                    );
                });  
            }
            let id_card = b.get("id-card").unwrap();
            let name = b.get("name").unwrap();
            let password = b.get("password").unwrap();
            button_alpha!(ui, s, "registered", 3, |db: &mut PooledConn| {
                if !sql_select(db, "passengers", |(id_card, name, password)| {
                    passenger::Passenger {
                        id_card, name, password
                    }
                }, format!("where id_card = {}", &id_card).as_str()).is_empty() {
                    // id card 已经存在
                    println!("id card already in use!");
                    return false;
                } else {
                    // 插入数据库
                    let mut no_err = true;
                    let id = id_card.parse::<usize>().unwrap_or_else(|_e| {
                        no_err = false;
                        0
                    });
                    if name.is_empty() || password.is_empty() {
                        println!("name or password is empty");
                        no_err = false;
                    }
                    if no_err {
                        let passenger = passenger::Passenger {
                            id_card: id,
                            name: name.clone(),
                            password: password.clone()
                        };
                        passenger.insert(db, "passengers").expect("failed to insert to database");
                        println!("succeed in inserting to table passengers");
                    }
                    return no_err;
                }
            });
            button_alpha!(ui, s, "login", 4, |db: &mut PooledConn| {
                if id_card.is_empty() || name.is_empty() || password.is_empty() {
                    println!("id card or name or password is empty");
                    return false;
                }
                // 查询数据库里面是否有相应的记录
                let mut select_ret = sql_select(db, "passengers", |(id_card, name, password)| {
                    passenger::Passenger {
                        id_card, name, password
                    }
                }, format!("where id_card = {}", &id_card).as_str());
                if select_ret.is_empty() {
                    // 没有相应记录
                    println!("no such id card");
                    return false;
                } else {
                    let ret = select_ret.pop().unwrap();
                    if ret.name == *name && ret.password == *password {
                        println!("successfully login");
                        let id = id_card.parse::<usize>().unwrap();
                        unsafe { USER = Some(id); }
                        return true;
                    } else {
                        println!("name or password error");
                        return false;
                    }
                }
            });
        }, s: &mut StateMachineRef);

        show!(CentralPanel, center_show_f3, |ui| {
            button!(ui, s, "book", 3);
            button!(ui, s, "unsubscribe or pay", 4);
            button!(ui, s, "logout", 5);
        }, s: &mut StateMachineRef);

        show!(CentralPanel, center_show_f4, |ui| {
            let mut b = INPUTBOX.lock().unwrap();
            for k in INPUTBOX_KEY[3].iter() {
                ui.horizontal(|ui| {
                    ui.label(*k);
                    ui.text_edit_singleline(
                        b.get_mut(k).unwrap()
                    );
                });   
            }
            button!(ui, s, "search", 3);
            button!(ui, s, "back", 4);
        }, s: &mut StateMachineRef);

        show!(CentralPanel, center_show_f5, |ui| {
            let mut b = INPUTBOX.lock().unwrap();
            unsafe {
                if let Some(user_id) = USER {
                    let mut db = DB.lock().unwrap();
                    let select_ret = db.query_map(
                        format!("select * from booked_records where pid_card = {}", user_id),
                        |(id, pid_card, flight_id, state)| {
                            let state = match state {
                                0 => passenger::BookdedState::NotPaied,
                                1 => passenger::BookdedState::PaiedNotFinished,
                                2 => passenger::BookdedState::Finished,
                                _ => panic!("unknown state value!")
                            };
                            passenger::BookedRecord {
                                id,
                                pid_card,
                                flight_id,
                                state
                            }
                        }
                    ).expect("failed to select from database");
                    for r in select_ret {
                        ui.label(format!("booked id: {}", r.id));
                        ui.label(format!("flight id: {}", r.flight_id));
                        ui.label(format!("state: {:?}", r.state));
                    }
                    
                }
            }
            for k in INPUTBOX_KEY[4].iter() {
                ui.horizontal(|ui| {
                    ui.label(*k);
                    ui.text_edit_singleline(
                        b.get_mut(k).unwrap()
                    );
                });    
            }
            let handle_id = b.get("handled-id").unwrap();
            button_alpha!(ui, s, "unsubscribe", 3, |db: &mut PooledConn| {
                // 要处理的航班 id
                if let Ok(id) = handle_id.parse::<usize>() {
                    // todo: 检查输入，表中是否与之相应的记录
                    db.query_drop(
                        format!("delete from booked_records where id = {}", id)
                    ).expect("failed to delete entry in database");
                    println!("delete entry from booked_records");
                    return true
                } else {
                    return false
                }
            });
            button_alpha!(ui, s, "pay", 4, |db: &mut PooledConn| {
                if let Ok(id) = handle_id.parse::<usize>() {
                    db.query_drop(
                        format!("update booked_records set state = 1 where id = {}", id)
                    ).expect("failed to update entry in database");
                    println!("update entry from booked_records");
                    return true
                } else {
                    return false
                }
            });
            button!(ui, s, "back", 5);
        }, s: &mut StateMachineRef);

        show!(CentralPanel, center_show_f6, |ui| {
            let mut b = INPUTBOX.lock().unwrap();
            let s_time = b.get("stime").unwrap();
            let e_time = b.get("etime").unwrap();
            if let Ok(stime) = str_to_time(s_time) {
                if let Ok(etime) = str_to_time(e_time) {
                    let mut db = DB.lock().unwrap();
                    let start = time::Datetime::new(2021, stime.0, stime.1, stime.2, stime.3, 0, 0).as_sql();
                    // 最晚起飞时间
                    let end = time::Datetime::new(2021, etime.0, etime.1, etime.2, etime.3, 0, 0).as_sql();
                    let select_ret = sql_select(&mut db, "flights", |(id, mtype, stime, ftime, capacity, price)| {
                        flight::Flight {
                            id,
                            mtype,
                            stime,
                            ftime,
                            capacity,
                            price
                        }
                    }, format!("where stime >= '{}' and stime <= '{}'", start, end).as_str());
                    // println!("found ret: {:?}", select_ret);
                    for ret in select_ret {
                        ui.horizontal(|ui| {
                            ui.label(
                                format!("id: {}, stime: {}, ftime: {}, price: {}", ret.id, ret.stime, ret.ftime, ret.price)
                            );
                        });
                    }    
                }
            }
            for k in INPUTBOX_KEY[5].iter() {
                ui.horizontal(|ui| {
                    ui.label(*k);
                    ui.text_edit_singleline(
                        b.get_mut(k).unwrap()
                    );
                });   
            }
            button_alpha!(ui, s, "book", 3, |db: &mut PooledConn| {
                let book_id = b.get("book-id").unwrap();
                if let Ok(id) = book_id.parse::<usize>() {
                    // todo: 修改 booked_records 表，seats
                    let booked_record = passenger::BookedRecord {
                        id: 0,
                        pid_card: unsafe { USER.unwrap() },
                        flight_id: id,
                        state: passenger::BookdedState::NotPaied
                    };
                    booked_record.insert(db, "booked_records").expect("failed to insert to database");
                    true
                } else {
                    false
                }
            });
            button!(ui, s, "back", 4);
        }, s: &mut StateMachineRef);

        let scene0 = scene!(
            "Scene0".to_string(),
            "left side panel 0",
            SIDE_PANEL_WIDTH,
            "top side panel 0",
            None,
            left_show_f,
            top_show_f,
            center_show_f0
        );

        let scene1 = scene!(
            "Scene1".to_string(),
            "left side panel 1",
            SIDE_PANEL_WIDTH,
            "top side panel 1",
            None,
            left_show_f,
            top_show_f,
            center_show_f1
        );

        let scene2 = scene!(
            "Scene2".to_string(),
            "left side panel 2",
            SIDE_PANEL_WIDTH,
            "top side panel 2",
            None,
            left_show_f,
            top_show_f,
            center_show_f2
        );

        let scene3 = scene!(
            "Scene3".to_string(),
            "left side panel 3",
            SIDE_PANEL_WIDTH,
            "top side panel 3",
            None,
            left_show_f,
            top_show_f,
            center_show_f3
        );

        let scene4 = scene!(
            "Scene4".to_string(),
            "left side panel 4",
            SIDE_PANEL_WIDTH,
            "top side panel 4",
            None,
            left_show_f,
            top_show_f,
            center_show_f4
        );

        let scene5 = scene!(
            "Scene5".to_string(),
            "left side panel 5",
            SIDE_PANEL_WIDTH,
            "top side panel 5",
            None,
            left_show_f,
            top_show_f,
            center_show_f5
        );

        let scene6 = scene!(
            "Scene6".to_string(),
            "left side panel 6",
            SIDE_PANEL_WIDTH,
            "top side panel 6",
            None,
            left_show_f,
            top_show_f,
            center_show_f6
        );

        Self {
            inner: [
                scene0, scene1, scene2, scene3,
                scene4, scene5, scene6
            ]
        }
    }
}

impl<'s> Default for AppScenes<'s> {
    fn default() -> Self {
        let inner = [
            Scene::default(), Scene::default(), Scene::default(), Scene::default(),
            Scene::default(), Scene::default(), Scene::default()];
        Self {
            inner
        }
    }
}

#[macro_export]
macro_rules! button {
    ($ui:expr, $s:expr, $name:expr, $input:expr) => {
        if $ui.button($name).clicked() {
            let curr_state = $s.current_state();
            if let Some(next_state) = $s.state_transfer($input) {
                println!("state conversion: {} -> {}", curr_state, next_state);
            }
        }
    }
}

#[macro_export]
macro_rules! button_alpha {
    ($ui:expr, $s:expr, $name:expr, $input:expr, $closure:expr) => {
        if $ui.button($name).clicked() {
            let mut db = DB.lock().unwrap();
            if $closure(&mut db) {
                let curr_state = $s.current_state();
                if let Some(next_state) = $s.state_transfer($input) {
                    println!("state conversion: {} -> {}", curr_state, next_state);
                }
            }
        }
    }
}

#[macro_export]
macro_rules! show {
    ($ty:ty, $ident:ident, $closure:expr, $($s:ident: $s_ty:ty)?) => {
        fn $ident(
            p: $ty,
            ctx: &CtxRef,
            $($s: $s_ty)?
        ) -> InnerResponse<()> {
            p.show(ctx, $closure)
        }
    };
}

#[macro_export]
macro_rules! scene {
    ($name:expr, $left_src:expr, $width:expr, $top_src:expr,
        $frame:expr, $left_f:expr, $top_f:expr, $center_f:expr ) => {
            Scene::<&'s str, ShowF<SidePanel, ()>, ShowF<TopPanel, ()>, ShowF<CentralPanel, ()>, ()>::new(
                $name, $left_src, $width, $top_src, $frame, $left_f, $top_f, $center_f
            )
    };
}

fn sql_select<S: AsRef<str>, T: FromRow, F: FnMut(T) -> U, U>(
    db: &mut PooledConn, tb_name: S, closure: F, constraint: S
) -> Vec<U> {
    let query = format!(
        r#"select * from {} {}"#,
        tb_name.as_ref(),
        constraint.as_ref()
    );
    db.query_map(query, closure).expect("failed to select from database")
}

// month, day, hour, minute
fn str_to_time<S: AsRef<str>>(s: S) -> std::result::Result<(u8, u8, u8, u8), ()> {
    let s = String::from(s.as_ref());
    let split_ret: Vec<&str> = s.split('-').collect();
    if split_ret.len() < 4 {
        Err(())
    } else {
        let mut ret = Vec::new();
        for s in split_ret.iter() {
            if let Ok(d) = s.parse::<u8>() {
                ret.push(d);
            } else {
                return Err(());
            }
        }
        Ok((
            ret[0], ret[1], ret[2], ret[3]
        ))
    }
}