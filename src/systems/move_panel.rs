use std::borrow::BorrowMut;
use std::ops::Deref;
use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    core::SystemDesc,
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
};
use amethyst::core::math::Vector3;
use amethyst::input::{VirtualKeyCode};
use amethyst::renderer::{Sprite, SpriteRender};
use crate::systems::{Controller, ControllerSystem};

use crate::tetris::{HEART_PANEL, INVERTED_TRIANGLE_PANEL, Panel, PANEL_HEIGHT, PANEL_WIDTH, STAR_PANEL, TRIANGLE_PANEL};

const TIME_BETWEEN_MOVES: f32 = 0.3;


pub struct MovePanelSystem {
    move_timer: f32
}


impl Default for MovePanelSystem {
    fn default() -> Self {
        MovePanelSystem { move_timer: TIME_BETWEEN_MOVES }
    }
}

impl<'s> System<'s> for MovePanelSystem {
    type SystemData = (
        WriteStorage<'s, Panel>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, SpriteRender>,
        Read<'s, Controller>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut panels, mut locals, mut sprites, controller, time): Self::SystemData) {
        for(panel, local, sprite) in (&mut panels, &mut locals, &mut sprites).join() {
            let dt = time.delta_seconds();
            self.move_timer -= dt;
            let (dx, dy) = {
                let mut dx = 0.0;
                let mut dy = 0.0;
                if controller.is_key_just_pressed(VirtualKeyCode::Left){
                    dx -= PANEL_WIDTH;
                    self.move_timer = TIME_BETWEEN_MOVES;
                    sprite.sprite_number = HEART_PANEL;
                }
                else if controller.is_key_just_pressed(VirtualKeyCode::Right){
                    dx += PANEL_WIDTH;
                    self.move_timer = TIME_BETWEEN_MOVES;
                    sprite.sprite_number = TRIANGLE_PANEL;
                }
                else if controller.is_key_just_pressed(VirtualKeyCode::Up){
                    dy += PANEL_HEIGHT;
                    self.move_timer = TIME_BETWEEN_MOVES;
                    sprite.sprite_number = STAR_PANEL;
                }
                else if controller.is_key_just_pressed(VirtualKeyCode::Down){
                    dy -= PANEL_HEIGHT;
                    self.move_timer = TIME_BETWEEN_MOVES;
                    sprite.sprite_number = INVERTED_TRIANGLE_PANEL;
                }
                (dx, dy)
            };
            local.prepend_translation(Vector3::new(dx, dy, 0.0));


        }
    }

}