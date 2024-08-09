use raylib::prelude::*;

const SCREEN_WIDTH : u32 = 640;
const SCREEN_HEIGHT: u32 = 480;

const STARTING_SNAKE_LENGTH: u32 = 20;

#[derive(PartialEq, Clone, Copy)]
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

#[derive(PartialEq, Clone, Copy, Debug)]
struct Apple {
    x: f32,
    y: f32
}

fn main() {
    let (mut raylib, render_thread) = raylib::init()
        .size(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
        .title("Snake!")
        .build();

    let apple_texture         = raylib.load_texture(&render_thread, r"D:\Projects\snake\assets\apple_20x20.png").unwrap();
    let snake_corner_texture  = raylib.load_texture(&render_thread, r"D:\Projects\snake\assets\snake_corner_20x20.png").unwrap();
    let snake_head_texture    = raylib.load_texture(&render_thread, r"D:\Projects\snake\assets\snake_head_20x20.png").unwrap();
    let snake_middle_texture  = raylib.load_texture(&render_thread, r"D:\Projects\snake\assets\snake_middle_20x20.png").unwrap();

    let mut snake = create_starting_snake();

    let mut apple = Apple {
        x: 200.0,
        y: 200.0
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

        if raylib.is_key_released(KeyboardKey::KEY_W)        {//&& snake_direction != Direction::UP {
            snake.head_direction = Direction::UP;
            time_since_last_move = TIME_TO_MOVE; // @note: Make the move happen NOW. 
        } else if raylib.is_key_released(KeyboardKey::KEY_D) {//&& snake_direction != Direction::RIGHT  {
            snake.head_direction = Direction::RIGHT;
            time_since_last_move = TIME_TO_MOVE; // @note: Make the move happen NOW. 
        } else if raylib.is_key_released(KeyboardKey::KEY_S) {//&& snake_direction != Direction::DOWN {
            snake.head_direction = Direction::DOWN;
            time_since_last_move = TIME_TO_MOVE; // @note: Make the move happen NOW. 
        } else if raylib.is_key_released(KeyboardKey::KEY_A) {//&& snake_direction != Direction::LEFT {
            snake.head_direction = Direction::LEFT;
            time_since_last_move = TIME_TO_MOVE; // @note: Make the move happen NOW. 
        }

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
            place_apple(&mut apple, &snake, &raylib, play_area);
            score += 1;
        }

        let mut draw_context = raylib.begin_drawing(&render_thread);

        if !game_over {
            time_since_last_move += draw_context.get_frame_time();
            if time_since_last_move >= TIME_TO_MOVE {
                propogate_snake_movement(&mut snake, enlarge_snake);
                enlarge_snake = false;
                time_since_last_move = 0.0;
            }
        }

        draw_game(&mut draw_context, &snake_head_texture, &snake_corner_texture, &snake_middle_texture, &apple_texture, &snake, &apple, play_area);        

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

fn draw_game(
    draw_context:           &mut RaylibDrawHandle, 
    snake_head_texture:     &Texture2D, 
    snake_corner_texture:   &Texture2D, 
    snake_middle_texture:   &Texture2D, 
    apple_texture:          &Texture2D,
    snake:                  &Snake,
    apple:                  &Apple, 
    play_area:              Rectangle) {

    draw_context.clear_background(Color::WHITE);

    draw_context.draw_rectangle_lines_ex(play_area, 1.0, Color::BLACK);

    for snake_part in &snake.parts {
        draw_context.draw_rectangle_rounded(
            Rectangle {
                x: snake_part.x, 
                y: snake_part.y, 
                width:  20.0, 
                height: 20.0
            }, 0.5, 10, Color::GRAY
        );
    }

    draw_context.draw_texture(
        apple_texture, apple.x as i32, apple.y as i32, Color::WHITE
    );
}

fn create_starting_snake() -> Snake {
    let mut snake_parts: Vec<SnakePart> = Vec::new();
    for _ in 0..STARTING_SNAKE_LENGTH {
        snake_parts.push(SnakePart {
            x: 100.0,
            y: 120.0,
        });
    }
    snake_parts.push(SnakePart {
        x: 120.0,
        y: 120.0,
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
            y: raylib_handle.get_random_value::<i32>(bounds.y as i32..(bounds.y+bounds.height) as i32) as f32
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
    }
}