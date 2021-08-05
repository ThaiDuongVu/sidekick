use sidekick::app::App;
use sidekick::input::MouseButton;

fn main() {
    // Create a sidekick app
    let app = App::new();

    // Initialize app before first frame update
    // Note: Dynamic environment should be initialized outside of init
    let init = move |_app: &mut App| {};

    // Update and render game objects every frame
    let update = move |app: &mut App| {
        // is_mouse_button_down returns true while a mouse button is being held down
        if app.input.is_mouse_button_down(MouseButton::Left) {
            println!("Left mouse button down");
        }
        // is_mouse_button_up returns true while a mouse button is NOT held down
        if app.input.is_mouse_button_up(MouseButton::Left) {
            // I comment the line below out because I don't want your terminal flooded with "LEFT MOUSE BUTTON UP" :) feel free to uncomment to see it works
            // println!("Left mouse button up");
        }

        // on_mouse_button_down returns true for the first frame a mouse button is pressed
        if app.input.on_mouse_button_down(MouseButton::Right) {
            println!("Right mouse button pressed");
        }
        // on_mouse_button_up returns true for the first frame a mouse button is released
        if app.input.on_mouse_button_up(MouseButton::Right) {
            println!("Right mouse button released");
        }

        if app.input.on_mouse_button_down(MouseButton::Middle) {
            // mouse_position returns current x and y position of mouse cursor relative to app window as a Vector2
            println!("{}", app.input.mouse_position());
            // is_mouse_entered returns whether cursor is hovering over current app window
            println!("{}", app.input.is_mouse_entered());
        }
    };

    // Run app
    app.run(init, update);
}
