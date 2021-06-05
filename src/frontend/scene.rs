//! 场景的统一抽象

use std::marker::PhantomData;

use bevy_egui::egui::{CtxRef, InnerResponse, Ui};

#[derive(Default)]
pub struct Scene<F: FnOnce(&CtxRef) -> InnerResponse<()>>
{
    f: F,
}

impl<F> Scene<F>
where
    F: FnOnce(&CtxRef) -> InnerResponse<()>
{
    pub fn new(f: F) -> Self {
        Self {
            f,
        }
    }
}

