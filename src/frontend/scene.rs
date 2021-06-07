//! 场景的统一抽象

use std::marker::PhantomData;
use std::hash::Hash;
use std::default::Default;
use bevy::ecs::ResourceRefMut;
use bevy_egui::egui::Frame;
use bevy_egui::egui::{CentralPanel, CtxRef, InnerResponse, SidePanel, TopPanel};
use super::state::StateMachine;
use crate::config::*;

pub type ShowF<T, R> = fn(T, &CtxRef, &mut ResourceRefMut<StateMachine<usize, STATE_NUM>>) -> InnerResponse<R>;

pub struct Scenes<'s, R, const N: usize> {
    pub inner: [Scene<&'s str, ShowF<SidePanel, R>, ShowF<TopPanel, R>, ShowF<CentralPanel, R>, R>; N]
}

pub struct Scene<S, LF, TF, CF, R>
where
    S: Hash + Copy,
    LF: Fn(SidePanel, &CtxRef, &mut ResourceRefMut<StateMachine<usize, STATE_NUM>>) -> InnerResponse<R> + Copy,
    TF: Fn(TopPanel, &CtxRef, &mut ResourceRefMut<StateMachine<usize, STATE_NUM>>) -> InnerResponse<R> + Copy,
    CF: Fn(CentralPanel, &CtxRef, &mut ResourceRefMut<StateMachine<usize, STATE_NUM>>) -> InnerResponse<R> + Copy
{
    pub name: String,
    pub left_panel: SidePanelBuilder<S>,
    pub left_show: LF,
    pub top_panel: TopPanelBuilder<S>,
    pub top_show: TF,
    pub center_panel: CentralPanelBuilder,
    pub center_show: CF,
    _maker: PhantomData<R>
}

impl<S, LF, TF, CF, R> Scene<S, LF, TF, CF, R>
where
    S: Hash + Copy,
    LF: Fn(SidePanel, &CtxRef, &mut ResourceRefMut<StateMachine<usize, STATE_NUM>>) -> InnerResponse<R> + Copy,
    TF: Fn(TopPanel, &CtxRef, &mut ResourceRefMut<StateMachine<usize, STATE_NUM>>) -> InnerResponse<R> + Copy,
    CF: Fn(CentralPanel, &CtxRef, &mut ResourceRefMut<StateMachine<usize, STATE_NUM>>) -> InnerResponse<R> + Copy
{
    pub fn new(
        name: String,
        left_id_src: S,
        left_max_width: f32,
        top_id_src: S,
        central_frame: Option<Frame>,
        left_show_f: LF,
        top_show_f: TF,
        center_show_f: CF,

    ) -> Self
    {
        Self {
            name,
            left_panel: SidePanelBuilder::new(left_id_src, left_max_width),
            left_show: left_show_f,
            top_panel: TopPanelBuilder::new(top_id_src),
            top_show: top_show_f,
            center_panel: CentralPanelBuilder::new(central_frame),
            center_show: center_show_f,
            _maker: PhantomData
        }
    }

    pub fn show(&self, ctx: &CtxRef, state_machine: &mut ResourceRefMut<StateMachine<usize, STATE_NUM>>) {
        let side_panel = self.left_panel.side_panel();
        let top_panel = self.top_panel.top_panel();
        let center_panel = self.center_panel.center_panel();
        let f = self.left_show;
        f(side_panel, ctx, state_machine);
        let f = self.top_show;
        f(top_panel, ctx, state_machine);
        let f = self.center_show;
        f(center_panel, ctx, state_machine);
    }
}

impl<'s> Default for Scene<&'s str, ShowF<SidePanel, ()>, ShowF<TopPanel, ()>, ShowF<CentralPanel, ()>, ()>
{
    fn default() -> Self {
        fn left_show_f(
            left_panel: SidePanel,
            ctx: &CtxRef,
            state_machine: &mut ResourceRefMut<StateMachine<usize, STATE_NUM>>
        ) -> InnerResponse<()> {
            // todo: handle state machine
            left_panel.show(ctx, |ui| {
                ui.heading("Left Panel");
            })
        }
        fn top_show_f(
            top_panel: TopPanel,
            ctx: &CtxRef,
            state_machine: &mut ResourceRefMut<StateMachine<usize, STATE_NUM>>
        ) -> InnerResponse<()> {
            top_panel.show(ctx, |ui| {
                ui.heading("Top Panel");
            })
        }
        fn center_show_f(
            center_panel: CentralPanel,
            ctx: &CtxRef,
            state_machine: &mut ResourceRefMut<StateMachine<usize, STATE_NUM>>
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

#[derive(Clone, Debug)]
pub struct SidePanelBuilder<S: Hash + Copy> {
    id_src: S,
    max_width: f32
}

impl<S: Hash + Copy> SidePanelBuilder<S> {
    pub fn new(id_src: S, max_width: f32) -> Self {
        Self {
            id_src,
            max_width
        }
    }

    pub fn side_panel(&self) -> SidePanel {
        SidePanel::left(self.id_src, self.max_width)
    }
}

#[derive(Clone, Debug)]
pub struct TopPanelBuilder<S: Hash + Copy> {
    pub id_src: S
}

impl<S: Hash + Copy> TopPanelBuilder<S> {
    pub fn new(id_src: S) -> Self {
        Self {
            id_src
        }
    }

    pub fn top_panel(&self) -> TopPanel {
        TopPanel::top(self.id_src)
    }
}

#[derive(Clone, Debug)]
pub struct CentralPanelBuilder {
    pub frame: Option<Frame>
}

impl CentralPanelBuilder {
    pub fn new(frame: Option<Frame>) -> Self {
        Self {
            frame
        }
    }

    pub fn center_panel(&self) -> CentralPanel {
        self.frame.map_or_else(
            CentralPanel::default,
            |frame| {
                let panel = CentralPanel::default();
                CentralPanel::frame(panel, frame)
            }
        )
    }
}