use tcod::colors::*;
use tcod::console::*;
use tcod::input::Key;
use tcod::input::KeyCode::*;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

const LIMIT_FPS: i32 = 30; // 30 Frames per Second Limit

fn handle_keys(root: &mut Root, player_x: &mut i32, player_y: &mut i32) -> bool {
    let key = root.wait_for_keypress(true);
    match key {
        Key {
            code: Enter,
            alt: true,
            ..
        } => {
            // Alt+Enter: toggle fullscreen
            let fullscreen = root.is_fullscreen();
            root.set_fullscreen(!fullscreen);
        }
        Key { code: Escape, .. } => return true, // exit game

        // movement keys
        Key { code: Up, .. } => *player_y -= 1,
        Key { code: Down, .. } => *player_y += 1,
        Key { code: Left, .. } => *player_x -= 1,
        Key { code: Right, .. } => *player_x += 1,
        _ => {} // default
    }
    false
}

fn main() {
    let mut root = Root::initializer()
        .font("arial12x12.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Roguelike")
        .init();

    tcod::system::set_fps(LIMIT_FPS);

    let mut player_x = SCREEN_WIDTH / 2;
    let mut player_y = SCREEN_HEIGHT / 2;

    while !root.window_closed() {
        root.set_default_foreground(WHITE);
        root.clear();
        root.put_char(player_x, player_y, '@', BackgroundFlag::None);
        root.flush();
        root.wait_for_keypress(true);

        // handle keys and exit game if needed
        let exit = handle_keys(&mut root, &mut player_x, &mut player_y);
        if exit {
            break;
        }
    }
}
