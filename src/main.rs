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
use config::*;

lazy_static::lazy_static! {
    static ref INPUTBOX_KEY: Vec<Vec<&'static str>> = vec![
        vec!["flight id", "flight type", "flight stime", "flight ftime", "flight capacity", "flight price"],
        vec!["seat id", "seat flight_id", "seat row", "seat column", "seat is_booked"],
        vec!["p id_card", "p name", "p password"],
        vec!["stime", "etime"],
        vec!["flight_id to be handled"]
        ];
    static ref INPUTBOX: Arc<Mutex<HashMap<&'static str, String>>> = {
        let mut h = HashMap::new();
        INPUTBOX_KEY.iter().flat_map(|v| v.iter()).for_each(|s| {
            h.insert(*s, String::new());
        });
        Arc::new(Mutex::new(h))
    };
}

fn main() {
    App::build()
        .add_resource(AppScenes::init())
        .add_resource(StateMachine::<usize, STATE_NUM>::init())
        .add_plugins(DefaultPlugins) // 添加默认插件
        .add_plugin(EguiPlugin) // 添加 egui 插件
        .add_startup_system(setup_system.system())
        // .add_system(update_ui_scale_factor.system())
        .add_system(ui_menu.system())
        .run();
}

fn setup_system(world: &mut World, res: &mut Resources) {
    let mut egui_ctx = res.get_mut::<EguiContext>().expect("failed to get egui context");
    let asset_server = res.get::<AssetServer>().expect("failed to get asset server");
    let handle = asset_server.load("branding/icon.png");
    egui_ctx.set_egui_texture(BEVY_TEXTURE_ID, handle);
}

fn update_ui_scale_factor(mut egui_settings: ResMut<EguiSettings>, wins: Res<Windows>) {
    if let Some(win) = wins.get_primary() {
        egui_settings.scale_factor = 1.0 / win.scale_factor();
    }
}

fn ui_menu(
    world: &mut World,
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
        }, s: &mut StateMachineRef);

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
            button!(ui, s, "confirm", 3);
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
            button!(ui, s, "confirm", 3);
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
            button!(ui, s, "registered", 3);
            button!(ui, s, "login", 4);
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
            for k in INPUTBOX_KEY[4].iter() {
                ui.horizontal(|ui| {
                    ui.label(*k);
                    ui.text_edit_singleline(
                        b.get_mut(k).unwrap()
                    );
                });    
            }
            button!(ui, s, "unsubscribe", 3);
            button!(ui, s, "pay", 4);
            button!(ui, s, "back", 5);
        }, s: &mut StateMachineRef);

        show!(CentralPanel, center_show_f6, |ui| {
            button!(ui, s, "book", 3);
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