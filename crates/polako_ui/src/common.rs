use bevy::app::{App, Plugin, Update};
use bevy::color::{Color, Srgba};
use bevy::prelude::{BackgroundColor, Changed, Entity, NodeBundle, Query, TextBundle, World};
use bevy::ui::{BorderRadius, DefaultUiCamera, TargetCamera, Val};
use polako_constructivism::{blueprint, Construct, Element, Is};
use polako_eml::{Blueprint, Element, ElementBuilder, Empty, EntityMark, Implemented};

pub struct CommonWidgetsPlugin;

impl Plugin for CommonWidgetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (div_system, body_system));
    }
}

#[derive(Element)]
#[construct(Div -> Empty)]
pub struct Div {
    #[prop(construct)]
    pub bg: Color,
    #[prop(construct)]
    pub border_radius: BorderRadius,
}

#[derive(Element)]
#[construct(Body -> Div)]
pub struct Body {
    target_camera: Option<TargetCamera>,
}

impl ElementBuilder for Div {
    fn build_element(content: Vec<Entity>) -> Blueprint<Self> {
        blueprint! {
            Div::Base(.bg)
            + NodeBundle
            [[ content ]]
        }
    }
}

impl ElementBuilder for Body {
    fn build_element(content: Vec<Entity>) -> Blueprint<Self> {
        blueprint! {
            Body::Base [[content]]
        }
    }
}

impl DivDesign {
    // Div can accept string literals as content
    pub fn push_text<'c, S: AsRef<str>>(
        &self,
        world: &mut World,
        content: &'c mut Vec<Entity>,
        text: S,
    ) -> Implemented {
        let entity = world.spawn(TextBundle::with_text(text)).id();
        content.push(entity);
        Implemented
    }
    // Only Div and elements based on Div can be content of the Div
    pub fn push_content<E: Element + Is<Div>>(
        &self,
        _: &mut World,
        content: &mut Vec<Entity>,
        model: EntityMark<E>,
    ) -> Implemented {
        content.push(model.entity);
        Implemented
    }
    ///// Everything based on Div can access the styles using param extensions: `Row { .s.padding: 25 }`
    // pub fn s(&self) -> &'static Styles {
    //     &Styles
    // }
}

pub trait WithText {
    fn with_text<T: AsRef<str>>(text: T) -> Self;
}
impl WithText for TextBundle {
    fn with_text<T: AsRef<str>>(text: T) -> TextBundle {
        let mut text = TextBundle::from_section(text.as_ref(), Default::default());
        text.text.sections[0].style.font_size = 24.;
        text.text.sections[0].style.color = Color::Srgba(Srgba::hex("2f2f2fff").unwrap());
        text
    }
}

fn div_system(
    mut styles: Query<
        (
            &Div,
            &mut bevy::ui::BackgroundColor,
            &mut bevy::ui::BorderRadius,
        ),
        Changed<Div>,
    >,
) {
    styles
        .iter_mut()
        .for_each(|(div, mut bg, mut border_radius)| {
            bg.0 = div.bg;
            *border_radius = div.border_radius;
        });
}

fn body_system(
    mut bodies: Query<(&Body, &mut bevy::ui::TargetCamera), Changed<Div>>,
    default_ui_camera: DefaultUiCamera,
) {
    bodies.iter_mut().for_each(|(body, mut target_camera)| {
        target_camera.0 = body
            .target_camera
            .as_ref()
            .map(|c| c.0)
            .unwrap_or(default_ui_camera.get().unwrap());
    });
}
