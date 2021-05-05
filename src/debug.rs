use crossterm::{
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    ExecutableCommand,
};
use std::fmt::Display;
use std::io::stdout;

pub struct Debug {}

impl Debug {
    // Log a message of type T to the screen of a specific color
    fn log_colored<T>(&self, message: T, color: Color)
    where
        T: Display,
    {
        match stdout().execute(SetBackgroundColor(Color::Black)) {
            Ok(_) => {}
            Err(err) => println!("{}", err),
        };
        match stdout().execute(SetForegroundColor(color)) {
            Ok(_) => {}
            Err(err) => println!("{}", err),
        };
        match stdout().execute(Print(message)) {
            Ok(_) => {}
            Err(err) => println!("{}", err),
        };
        match stdout().execute(Print("\n")) {
            Ok(_) => {}
            Err(err) => println!("{}", err),
        };
        match stdout().execute(ResetColor) {
            Ok(_) => {}
            Err(err) => println!("{}", err),
        };
    }

    // Log a message of type T to the screen
    // Note that type T must implement Display trait to print
    pub fn log<T>(&self, message: T)
    where
        T: Display,
    {
        self.log_colored(message, Color::White);
    }

    // Log a warning message of type T to the screen
    // Note that type T must implement Display trait to print
    pub fn log_warning<T>(&self, message: T)
    where
        T: std::fmt::Display,
    {
        self.log_colored(message, Color::Yellow);
    }

    // Log a error message of type T to the screen
    // Note that type T must implement Display trait to print
    pub fn log_error<T>(&self, message: T)
    where
        T: std::fmt::Display,
    {
        self.log_colored(message, Color::Red);
    }

    // Log a error message of type T to the screen
    // Note that type T must implement Display trait to print
    pub fn log_success<T>(&self, message: T)
    where
        T: std::fmt::Display,
    {
        self.log_colored(message, Color::Green);
    }
}
