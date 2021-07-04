use sidekick::app::App;
use sidekick::entities::rectangle::Rectangle;
use sidekick::entities::line::Line;
use sidekick::input::Key;
use sidekick::types::vector2::Vector2;

const PLAYER_SIZE: Vector2 = Vector2 { x: 20., y: 80. };
const PLAYER_INIT_POSITION: Vector2 = Vector2 { x: 350., y: 0. };
const PLAYER_BOUNDED: bool = true;

fn main() {
    // Create a sidekick app
    let app = App::new();

    // Player 1: Left
    let mut player_1 = Rectangle::new();
    player_1.game_object.transform.position = -PLAYER_INIT_POSITION;
    player_1.game_object.transform.size = PLAYER_SIZE;
    player_1.game_object.is_bounded = PLAYER_BOUNDED;

    // Player 2: Right
    let mut player_2 = Rectangle::new();
    player_2.game_object.transform.position = PLAYER_INIT_POSITION;
    player_2.game_object.transform.size = PLAYER_SIZE;
    player_2.game_object.is_bounded = PLAYER_BOUNDED;

    // Divider between 2 sides
    let mut divider = Line::new();
    divider.stroke_size = 5.;
    divider.game_object.transform.radius = app.height() as f32 / 2.;

    // Changes to dynamic environment values will not be saved if done in init
    let init = move |app: &mut App| {
        app.set_title("Pong");
    };
    let update = move |app: &mut App| {
        if app.input.on_key_down(Key::Esc) {
            app.quit();
        }

        player_1.game_object.r#move(Vector2 {
            x: 0.,
            y: -(app.input.is_key_down(Key::S) as i32 as f32)
                + (app.input.is_key_down(Key::W) as i32 as f32),
        });
        player_2.game_object.r#move(Vector2 {
            x: 0.,
            y: -(app.input.is_key_down(Key::Down) as i32 as f32)
                + (app.input.is_key_down(Key::Up) as i32 as f32),
        });

        player_1.draw(app);
        player_2.draw(app);
        divider.draw(app);
    };

    // Run app
    app.run(init, update);
}
