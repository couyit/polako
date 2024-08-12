use bevy::prelude::Entity;
use polako_constructivism::{blueprint, Element};
use polako_eml::{Blueprint, ElementBuilder};
use crate::common::Div;

#[derive(Element)]
#[construct(Button -> Div)]
#[signals(

)]
pub struct Button {

}

impl ElementBuilder for Button {
    fn build_element(content: Vec<Entity>) -> Blueprint<Self> {
        blueprint!{
            Button::Base [[content]]
        }
    }
}