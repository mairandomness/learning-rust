extern crate tcod;

use tcod::console::*;
use tcod::colors;
use tcod::input::Key;
use tcod::input::KeyCode::*;

// screen size
const SCREEN_WIDTH: i32 = 69;
const SCREEN_HEIGHT: i32 = 45;
// screen limit
const WIDTH_MAX: i32 = SCREEN_WIDTH - 5;
const WIDTH_MIN: i32 = 5;
// number of enemies
const ENEMY_LINES: usize = 5;
const ENEMY_ROWS: usize = 13;
// number of frames to wait after moving/attacking
const ENEMY_SPEED: i32 = 60;
const PLAYER_SPEED: i32 = 2;
const LIMIT_FPS: i32 = 60;
// the line the player is in
const PLAYER_Y: i32 = SCREEN_HEIGHT - 8;

fn main() {
    let mut root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Underscore invaders")
        .init();
    tcod::system::set_fps(LIMIT_FPS);

    let mut con = Offscreen::new(SCREEN_WIDTH, SCREEN_HEIGHT);
    let score = 0;
    let mut player = SCREEN_WIDTH / 2;

    let mut enemies = [[(0, 0, false); ENEMY_ROWS]; ENEMY_LINES];
    enemies = *start_enemies(&mut enemies);
    let mut direction = 1;

    let mut enemy_wait = ENEMY_SPEED;
    let mut player_wait = PLAYER_SPEED;

    while !root.window_closed() {

        // draw
        draw_objects(&mut con, &enemies, player);
        draw_canvas(&mut con, score);
        blit(&mut con, (0, 0), (SCREEN_WIDTH, SCREEN_HEIGHT), &mut root, (0, 0), 1.0, 1.0);
        root.flush();

        // erase objects before they move
        erase(&mut con, &enemies, player);

        // move enemies
        if enemy_wait > 0 {
            enemy_wait -= 1;
        } else {
            move_enemies(&mut enemies, &mut direction);
            enemy_wait = ENEMY_SPEED;
        }
        // handle keys
        if player_wait > 0 {
            player_wait -= 1;
        } else {
            let exit = handle_keys(&mut root, &mut player);
            if exit {
                break
            }
        }
    }
}

fn handle_keys(root: &mut Root, player: &mut i32) -> bool {
    let option_key = root.check_for_keypress(tcod::input::KeyPressFlags::all());
    match option_key {
        Some(key) => {
        match key {
            Key { code: Left, ..}  =>
                if player > &mut WIDTH_MIN {*player -= 1},
            Key { code: Right, .. } =>
                if player < &mut WIDTH_MAX {*player += 1},
            Key { code: Escape, .. } => return true,

            _ => {},
        }},
        None => {},
    }

    false
}

fn start_enemies(enemies: &mut [[(i32, i32, bool); ENEMY_ROWS]; ENEMY_LINES])
    -> &mut [[(i32, i32, bool); ENEMY_ROWS]; ENEMY_LINES] {

    let mut y = 3;
    let mut x: i32;
    let x_start = (SCREEN_WIDTH - (ENEMY_ROWS as i32) * 2 + 1) / 2 ;

    for i in 0..ENEMY_LINES {
        y += 2;
        x = x_start;
        for j in 0..ENEMY_ROWS {
            enemies[i][j] = (x, y, true);
            x += 2;
        }
    }
    enemies
}

fn move_enemies(enemies: &mut [[(i32, i32, bool); ENEMY_ROWS]; ENEMY_LINES], direction: &mut i32) {

    for i in 0..ENEMY_LINES {
        for j in 0..ENEMY_ROWS {
            enemies[i][j].0 += *direction;
        }
    }

    if enemies[0][0].0 <= WIDTH_MIN ||
        enemies[0][ENEMY_ROWS - 1].0 >= WIDTH_MAX {
            *direction *= -1;
    }
}


fn draw_canvas(con: &mut Offscreen, score: i32) {
    con.set_default_foreground(colors::WHITE);
    con.print(SCREEN_WIDTH - 15, 2, format!("SCORE: {:03}00", score));
    for i in WIDTH_MIN..(WIDTH_MAX + 1) {
        con.put_char( i, PLAYER_Y + 1, '_', BackgroundFlag::None);
    }
}



fn draw_objects(con: &mut Offscreen,
        enemies: &[[(i32, i32, bool); ENEMY_ROWS]; ENEMY_LINES], player: i32) {

    con.set_default_foreground(colors::GREEN);
    con.put_char(player, PLAYER_Y, 'W', BackgroundFlag::None);
    for i in 0..ENEMY_LINES {
        for j in 0..ENEMY_ROWS {
            match enemies[i][j] {
                p if p.2 == true => con.put_char(p.0, p.1, '_', BackgroundFlag::None),
                p => con.put_char(p.0, p.1, ' ', BackgroundFlag::None),
            };
        }
    }
}

fn erase(con: &mut Offscreen,
         enemies: &[[(i32, i32, bool); ENEMY_ROWS]; ENEMY_LINES], player: i32) {

    con.put_char(player, PLAYER_Y, ' ', BackgroundFlag::None);
    for i in 0..ENEMY_LINES {
        for j in 0..ENEMY_ROWS {
            con.put_char(enemies[i][j].0, enemies[i][j].1, ' ', BackgroundFlag::None);
        }
    }
}

