use crate::common::Div;
use bevy::prelude::Entity;
use bevy::ui::BorderRadius;
use peniko::Color;
use polako_constructivism::{blueprint, derive_construct, Element};
use polako_eml::{Blueprint, ElementBuilder};
use polako_flow::input::DragStSignal;

#[derive(Element)]
#[construct(Slider -> Div)]
pub struct Slider {
    #[prop(construct)]
    value: SliderValue,
}

#[derive(Element, Default)]
#[construct(SliderHandle -> Div)]
#[signals(
    drag_start: DragStSignal,
)]
pub struct SliderHandle {
    abc: u32,
}

impl ElementBuilder for Slider {
    fn build_element(_: Vec<Entity>) -> Blueprint<Self> {
        blueprint! {
            Slider::Base
        }
    }
}

impl ElementBuilder for SliderHandle {
    fn build_element(_: Vec<Entity>) -> Blueprint<Self> {
        blueprint! {
            SliderHandle::Base(.bg: Color::WHITE)
        }
    }
}

#[derive(Default)]
pub struct SliderValue {
    value: f32,
}

trait SliderProps {
    fn value(&self) -> f32;
    fn set_value(&mut self, value: f32);
}

impl SliderProps for SliderValue {
    fn value(&self) -> f32 {
        self.value
    }

    fn set_value(&mut self, value: f32) {
        self.value = value;
    }
}

derive_construct! {
    seq => SliderValue -> Nothing;
    construct => (value: f32 = 0.) -> {
        SliderValue { value }
    };
    props => {
        value: f32 = [value, set_value];
    };
}
