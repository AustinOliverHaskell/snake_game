use raylib::prelude::*;

const SCREEN_WIDTH : u32 = 640;
const SCREEN_HEIGHT: u32 = 480;

const STARTING_SNAKE_LENGTH: u32 = 3;

const MAX_APPLE_DURATION: f32 = 5.0;

#[derive(PartialEq, Clone, Copy, Debug)]
enum Direction {
    UP,
    LEFT,
    DOWN,
    RIGHT
}

#[derive(PartialEq, Clone, Copy)]
struct SnakePart {
    x: f32,
    y: f32,
    direction: Direction
}

#[derive(PartialEq, Clone)]
struct Snake {
    parts: Vec<SnakePart>,
    head_direction: Direction
}

impl Snake {
    fn head(self: &Self) -> SnakePart {
        return self.parts.last().unwrap().clone();
    }
}

struct TextureMap {
    snake_head_texture:     Texture2D, 
    snake_corner_texture:   Texture2D, 
    snake_middle_texture:   Texture2D, 
    snake_tail_texture:     Texture2D,
    apple_texture:          Texture2D,
    background_texture:     Texture2D
}

#[derive(PartialEq, Clone, Copy, Debug)]
struct Apple {
    x: f32,
    y: f32,
    time_left: f32
}

fn main() {
    let (mut raylib, render_thread) = raylib::init()
        .size(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
        .title("Snake!")
        .build();

    let audio_device = RaylibAudio::init_audio_device();
    if audio_device.is_err() {
        println!("Failed to initialize audio device...");
        return; 
    } 
    let audio_device = audio_device.unwrap();

    let gulp_sound   = audio_device.new_sound(r"D:\Projects\snake_game\assets\sounds\woosh.mp3");
    if  gulp_sound.is_err() {
        println!("Failed to load gulp sound effect. ");
        return;
    }
    let gulp_sound = gulp_sound.unwrap();

    let background_music   = audio_device.new_music(r"D:\Projects\snake_game\assets\music\jungle_background.mp3");
    if  background_music.is_err() {
        println!("Failed to load background music. ");
        return;
    }
    let mut background_music = background_music.unwrap();
    background_music.set_volume(1.0);
    background_music.play_stream();
    if !background_music.is_stream_playing() {
        println!("Music isnt playing...");
    }

    let texture_map = TextureMap {
        snake_head_texture:     raylib.load_texture(&render_thread, r"D:\Projects\snake_game\assets\sprites\snake_head_20x20.png").unwrap(),
        snake_corner_texture:   raylib.load_texture(&render_thread, r"D:\Projects\snake_game\assets\sprites\snake_corner_20x20.png").unwrap(),
        snake_middle_texture:   raylib.load_texture(&render_thread, r"D:\Projects\snake_game\assets\sprites\snake_middle_20x20.png").unwrap(),
        snake_tail_texture:     raylib.load_texture(&render_thread, r"D:\Projects\snake_game\assets\sprites\snake_tail_20x20.png").unwrap(),
        apple_texture:          raylib.load_texture(&render_thread, r"D:\Projects\snake_game\assets\sprites\apple_20x20.png").unwrap(),
        background_texture:     raylib.load_texture(&render_thread, r"D:\Projects\snake_game\assets\sprites\background.png").unwrap(),
    };

    let mut snake = create_starting_snake();

    let mut apple = Apple {
        x: 200.0,
        y: 200.0,
        time_left: MAX_APPLE_DURATION
    };

    let play_area = Rectangle {
        x: 20.0, 
        y: 40.0, 
        width:  SCREEN_WIDTH  as f32 - 40.0, 
        height: SCREEN_HEIGHT as f32 - 80.0
    };

    let mut score: u16 = 0;

    const TIME_TO_MOVE: f32             = 0.15;
    let mut time_since_last_move: f32   = 0.0;

    let mut game_over: bool = false;

    let mut enlarge_snake: bool = false;
    while !raylib.window_should_close() {

        if game_over && raylib.is_key_released(KeyboardKey::KEY_ENTER) {
            game_over   = false;
            snake       = create_starting_snake();
        }

        background_music.update_stream();
        if !background_music.is_stream_playing() {
            background_music.play_stream();
        }

        if raylib.is_key_released(KeyboardKey::KEY_W)        && snake.head_direction != Direction::DOWN  {
            snake.head_direction = Direction::UP;
        } else if raylib.is_key_released(KeyboardKey::KEY_D) && snake.head_direction != Direction::LEFT  {
            snake.head_direction = Direction::RIGHT;
        } else if raylib.is_key_released(KeyboardKey::KEY_S) && snake.head_direction != Direction::UP    {
            snake.head_direction = Direction::DOWN;
        } else if raylib.is_key_released(KeyboardKey::KEY_A) && snake.head_direction != Direction::RIGHT {
            snake.head_direction = Direction::LEFT;
        }

        let speed_up: bool = raylib.is_key_down(KeyboardKey::KEY_SPACE);

        let apple_overlaps_with_snake_head = Rectangle {
                x: apple.x, 
                y: apple.y, 
                width:  20.0, 
                height: 20.0
        }.check_collision_point_rec(
            Vector2 { x: snake.head().x, y: snake.head().y }
        );

        if apple_overlaps_with_snake_head {
            enlarge_snake = true;
            gulp_sound.play();
            place_apple(&mut apple, &snake, &raylib, play_area);
            score += 1;
        }

        if apple.time_left <= 0.0 {
            place_apple(&mut apple, &snake, &raylib, play_area);
        }

        let mut draw_context = raylib.begin_drawing(&render_thread);

        if !game_over {
            apple.time_left      -= draw_context.get_frame_time();
            time_since_last_move += draw_context.get_frame_time();
            if speed_up {
                time_since_last_move += draw_context.get_frame_time();
            }
            if time_since_last_move >= TIME_TO_MOVE {
                propogate_snake_movement(&mut snake, enlarge_snake);
                enlarge_snake = false;
                time_since_last_move = 0.0;
            }
        }

        draw_game(&mut draw_context, &texture_map, &snake, &apple);        

        if game_over {
            draw_context.draw_text("GAME OVER", 640 / 2 - 50, 480 / 2, 20, Color::BLACK);
        }

        if !game_over {
            let is_snake_inside_bounds = play_area.check_collision_point_rec(Vector2 { x: snake.head().x, y: snake.head().y });
            if !is_snake_inside_bounds {
                game_over = true; 
            }

            if does_snake_self_intersect(&snake) {
                game_over = true;
            }

            draw_context.draw_text(&("Score: ".to_string() + &score.to_string()), 12, 12, 20, Color::BLACK);
        }
    }
}

// @note: This assumes that all images are drawn facing up
fn map_direction_to_rotation(direction: Direction) -> f32 {
    match direction {
        Direction::UP    =>    0.0,
        Direction::LEFT  =>  270.0,
        Direction::DOWN  =>  180.0,
        Direction::RIGHT =>   90.0,
    }
}

// @note: This assumes that the directions are different. The error 
//  for that is handled by returning 0.0. 
// @note: The rest of the system should forbid the case of current
//  and previous pointing in different directions. Here we will also
//  just return a rotation of 0.0 if we get that input. 
fn map_difference_in_direction_to_rotation_for_snake_corner(
    segment_before: Direction, 
    current_direction:  Direction ) -> f32 {

    if segment_before == current_direction {
        println!("WARN: This function isnt meant to be used for non-corner snake peices");
        return 0.0;
    }

    match segment_before {
        Direction::UP    => return match current_direction {
            Direction::LEFT  => 180.0,
            Direction::RIGHT =>  90.0,
            _ => 0.0
        },
        Direction::LEFT  => return match current_direction {
            Direction::UP    => 270.0,
            Direction::DOWN  => 180.0,
            _ => 0.0
        },
        Direction::DOWN  => return match current_direction {
            Direction::LEFT  =>   0.0,
            Direction::RIGHT => 270.0,
            _ => 0.0
        },
        Direction::RIGHT => return match current_direction {
            Direction::UP    => 0.0,
            Direction::DOWN  => 90.0,
            _ => 0.0
        },
    }
}

fn draw_game(
    draw_context:           &mut RaylibDrawHandle, 
    texture_map:            &TextureMap,
    snake:                  &Snake,
    apple:                  &Apple) {

    draw_context.clear_background(Color::WHITE);

    draw_context.draw_texture(&texture_map.background_texture, 0, 0, Color::WHITE);

    draw_snake(
        draw_context,
        &texture_map,
        &snake,
    );

    draw_apple(draw_context, &texture_map.apple_texture, apple);
}

fn draw_apple(
    draw_context:  &mut RaylibDrawHandle, 
    apple_texture: &Texture2D,
    apple:         &Apple) {

    draw_context.draw_texture(
        apple_texture, apple.x as i32, apple.y as i32, Color {
            r: 255, g: 255, b: 255, a: lerp(0.0, 255.0, apple.time_left) as u8
        }
    );
}

fn draw_snake_part(
    draw_context: &mut RaylibDrawHandle,
    texture:      &Texture2D,
    x:            f32,
    y:            f32,
    rotation:     f32 ) {

    draw_context.draw_texture_pro(
        texture, 
        Rectangle {
            x: 0.0,
            y: 0.0,
            width:  texture.width as f32,
            height: texture.height as f32
        },
        Rectangle {
            x: x + 10.0, 
            y: y + 10.0,
            width:  texture.width  as f32,
            height: texture.height as f32
        },
        Vector2 { x: 10.0, y: 10.0 },
        rotation,
        Color::WHITE
    );

}

fn draw_snake(
    draw_context: &mut RaylibDrawHandle, 
    texture_map:  &TextureMap,
    snake:        &Snake) {

    let mut previous_direction: Option<Direction> = None;
    for snake_part in (&snake.parts).into_iter().rev() {
        // Draw head
        if previous_direction.is_none() {
            draw_snake_part(
                draw_context, 
                &texture_map.snake_head_texture, 
                snake_part.x, 
                snake_part.y, 
                map_direction_to_rotation(snake.head_direction)
            );
            
            previous_direction = Some(snake.head_direction);
        } else {
            // Shouldnt ever hit this, but just here for safety. 
            if previous_direction.is_none() {
                continue;
            }

            // Do we need to draw a middle straight segment?
            if snake_part.direction == previous_direction.unwrap() {
                match snake_part.direction {
                    Direction::UP   | Direction::DOWN   => {
                        draw_snake_part(
                            draw_context, 
                            &texture_map.snake_middle_texture, 
                            snake_part.x, 
                            snake_part.y, 
                            0.0
                        );
                    }
                    Direction::LEFT | Direction::RIGHT => {
                        draw_snake_part(
                            draw_context, 
                            &texture_map.snake_middle_texture, 
                            snake_part.x, 
                            snake_part.y, 
                            90.0
                        );
                    }
                }
            // @note: Curved segment
            } else {

                // draw_snake_part(
                //     draw_context, 
                //     &texture_map.snake_corner_texture, 
                //     snake_part.x, 
                //     snake_part.y, 
                //     map_difference_in_direction_to_rotation_for_snake_corner(
                //         previous_direction.unwrap(), 
                //         snake_part.direction
                //     )
                // );

                draw_context.draw_rectangle_rounded(
                    Rectangle {
                        x: snake_part.x, 
                        y: snake_part.y, 
                        width:  20.0, 
                        height: 20.0
                    }, 0.5, 10, Color::GRAY
                );
            }

            previous_direction = Some(snake_part.direction);
        }
    }

}

fn create_starting_snake() -> Snake {
    let mut snake_parts: Vec<SnakePart> = Vec::new();
    for _ in 0..STARTING_SNAKE_LENGTH-1 {
        snake_parts.push(SnakePart {
            x: 100.0,
            y: 120.0,
            direction: Direction::RIGHT
        });
    }
    snake_parts.push(SnakePart {
        x: 120.0,
        y: 120.0,
        direction: Direction::RIGHT
    });

    Snake {
        parts: snake_parts,
        head_direction: Direction::RIGHT
    }
}

fn propogate_snake_movement(snake: &mut Snake, enlarge_snake: bool) {
    let mut new_head = snake.head();
    match snake.head_direction {
        Direction::UP => {
            new_head.y -= 20.0;
        }, 
        Direction::LEFT => {
            new_head.x -= 20.0;
        }, 
        Direction::DOWN => {
            new_head.y += 20.0;
        },
        Direction::RIGHT => {
            new_head.x += 20.0;
        }
    }
    new_head.direction = snake.head_direction;
    if !enlarge_snake {
        snake.parts.remove(0);
    }

    snake.parts.push(new_head);
}

fn does_snake_self_intersect(snake: &Snake) -> bool {

    for index in 0..snake.parts.len()-1 {
        let snake_part = snake.parts[index];

        if (Rectangle {
            x: snake_part.x,
            y: snake_part.y,
            width:  20.0,
            height: 20.0
        }).check_collision_point_rec(
            Vector2 { x: snake.head().x, y: snake.head().y }
        ) {
            return true;
        }
    }

    false
}

fn place_apple(apple: &mut Apple, snake: &Snake, raylib_handle: &RaylibHandle, bounds: Rectangle) {
    loop {
        let new_apple = Apple {
            x: raylib_handle.get_random_value::<i32>(bounds.x as i32..(bounds.x+bounds.width)  as i32) as f32,
            y: raylib_handle.get_random_value::<i32>(bounds.y as i32..(bounds.y+bounds.height) as i32) as f32,
            time_left: MAX_APPLE_DURATION
        };
        let new_apple = snap_apple_to_grid(&new_apple);
        println!("New apple location is: {:?}", new_apple);

        let mut pick_new_apple = false;
        for snake_part in &snake.parts {
            let is_snake_where_apple_is = Rectangle {
                x: snake_part.x, 
                y: snake_part.y, 
                width:  20.0, 
                height: 20.0
            }.check_collision_point_rec(Vector2 { x: new_apple.x, y: new_apple.y });
            if is_snake_where_apple_is {
                pick_new_apple = true;
                break; 
            }
        }
        if pick_new_apple {
            continue;
        }

        *apple = new_apple;

        break;
    }
}

fn snap_apple_to_grid(apple: &Apple) -> Apple{
    Apple {
        x: apple.x - apple.x % 20.0,
        y: apple.y - apple.y % 20.0,
        time_left: apple.time_left
    }
}