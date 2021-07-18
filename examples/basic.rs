use sidekick::app::App;

fn main() {
    // Create a sidekick app
    let app = App::new();
    
    // Initialize app before first frame update
    // Note: Dynamic environment should be initialized outside of init
    let init = move |_app: &mut App| {};
    
    // Update and render game objects every frame
    let update = move |_app: &mut App| {};

    // Run app
    app.run(init, update);
}
