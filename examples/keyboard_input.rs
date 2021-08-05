use sidekick::app::App;
use sidekick::input::Key;

fn main() {
    // Create a sidekick app
    let app = App::new();

    // Initialize app before first frame update
    // Note: Dynamic environment should be initialized outside of init
    let init = move |_app: &mut App| {};

    // Update and render game objects every frame
    let update = move |app: &mut App| {
        // is_key_down returns true while a keyboard key is being held down
        if app.input.is_key_down(Key::Space) {
            println!("Space key down");
        }
        // is_key_up returns true while a keyboard key is NOT being held down
        if app.input.is_key_up(Key::Enter) {
            // I comment the line below out because I don't want your terminal flooded with "LEFT MOUSE BUTTON UP" :) feel free to uncomment to see it works
            // println!("Enter key up");
        }

        // on_key_down returns true for the first frame a key is pressed
        if app.input.on_key_down(Key::Esc) {
            println!("Esc key pressed");
        }
        // on_key_up returns true for the first frame a key is released
        if app.input.on_key_up(Key::Enter) {
            println!("Enter key released");
        }

        // get_axis_horizontal returns sum of horizontal input keys (AD and Left Right arrows)
        // println!("Horizontal: {}", app.input.get_axis_horizontal());
        // get_axis_vertical returns sum of vertical input keys (WS and Up Down arrows)
        // println!("Vertical: {}", app.input.get_axis_vertical());
    };

    // Run app
    app.run(init, update);
}
