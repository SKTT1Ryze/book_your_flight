//! 华中科技大学数据库系统综合实验
//! 
//! 机票预定系统

mod backend;
mod frontend;
mod config;

use bevy::{ecs::ResourceRefMut, prelude::*};
use bevy_egui::{EguiContext, EguiPlugin, EguiSettings, egui::{self, CentralPanel, CtxRef, InnerResponse, SidePanel, TopPanel}};
use frontend::scene::{Scenes, Scene, ShowF};
use frontend::state::StateMachine;
use config::*;

fn main() {
    App::build()
        .add_resource(AppScenes::default())
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
    let state_machine = res.get_mut::<StateMachine<usize, STATE_NUM>>().unwrap();
    let scene = scenes.inner.iter().next().unwrap();
    scene.show(ctx, &state_machine);
}

type AppScenes<'s> = Scenes<'s, (), STATE_NUM>;
type StateMachineRef<'r> = ResourceRefMut<'r, StateMachine<usize, STATE_NUM>>;
impl<'s> AppScenes<'s> {
    fn init() -> Self {
        fn left_show_f(
            l: SidePanel,
            ctx: &CtxRef,
            s: &mut StateMachineRef
        ) -> InnerResponse<()> {
            l.show(ctx, |ui| {
                ui.heading("menu");
                // if ui.button("flight message input").clicked() {
                //     let curr_state = s.current_state();
                //     if let Some(next_state) = s.state_transfer(0) {
                //         println!("state conversion: {} -> {}", curr_state, next_state);
                //     }
                // }
                button!(ui, s, "flight message input", 0);
                button!(ui, s, "seats info input", 1);
                button!(ui, s, "passenger login", 2);
            })
        }
        
        fn top_show_f(
            t: TopPanel,
            ctx: &CtxRef,
            s: &mut StateMachineRef
        ) -> InnerResponse<()> {
            t.show(ctx, |ui| {
                ui.heading("book your flight!");
            })
        }

        fn center_show_f0(
            c: CentralPanel,
            ctx: &CtxRef,
            s: &mut StateMachineRef
        ) -> InnerResponse<()> {
            c.show(ctx, |ui| {
                button!(ui, s, "confirm", 3);
            })
        }

        let scene0 = Scene::<&'s str, ShowF<SidePanel, ()>, ShowF<TopPanel, ()>, ShowF<CentralPanel, ()>, ()>::new(
            "Scene0".to_string(),
            "left side panel 0",
            SIDE_PANEL_WIDTH,
            "top side panel 0",
            None,
            left_show_f,
            top_show_f,
            center_show_f0
        );

        fn center_show_f1(
            c: CentralPanel,
            ctx: &CtxRef,
            s: &StateMachineRef
        ) -> InnerResponse<()> {
            c.show(ctx, |ui| {
                button!(ui, s, "confirm", 3);
            })
        }

        let scene1 = Scene::<&'s str, ShowF<SidePanel, ()>, ShowF<TopPanel, ()>, ShowF<CentralPanel, ()>, ()>::new(
            "Scene1".to_string(),
            "left side panel 1",
            SIDE_PANEL_WIDTH,
            "top side panel 1",
            None,
            left_show_f,
            top_show_f,
            center_show_f1
        );

        fn center_show_f2(
            c: CentralPanel,
            ctx: &CtxRef,
            s: &StateMachineRef
        ) -> InnerResponse<()> {
            c.show(ctx, |ui| {
                button!(ui, s, "registered", 3);
                button!(ui, s, "login", 4);
            })
        }

        let scene2 = Scene::<&'s str, ShowF<SidePanel, ()>, ShowF<TopPanel, ()>, ShowF<CentralPanel, ()>, ()>::new(
            "Scene2".to_string(),
            "left side panel 2",
            SIDE_PANEL_WIDTH,
            "top side panel 2",
            None,
            left_show_f,
            top_show_f,
            center_show_f2
        );

        fn center_show_f3(
            c: CentralPanel,
            ctx: &CtxRef,
            s: &StateMachineRef
        ) -> InnerResponse<()> {
            c.show(ctx, |ui| {
                button!(ui, s, "book", 3);
                button!(ui, s, "unsubscribe or pay", 4);
                button!(ui, s, "logout", 5);
            })
        }

        let scene3 = Scene::<&'s str, ShowF<SidePanel, ()>, ShowF<TopPanel, ()>, ShowF<CentralPanel, ()>, ()>::new(
            "Scene3".to_string(),
            "left side panel 3",
            SIDE_PANEL_WIDTH,
            "top side panel 3",
            None,
            left_show_f,
            top_show_f,
            center_show_f3
        );

        fn center_show_f4(
            c: CentralPanel,
            ctx: &CtxRef,
            s: &StateMachineRef
        ) -> InnerResponse<()> {
            c.show(ctx, |ui| {
                button!(ui, s, "search", 3);
                button!(ui, s, "back", 4);
            })
        }

        let scene4 = Scene::<&'s str, ShowF<SidePanel, ()>, ShowF<TopPanel, ()>, ShowF<CentralPanel, ()>, ()>::new(
            "Scene4".to_string(),
            "left side panel 4",
            SIDE_PANEL_WIDTH,
            "top side panel 4",
            None,
            left_show_f,
            top_show_f,
            center_show_f4
        );

        fn center_show_f5(
            c: CentralPanel,
            ctx: &CtxRef,
            s: &StateMachineRef
        ) -> InnerResponse<()> {
            c.show(ctx, |ui| {
                button!(ui, s, "unsubscribe", 3);
                button!(ui, s, "pay", 4);
                button!(ui, s, "back", 5);
            })
        }

        let scene5 = Scene::<&'s str, ShowF<SidePanel, ()>, ShowF<TopPanel, ()>, ShowF<CentralPanel, ()>, ()>::new(
            "Scene5".to_string(),
            "left side panel 5",
            SIDE_PANEL_WIDTH,
            "top side panel 5",
            None,
            left_show_f,
            top_show_f,
            center_show_f5
        );

        fn center_show_f6(
            c: CentralPanel,
            ctx: &CtxRef,
            s: &StateMachineRef
        ) -> InnerResponse<()> {
            c.show(ctx, |ui| {
                button!(ui, s, "book", 3);
                button!(ui, s, "back", 4);
            })
        }

        let scene6 = Scene::<&'s str, ShowF<SidePanel, ()>, ShowF<TopPanel, ()>, ShowF<CentralPanel, ()>, ()>::new(
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