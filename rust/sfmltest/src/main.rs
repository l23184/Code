extern crate sfml;

use sfml::system::*;
use sfml::window::*;
use sfml::graphics::*;

extern crate rand;
use rand::distributions::{Range, IndependentSample};

mod math;

mod player;
use player::Player;


static WINDOW_WIDTH: f32 = 1000.;
static WINDOW_HEIGHT: f32 = 1000.;


fn main() {
    // Setup

    // Window
    let mut window = RenderWindow::new(VideoMode::new_init(WINDOW_WIDTH as u32,
                                                           WINDOW_HEIGHT as u32,
                                                           32),
                                       "Game of Life",
                                       window_style::CLOSE,
                                       &ContextSettings::default())
                         .unwrap();

    window.set_framerate_limit(60);
    window.set_vertical_sync_enabled(true);

    // Background
    let background_image = Image::new_from_file("resources/background.jpg").unwrap();
    let background_texture = Texture::new_from_image(&background_image).unwrap();
    let mut background = RectangleShape::new_with_texture(&background_texture).unwrap();
    background.set_size(&Vector2f::new(WINDOW_HEIGHT, WINDOW_WIDTH));

    // Player
    let mut texture = Texture::new_from_file("resources/ship.png").unwrap();
    texture.set_repeated(true);

    let mut sprite = Sprite::new_with_texture(&texture).unwrap();

    sprite.set_origin(&Vector2f::new(25., 25.));

    let mut player = Player {
        x: 100.,
        y: 100.,
        vel_x: 0.,
        vel_y: 0.,
        angle: 0.,
    };

    let enemy_texture = Image::new_from_file("resources/enemy.png").unwrap();
    let enemy_texture = Texture::new_from_image(&enemy_texture).unwrap();

    let mut enemy_sprite = create_enemy(&enemy_texture);

    let mut enemies = vec![];

    enemies.push(create_enemy(&enemy_texture));
    enemies.push(create_enemy(&enemy_texture));
    enemies.push(create_enemy(&enemy_texture));
    enemies.push(create_enemy(&enemy_texture));
    enemies.push(create_enemy(&enemy_texture));
    enemies.push(create_enemy(&enemy_texture));


    let mut is_running = true;

    // Event loop
    'running: while window.is_open() {
        for event in window.events() {
            match event {
                event::Closed => break 'running,
                event::KeyPressed {code, ..} => {
                    match code {
                        Key::Escape => break 'running,
                        Key::Space => is_running = !(is_running),
                        _ => (),
                    }
                }
                _ => (),
            }
        }

        if is_running {
            // Draw
            handle_keyboard_input(&mut player);


            sprite.set_position(&Vector2f::new(player.x, player.y));
            sprite.set_rotation(-player.angle + 180.);

            let mut enemy_sprite_current_position = enemy_sprite.get_position();
            enemy_sprite_current_position.y += 5.;
            enemy_sprite.set_position(&enemy_sprite_current_position);
        }


        window.clear(&Color::new_rgb(255, 255, 255));
        window.draw(&background);

        let mut indeces_to_remove = vec![];

        for index in 0..enemies.len() {
            let mut enemy_current_position = enemies[index].get_position();

            if enemy_current_position.y > WINDOW_HEIGHT {
                indeces_to_remove.push(index);
            } else {
                enemy_current_position.y += 5.;
                enemies[index].set_position(&enemy_current_position);
                window.draw(&mut enemies[index]);
            }
        }

        // for i in indeces_to_remove {
        //     enemies.remove(i);
        // }

        // for enemy in &mut enemies {
        //     let mut enemy_current_position = enemy.get_position();

        //     enemy_current_position.y += 5.;
        //     enemy.set_position(&enemy_current_position);

        //     window.draw(enemy);
        // }

        window.draw(&enemy_sprite);
        window.draw(&sprite);
        window.display();
    }
}

fn handle_keyboard_input(p: &mut Player) {
    if Key::W.is_pressed() {
        p.accelerate();
    }
    if Key::S.is_pressed() {
        p.reverse();
    }
    if Key::A.is_pressed() {
        p.turn_left();
    }
    if Key::D.is_pressed() {
        p.turn_right();
    }

    p.update(WINDOW_WIDTH, WINDOW_HEIGHT);
}


fn create_enemy<'a>(texture: &'a sfml::graphics::Texture) -> Sprite<'a> {
    let range = Range::new(0., WINDOW_WIDTH);
    let mut rng = rand::thread_rng();

    let mut enemy = Sprite::new_with_texture(&texture).unwrap();

    enemy.set_position(&Vector2f::new(range.ind_sample(&mut rng).floor(), 100.));
    enemy.set_rotation(180.);

    enemy
}
