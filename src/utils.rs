use rand::Rng;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub fn set_random_colour() {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout
        .set_color(ColorSpec::new().set_fg(Some(random_colour())))
        .unwrap();
}

pub fn random_colour() -> Color {
    let colours = [
        Color::White,
        Color::Green,
        Color::Red,
        Color::Black,
        Color::Blue,
        Color::Cyan,
        Color::Magenta,
        Color::Yellow,
    ];

    let mut rng = rand::thread_rng();

    let random_index = rng.gen_range(0..colours.len());

    colours[random_index]
}

pub fn set_default_colour() {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    stdout
        .set_color(ColorSpec::new().set_fg(Some(Color::White)))
        .unwrap();
}
