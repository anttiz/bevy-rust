use crate::game::constants::*;
use bevy::prelude::*;

pub fn spawn_sky(commands: &mut Commands) {
  commands.spawn(SpriteBundle {
      sprite: Sprite {
          color: COLOR_SKY,
          ..Default::default()
      },
      transform: Transform {
          translation: Vec3::new(0.0, SKY_CENTER_Y, 0.0),
          scale: Vec3::new(
              WINDOW_WIDTH,
              SKY_HEIGHT,
              1.0,
          ),
          ..Default::default()
      },
      ..Default::default()
  });
}
