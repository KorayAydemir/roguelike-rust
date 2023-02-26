use bracket_lib::terminal::{FontCharType, RGB};
use specs::prelude::*;
use specs_derive::*;

#[derive(Component, Debug)]
pub struct Name {
    pub name: String,
}

#[derive(Component, Debug)]
pub struct Monster {}

#[derive(Component)]
pub struct Viewshed {
    pub visible_tiles: Vec<bracket_lib::terminal::Point>,
    pub dirty: bool,
    pub range: i32,
}

#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Renderable {
    pub glyph: FontCharType,
    pub fg: RGB,
    pub bg: RGB,
}

#[derive(Component, Debug)]
pub struct Player {}
