//! 场景的统一抽象

use std::marker::PhantomData;
use std::hash::Hash;
use std::default::Default;
use bevy_egui::egui::Frame;
use bevy_egui::egui::{self, CentralPanel, CtxRef, InnerResponse, SidePanel, TopPanel};
use super::state::StateMachine;
use crate::config::*;

pub type ShowF<T, R> = fn(T, &CtxRef, &mut StateMachine) -> InnerResponse<R>;

pub struct Scene<LF, TF, CF, R>
where
    LF: Fn(SidePanel, &CtxRef, &mut StateMachine) -> InnerResponse<R>,
    TF: Fn(TopPanel, &CtxRef, &mut StateMachine) -> InnerResponse<R>,
    CF: Fn(CentralPanel, &CtxRef, &mut StateMachine) -> InnerResponse<R>
{
    pub name: String,
    pub left_panel: SidePanel,
    pub left_show: LF,
    pub top_panel: TopPanel,
    pub top_show: TF,
    pub center_panel: CentralPanel,
    pub center_show: CF,
    _maker: PhantomData<R>
}



impl<LF, TF, CF, R> Scene<LF, TF, CF, R>
where
    LF: Fn(SidePanel, &CtxRef, &mut StateMachine) -> InnerResponse<R>,
    TF: Fn(TopPanel, &CtxRef, &mut StateMachine) -> InnerResponse<R>,
    CF: Fn(CentralPanel, &CtxRef, &mut StateMachine) -> InnerResponse<R>
{
    pub fn new(
        name: String,
        left_id_src: impl Hash,
        left_max_width: f32,
        top_id_src: impl Hash,
        central_frame: Option<Frame>,
        left_show_f: LF,
        top_show_f: TF,
        center_show_f: CF,

    ) -> Self
    {
        let center_panel = central_frame.map_or_else(
            CentralPanel::default,
            |frame| {
                let panel = CentralPanel::default();
                CentralPanel::frame(panel, frame)
            }
        );
        Self {
            name,
            left_panel: egui::SidePanel::left(left_id_src, left_max_width),
            left_show: left_show_f,
            top_panel: egui::TopPanel::top(top_id_src),
            top_show: top_show_f,
            center_panel,
            center_show: center_show_f,
            _maker: PhantomData
        }
    }
}

impl Default for Scene<ShowF<SidePanel, ()>, ShowF<TopPanel, ()>, ShowF<CentralPanel, ()>, ()>
{
    fn default() -> Self {
        fn left_show_f(
            left_panel: SidePanel,
            ctx: &CtxRef,
            state_machine: &mut StateMachine
        ) -> InnerResponse<()> {
            // todo: handle state machine
            left_panel.show(ctx, |ui| {
                ui.heading("Left Panel");
            })
        }
        fn top_show_f(
            top_panel: TopPanel,
            ctx: &CtxRef,
            state_machine: &mut StateMachine
        ) -> InnerResponse<()> {
            top_panel.show(ctx, |ui| {
                ui.heading("Top Panel");
            })
        }
        fn center_show_f(
            center_panel: CentralPanel,
            ctx: &CtxRef,
            state_machine: &mut StateMachine
        ) -> InnerResponse<()> {
            center_panel.show(ctx, |ui| {
                ui.heading("Center Panel");
            })
        }
        Scene::new(
            format!("default scene"),
            "left side panel",
            SIDE_PANEL_WIDTH,
            "top side panel",
            None,
            left_show_f,
            top_show_f,
            center_show_f
        )
    }
}


pub struct Scenes<R, const N: usize> {
    pub inner: [Scene<ShowF<SidePanel, R>, ShowF<TopPanel, R>, ShowF<CentralPanel, R>, R>; N]
}