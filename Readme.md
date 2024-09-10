# Colored++

Lighweight crate for coloring and animating texts on the terminal. Supports almost every ANSII encoding colors and plus more.

## installation

```sh
cargo add coloredpp
```

## usage

```rs
use coloredpp::Colorize;

fn main() {
    println!("{}", "hi".red());
}
```

## methods

Colorize methods apply to anything with `Display` trait implemented, including `String`, `str` and `char`.

### colors

```rs
fn black(&self) -> String;
fn gray(&self) -> String;
fn red(&self) -> String;
fn bright_red(&self) -> String;
fn blue(&self) -> String;
fn bright_blue(&self) -> String;
fn green(&self) -> String;
fn bright_green(&self) -> String;
fn yellow(&self) -> String;
fn bright_yellow(&self) -> String;
fn magenta(&self) -> String;
fn bright_magenta(&self) -> String;
fn cyan(&self) -> String;
fn bright_cyan(&self) -> String;
fn white(&self) -> String;
fn bright_white(&self) -> String;
```

### background colors

```rust
fn bg_black(&self) -> String;
fn bg_gray(&self) -> String;
fn bg_red(&self) -> String;
fn bg_bright_red(&self) -> String;
fn bg_green(&self) -> String;
fn bg_bright_green(&self) -> String;
fn bg_yellow(&self) -> String;
fn bg_bright_yellow(&self) -> String;
fn bg_blue(&self) -> String;
fn bg_bright_blue(&self) -> String;
fn bg_magenta(&self) -> String;
fn bg_bright_magenta(&self) -> String;
fn bg_cyan(&self) -> String;
fn bg_bright_cyan(&self) -> String;
fn bg_white(&self) -> String;
fn bg_bright_white(&self) -> String;
```

### custom colors

```rust
fn fg_rgb(&self, r: u8, g: u8, b: u8) -> String;
fn bg_rgb(&self, r: u8, g: u8, b: u8) -> String;
fn fg_hex(&self, hex_color: &str) -> String;
fn bg_hex(&self, hex_color: &str) -> String;
```

### gradient colors

```rust
fn fg_gradient(&self, from: (u8, u8, u8), to: (u8, u8, u8)) -> String;
fn bg_gradient(&self, from: (u8, u8, u8), to: (u8, u8, u8)) -> String;
fn fg_hex_gradient(&self, from: &str, to: &str) -> String;
fn bg_hex_gradient(&self, from: &str, to: &str) -> String;
```

### removers

```rust
fn remove_weight(&self) -> String;
fn remove_underline(&self) -> String;
fn remove_blink(&self) -> String;
fn reveal(&self) -> String;
```

### other

```rust
fn clear(&self) -> String;
fn bold(&self) -> String;
fn faint(&self) -> String;
fn italic(&self) -> String;
fn underline(&self) -> String;
fn blink(&self) -> String;
fn invert(&self) -> String;
fn hide(&self) -> String;
fn fg_default(&self) -> String;
fn bg_default(&self) -> String;
fn overline(&self) -> String;
```
