use bevy::prelude::*;
use polako::eml::*;
use polako::flow::*;
use polako_ui::common::{CommonWidgetsPlugin, Div};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Polako UI Sample".into(),
                resolution: (450., 400.).into(),
                position: WindowPosition::At(IVec2::new(300, 300)),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(FlowPlugin)
        .add_plugins(CommonWidgetsPlugin)
        .add_systems(Startup, hello_world)
        .add_systems(Update, ui_text_system)
        .run();
}

fn hello_world(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.add(eml! {
        resource(time, Time);
        bind(time.elapsed.fmt("{:0.2}") => elapsed.text);
        bind(time.elapsed * 0.5 - 0.5 => content.bg.r);
        bind(content.bg.hex => color.text);
        Body + Name { .value: "body" } [
            content: Column { .bg: #9d9d9dff, } + Style( .padding: [25, 50].into_rect(), )[
                Div { .bg: #dededeff, } + Style ( .padding: 50.into_rect(), ) [
                    "Hello world!"
                ],
                Row [
                    "Elapsed: ", elapsed: Label { .text: "0.00" }
                ],
                Row [
                    "Color: ", color: Label
                ]
            ]
        ]
    });
}

#[derive(Behavior)]
pub struct UiText {
    /// The text value of UiText element.
    pub text: String,
    #[param(default = Color::Srgba(Srgba::hex("2f2f2fff").unwrap()))]
    pub text_color: Color,
}

#[derive(Element)]
#[construct(Label -> UiText -> Div)]
pub struct Label;
impl ElementBuilder for Label {
    fn build_element(_: Vec<Entity>) -> Blueprint<Self> {
        blueprint! {
            Label::Base + TextBundle
        }
    }
}

#[derive(Element)]
#[construct(Body -> Div)]
pub struct Body;
impl ElementBuilder for Body {
    fn build_element(content: Vec<Entity>) -> Blueprint<Self> {
        blueprint! {
            Body::Base
            + Style(
                .width: Val::Percent(100.),
                .height: Val::Percent(100.),
                .justify_content: JustifyContent::Center,
                .align_content: AlignContent::Center,
                .align_items: AlignItems::Center,
            )
            [[ content ]]
        }
    }
}

#[derive(Element)]
#[construct(Column -> Div)]
pub struct Column;
impl ElementBuilder for Column {
    fn build_element(content: Vec<Entity>) -> Blueprint<Self> {
        blueprint! {
            Column::Base
            + Style (
                .display: Display::Flex,
                .flex_direction: FlexDirection::Column,
                .align_items: AlignItems::Center,
                .align_content: AlignContent::Center,
                .row_gap: Val::Px(3.),
            )
            [[ content ]]
        }
    }
}

#[derive(Element)]
#[construct(Row -> Div)]
pub struct Row;
impl ElementBuilder for Row {
    fn build_element(content: Vec<Entity>) -> Blueprint<Self> {
        blueprint! {
            Row::Base
            + Style (
                .display: Display::Flex,
                .flex_direction: FlexDirection::Row,
                .align_items: AlignItems::Center,
                .align_content: AlignContent::Center,
                .column_gap: Val::Px(3.),
            )
            [[ content ]]
        }
    }
}

use polako_constructivism::Singleton;
use polako_flow::input::HoverSignal;
use polako_flow::Signal;
pub struct Signals;
impl Signals {
    pub fn hover(&self) -> &'static <HoverSignal as Signal>::Descriptor {
        <<HoverSignal as Signal>::Descriptor as Singleton>::instance()
    }
}

// helpers for spawning text bundle
impl Default for UiText {
    fn default() -> Self {
        UiText {
            text: "".into(),
            text_color: Color::Srgba(Srgba::hex("2f2f2fff").unwrap()),
        }
    }
}
pub trait WithText {
    fn with_text<T: AsRef<str>>(text: T) -> Self;
}

impl WithText for Text {
    fn with_text<T: AsRef<str>>(text: T) -> Self {
        let mut text = Text::from_section(text.as_ref().to_string(), Default::default());
        text.sections[0].style.font_size = 24.;
        text.sections[0].style.color = Color::Srgba(Srgba::hex("2f2f2fff").unwrap());
        text
    }
}

/// bypass UiText text value & color to Text.sections[0] when changed
fn ui_text_system(mut texts: Query<(&UiText, &mut Text), Changed<UiText>>) {
    for (ui_text, mut text) in texts.iter_mut() {
        if text.sections.is_empty() {
            *text = Text::with_text("");
        }
        text.sections[0].value = ui_text.text.clone();
        text.sections[0].style.color = ui_text.text_color;
    }
}

pub trait IntoRect {
    fn into_rect(self) -> UiRect;
}

impl IntoRect for i32 {
    fn into_rect(self) -> UiRect {
        UiRect::all(Val::Px(self as f32))
    }
}
impl IntoRect for f32 {
    fn into_rect(self) -> UiRect {
        UiRect::all(Val::Px(self))
    }
}

impl IntoRect for [i32; 2] {
    fn into_rect(self) -> UiRect {
        UiRect {
            left: Val::Px(self[0] as f32),
            right: Val::Px(self[0] as f32),
            top: Val::Px(self[1] as f32),
            bottom: Val::Px(self[1] as f32),
        }
    }
}
