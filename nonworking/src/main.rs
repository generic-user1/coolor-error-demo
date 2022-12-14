fn main() {
    let coolor_color = coolor::Color::Rgb(coolor::Rgb {
        r: 10,
        g: 10,
        b: 10,
    });
    let crossterm_color: crossterm::style::Color = coolor_color.into();
    println!("color value: {:?}", crossterm_color);
}
