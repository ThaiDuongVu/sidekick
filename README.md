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

**sidekick** is a free and open-source game framework under the [MIT license](LICENSE) that allows developers to create stunning 2D video games, desktop apps and everything in between. It uses [glutin](https://github.com/rust-windowing/glutin) for window context management and [rgx](https://github.com/cloudhead/rgx) as the rendering engine.

**sidekick** supports Windows, Linux and macOS (although untested properly on macOS, sorry I don't have a mac 😓).

**sidekick** is subject to change as it is actively developed so expect things to break (😅).

**sidekick** is an educational project for learning purposes, feel free to use it in personal and commercial projects, just keep in mind that support is minimal.

---

### Features

#### Current features
- Window Management
- Cursor Management
- Keyboard Input Handling.
- Mouse Input Handling.
- Game Time Management.
- Debugging Tools.

#### Planned features
- Custom Math & Logic Library
- Gamepad Input Handling.
- Basic Shapes Rendering (Line, Triangle, Square).
- Text Rendering.
- Sprite Rendering.
- Audio Playing.
- Animations.
- 2D Physics.
- 2D Lighting & Shadow.
- Profiling Tools.
- Custom Test Framework.
- And more to be added...

---

### Goals and Non-goals

#### Goals (Order from most to least important)
- Intuitive and Accessible.
- Fast and Easy to get started.
- 100% Rust.
- Performance.
- Cross-platform.

#### Non-goals (some of these might get implemented if heavily requested 🤔)
- 3D Rendering.
- UI Editor.

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
            // Intialize before first frame update
            let init = move |_app: &mut App| {};
            // Update once every frame
            let update = move |_app: &mut App| {};
            // Render game objects
            let render = move |_app: &mut App| {};
            // Clean up before game exit
            let exit = move |_app: &mut App| {};

            let app = App::new();
            app.run(init, update, render, exit);
        }

```

4. Build and run the project with `cargo run`.

5. You should see the following window by default: 
<img src="./images/window.png" width=600/>

---

### Tutorials and Documentation

You can refer to the [wiki page](https://github.com/ThaiDuongVu/sidekick/wiki) (whenever it's live 🤷‍♂️) for an in-depth guide to **sidekick**.
