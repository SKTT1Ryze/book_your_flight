//! 场景的统一抽象

use std::marker::PhantomData;
use bevy_egui::egui::{self, CtxRef, InnerResponse};
use super::state::StateMachine;
use crate::config::*;

#[derive(Default)]
pub struct Scene<F: Fn(&CtxRef, &mut StateMachine) -> InnerResponse<R>, R>
{
    show_f: F,
    _maker: PhantomData<R>
}

impl<F, R> Scene<F, R>
where
    F: Fn(&CtxRef, &mut StateMachine) -> InnerResponse<R>
{
    pub fn new(f: F) -> Self {
        Self {
            show_f: f,
            _maker: PhantomData
        }
    }
}

#[derive(Default)]
pub struct Scenes
{
    scenes: Vec<Scene<fn(&CtxRef, &mut StateMachine)->InnerResponse<()>, ()>>
}

impl Scenes {
    pub fn add_scene(
        &mut self,
    ) {
        fn f(ctx: &CtxRef, state_machine: &mut StateMachine) -> InnerResponse<()> {
            egui::SidePanel::left("side_panel", SIDE_PANEL_WIDTH)
                .show(ctx, |ui| {
                    ui.heading("Side Panel");
                    let mut input = String::new();
                    ui.horizontal(|ui| {
                        ui.label("input box: ");
                        ui.text_edit_singleline(&mut input);
                    });
                    if ui.add(egui::Button::new("button")).clicked() {
                        println!("button is cliked!");
                    }
                })
        };
        self.scenes.push(Scene::new(f));
    }
}