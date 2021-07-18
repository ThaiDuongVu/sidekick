use sidekick::app::App;
use sidekick::entities::line::Line;
use sidekick::entities::rectangle::Rectangle;
use sidekick::input::Key;
use sidekick::types::vector2::Vector2;

use rand::Rng;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

const PLAYER_SIZE: Vector2 = Vector2 { x: 20., y: 80. };
const PLAYER_INIT_POSITION: Vector2 = Vector2 { x: 350., y: 0. };
const PLAYER_BOUNDED: bool = true;
const BALL_SPEED: f32 = 0.5;

fn main() {
    // Create a sidekick app
    let app = App::new();

    // Player 1: Left
    let mut player_1 = Rectangle::new();
    player_1.game_object.transform.position = -PLAYER_INIT_POSITION;
    player_1.game_object.transform.size = PLAYER_SIZE;
    player_1.game_object.is_bounded = PLAYER_BOUNDED;
    let mut player_1_score: usize = 0;

    // Player 2: Right
    let mut player_2 = Rectangle::new();
    player_2.game_object.transform.position = PLAYER_INIT_POSITION;
    player_2.game_object.transform.size = PLAYER_SIZE;
    player_2.game_object.is_bounded = PLAYER_BOUNDED;
    let mut player_2_score: usize = 0;

    // Bouncy ball
    let mut ball = Rectangle::new();
    ball.game_object.transform.size = Vector2 { x: 20., y: 20. };
    let mut ball_movement = Vector2::left();

    // Divider between 2 sides
    let mut divider = Line::new();
    divider.stroke_size = 5.;
    divider.game_object.transform.radius = app.height() as f32 / 2.;

    // Initialize app before first frame update
    // Note: Dynamic environment should be initialized outside of init
    let init = move |app: &mut App| {
        app.set_title("Pong");
        app.set_size(WIDTH, HEIGHT);
    };
    // Update and render game objects every frame
    let update = move |app: &mut App| {
        if app.input.on_key_down(Key::Esc) {
            app.quit();
        }

        // Players move based on keyboard input
        player_1.game_object.r#move(Vector2 {
            x: 0.,
            y: -(app.input.is_key_down(Key::S) as i32 as f32)
                + (app.input.is_key_down(Key::W) as i32 as f32),
        });
        player_2.game_object.r#move(Vector2 {
            x: 0.,
            y: -(app.input.is_key_down(Key::K) as i32 as f32)
                + (app.input.is_key_down(Key::I) as i32 as f32),
        });

        // Ball bounce around
        ball.game_object.r#move(ball_movement);

        // Ball colliding with vertical walls
        if ball.game_object.transform.position.x + ball.game_object.transform.size.x / 2.
            >= WIDTH as f32 / 2.
        {
            // Reset ball position & movement
            ball.game_object.transform.position = Vector2::zero();
            ball_movement.x = -ball_movement.x;
            ball_movement.y = rand::thread_rng().gen_range(-BALL_SPEED..BALL_SPEED);

            // Increment player score
            player_1_score += 1;
            println!("{}, {}", player_1_score, player_2_score);
        } else if ball.game_object.transform.position.x - ball.game_object.transform.size.x / 2.
            <= -(WIDTH as f32) / 2.
        {
            // Resset ball position & movement
            ball.game_object.transform.position = Vector2::zero();
            ball_movement.x = -ball_movement.x;
            ball_movement.y = rand::thread_rng().gen_range(-BALL_SPEED..BALL_SPEED);

            // Increment player score
            player_2_score += 1;
            println!("{}, {}", player_1_score, player_2_score);
        }
        // Ball colliding with horizontal walls
        if ball.game_object.transform.position.y + ball.game_object.transform.size.y / 2.
            >= HEIGHT as f32 / 2.
            || ball.game_object.transform.position.y - ball.game_object.transform.size.y / 2.
                <= -(HEIGHT as f32) / 2.
        {
            ball_movement.y = -ball_movement.y;
        }

        // Bal colliding with players
        if ball.game_object.transform.position.x + ball.game_object.transform.size.x / 2.
            >= player_2.game_object.transform.position.x
                - player_2.game_object.transform.size.x / 2.
            && ball.game_object.transform.position.x - ball.game_object.transform.size.x / 2.
                <= player_2.game_object.transform.position.x
                    + player_2.game_object.transform.size.x / 2.
        {
            if ball_movement.x > 0. {
                if ball.game_object.transform.position.y + ball.game_object.transform.size.y / 2.
                    >= player_2.game_object.transform.position.y
                        - player_2.game_object.transform.size.y / 2.
                    && ball.game_object.transform.position.y
                        - ball.game_object.transform.size.y / 2.
                        <= player_2.game_object.transform.position.y
                            + player_2.game_object.transform.size.y / 2.
                {
                    // Reset ball movement
                    ball_movement.x = -ball_movement.x;
                    ball_movement.y = rand::thread_rng().gen_range(-BALL_SPEED..BALL_SPEED);

                    app.game_view.start_shaking(10., 0.5, 1.);
                }
            }
        } else if ball.game_object.transform.position.x + ball.game_object.transform.size.x / 2.
            >= player_1.game_object.transform.position.x
                - player_1.game_object.transform.size.x / 2.
            && ball.game_object.transform.position.x - ball.game_object.transform.size.x / 2.
                <= player_1.game_object.transform.position.x
                    + player_2.game_object.transform.size.x / 2.
        {
            if ball_movement.x < 0. {
                if ball.game_object.transform.position.y + ball.game_object.transform.size.y / 2.
                    >= player_1.game_object.transform.position.y
                        - player_1.game_object.transform.size.y / 2.
                    && ball.game_object.transform.position.y
                        - ball.game_object.transform.size.y / 2.
                        <= player_1.game_object.transform.position.y
                            + player_1.game_object.transform.size.y / 2.
                {
                    // Reset ball movement
                    ball_movement.x = -ball_movement.x;
                    ball_movement.y = rand::thread_rng().gen_range(-BALL_SPEED..BALL_SPEED);

                    app.game_view.start_shaking(10., 0.5, 1.);
                }
            }
        }

        divider.draw(app);
        player_1.draw(app);
        player_2.draw(app);
        ball.draw(app);
    };

    // Run app
    app.run(init, update);
}
