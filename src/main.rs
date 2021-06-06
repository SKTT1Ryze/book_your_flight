//! 华中科技大学数据库系统综合实验
//! 
//! 机票预定系统

mod backend;
mod frontend;
mod config;

use bevy::prelude::*;
use bevy_egui::{EguiContext, EguiPlugin, EguiSettings, egui::{self, CtxRef, InnerResponse}};
use frontend::scene::{Scenes, Scene};
use frontend::state::StateMachine;
use config::*;

type AppScenes<'scenes> = Scenes<'scenes, (), 5>;

impl<'scenes> Default for AppScenes<'scenes> {
    fn default() -> Self {
        let inner = [Scene::default(), Scene::default(), Scene::default(), Scene::default(), Scene::default()];
        Self {
            inner
        }
    }
}

fn main() {
    App::build()
        .add_resource(AppScenes::default())
        .add_resource(StateMachine)
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
    let state_machine = res.get_mut::<StateMachine>().unwrap();
    let scene = scenes.inner.iter().next().unwrap();
    scene.show(ctx, &state_machine);
}