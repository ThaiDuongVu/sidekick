# sidekick

<img src="images/logo.png"/>

2D Vulkan game framework, written entirely in Rust.

---

### Table of Contents
- [About](#about)
- [Features](#features)
- [Goals and Non-goals](#goals-and-non-goals)
- [Contributing](#contributing)
- [Requirements](#requirements)
- [Getting Started](#getting-started)
- [Tutorials and Documentation](#tutorials-and-documentation)

---

### About

**sidekick** is a free and open-source game framework under the [MIT license](LICENSE) that allows developers to create stunning 2D video games, prototypes, desktop apps and everything in between. It uses [glutin](https://github.com/rust-windowing/glutin) for window context management and [rgx](https://github.com/cloudhead/rgx) as the rendering engine.

**sidekick** is subject to change as it is actively developed so expect things to break (😅).

**sidekick** is an educational project for learning purposes, feel free to use it in personal and commercial projects, just keep in mind that support is minimal.

Supported platforms:
<img src="images/windows.png" width="60"> <img src="images/linux.png" width="60"> <img src="images/macos.png" width="60">

---

### Features

#### Current features
- Window Management
- Cursor Management
- Keyboard Input Handling
- Mouse Input Handling
- Game Time Management
- Debugging Tools
- Basic Shapes Rendering (Line, Triangle, Circle)

#### Planned features
- Custom Math & Logic Library
- Gamepad Input Handling
- Custom Polygonal Shape Rendering
- Text Rendering
- Sprite Rendering
- Audio Playing
- Animations
- 2D Physics
- 2D Lighting & Shadow
- Profiling Tools
- Custom Test Framework
- And more to be added...

You can always refer to the [sidekick project board](https://github.com/ThaiDuongVu/sidekick/projects/1) for more details on features being implement.

---

### Goals and Non-goals

#### Goals (Order from most important)
- Intuitive and Accessible.
- Fast and Easy to get started.
- Performance.
- Cross-platform.

#### Non-goals (some of these might get implemented if heavily requested 🤔)
- 3D Rendering.
- AAA-level Graphics.
- GUI Editor.

---

### Contributing

**sidekick** is completely open to community contribution. Any effort to help expand the scope and capability of the framework is greatly appreciated 🤗.

Please make sure that you follow [the Rust API Guidelines](https://rust-lang.github.io/api-guidelines/) when making a pull request, just to make the codebase clear and consistent 😊.

---

### Requirements

- Rust compiler and Cargo package manager, you can download them [here](https://www.rust-lang.org/).
- A code editor of your choice.

---

### Getting Started

To create a basic sidekick app:

1. Create a new project with Cargo using `cargo new project-name --bin`.

2. In your `Cargo.toml`, add:

```rust
        [dependencies]
        sidekick = { git = "https://github.com/ThaiDuongVu/sidekick" }
```

3. In your `main.rs`, add:
```rust
        use sidekick::app::App;

        fn main() {
            // Create a sidekick app
            let app = App::new();
            
            // Initialize app before first frame update
            // Note: Dynamic environment should be initialized outside of init
            let init = move |app: &mut App| {};
            
            // Update and render game objects every frame
            let update = move |app: &mut App| {};

            // Run app
            app.run(init, update);
        }
```

4. Build and run the project with `cargo run`.

5. You should see the following window by default: 
<img src="./images/window.png" width=600/>

---

### Tutorials and Documentation

You can refer to the [wiki page](https://github.com/ThaiDuongVu/sidekick/wiki) (whenever it's live 🤷‍♂️) for an in-depth guide to **sidekick**.

There are a handful of [examples](./examples) to learn from. To run them simply run the command `cargo run --example example-name` where `example-name` is, well, the name of the example to run.

---

### Want a feature that's currently not in sidekick's roadmap?

Well first of all you can implement it yourself (the power of open source 😏).

You can also go to the [issues tab](https://github.com/ThaiDuongVu/sidekick/issues) and submit a new issue (remember to add the "feature request" label).

