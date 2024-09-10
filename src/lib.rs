pub trait Colorize {
    // colors
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
    // background colors
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
    // custom colors
    fn fg_rgb(&self, r: u8, g: u8, b: u8) -> String;
    fn bg_rgb(&self, r: u8, g: u8, b: u8) -> String;
    fn fg_hex(&self, hex_color: &str) -> String;
    fn bg_hex(&self, hex_color: &str) -> String;
    // gradient
    fn fg_gradient(&self, from: (u8, u8, u8), to: (u8, u8, u8)) -> String;
    fn bg_gradient(&self, from: (u8, u8, u8), to: (u8, u8, u8)) -> String;
    fn fg_hex_gradient(&self, from: &str, to: &str) -> String;
    fn bg_hex_gradient(&self, from: &str, to: &str) -> String;
    // other
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
    // remove
    fn remove_weight(&self) -> String;
    fn remove_underline(&self) -> String;
    fn remove_blink(&self) -> String;
    fn reveal(&self) -> String;
}

impl<T> Colorize for T
where
    T: std::fmt::Display,
{
    fn black(&self) -> String {
        format!("\x1B[30m{}\x1B[0m", self)
    }

    fn gray(&self) -> String {
        format!("\x1B[90m{}\x1B[0m", self)
    }

    fn red(&self) -> String {
        format!("\x1B[31m{}\x1B[0m", self)
    }

    fn bright_red(&self) -> String {
        format!("\x1B[91m{}\x1B[0m", self)
    }

    fn blue(&self) -> String {
        format!("\x1B[34m{}\x1B[0m", self)
    }

    fn bright_blue(&self) -> String {
        format!("\x1B[94m{}\x1B[0m", self)
    }

    fn green(&self) -> String {
        format!("\x1B[32m{}\x1B[0m", self)
    }

    fn bright_green(&self) -> String {
        format!("\x1B[92m{}\x1B[0m", self)
    }

    fn yellow(&self) -> String {
        format!("\x1B[33m{}\x1B[0m", self)
    }

    fn bright_yellow(&self) -> String {
        format!("\x1B[93m{}\x1B[0m", self)
    }

    fn magenta(&self) -> String {
        format!("\x1B[35m{}\x1B[0m", self)
    }

    fn bright_magenta(&self) -> String {
        format!("\x1B[95m{}\x1B[0m", self)
    }

    fn cyan(&self) -> String {
        format!("\x1B[36m{}\x1B[0m", self)
    }

    fn bright_cyan(&self) -> String {
        format!("\x1B[96m{}\x1B[0m", self)
    }

    fn white(&self) -> String {
        format!("\x1B[37m{}\x1B[0m", self)
    }

    fn bright_white(&self) -> String {
        format!("\x1B[97m{}\x1B[0m", self)
    }

    fn bg_black(&self) -> String {
        format!("\x1B[40m{}\x1B[0m", self)
    }

    fn bg_gray(&self) -> String {
        format!("\x1B[100m{}\x1B[0m", self)
    }

    fn bg_red(&self) -> String {
        format!("\x1B[41m{}\x1B[0m", self)
    }

    fn bg_bright_red(&self) -> String {
        format!("\x1B[101m{}\x1B[0m", self)
    }

    fn bg_green(&self) -> String {
        format!("\x1B[42m{}\x1B[0m", self)
    }

    fn bg_bright_green(&self) -> String {
        format!("\x1B[102m{}\x1B[0m", self)
    }

    fn bg_yellow(&self) -> String {
        format!("\x1B[43m{}\x1B[0m", self)
    }

    fn bg_bright_yellow(&self) -> String {
        format!("\x1B[103m{}\x1B[0m", self)
    }

    fn bg_blue(&self) -> String {
        format!("\x1B[44m{}\x1B[0m", self)
    }

    fn bg_bright_blue(&self) -> String {
        format!("\x1B[104m{}\x1B[0m", self)
    }

    fn bg_magenta(&self) -> String {
        format!("\x1B[45m{}\x1B[0m", self)
    }

    fn bg_bright_magenta(&self) -> String {
        format!("\x1B[105m{}\x1B[0m", self)
    }

    fn bg_cyan(&self) -> String {
        format!("\x1B[46m{}\x1B[0m", self)
    }

    fn bg_bright_cyan(&self) -> String {
        format!("\x1B[106m{}\x1B[0m", self)
    }

    fn bg_white(&self) -> String {
        format!("\x1B[47m{}\x1B[0m", self)
    }

    fn bg_bright_white(&self) -> String {
        format!("\x1B[107m{}\x1B[0m", self)
    }

    fn clear(&self) -> String {
        format!("\x1B[0m{}", self)
    }

    fn bold(&self) -> String {
        format!("\x1B[1m{}\x1B[22m\x1B[0m", self)
    }

    fn faint(&self) -> String {
        format!("\x1B[2m{}\x1B[22m\x1B[0m", self)
    }

    fn italic(&self) -> String {
        format!("\x1B[3m{}\x1B[23m\x1B[0m", self)
    }

    fn underline(&self) -> String {
        format!("\x1B[4m{}\x1B[24m\x1B[0m", self)
    }

    fn blink(&self) -> String {
        format!("\x1B[5m{}\x1B[25m\x1B[0m", self)
    }

    fn invert(&self) -> String {
        format!("\x1B[7m{}\x1B[27m\x1B[0m", self)
    }

    fn hide(&self) -> String {
        format!("\x1B[8m{}\x1B[28m\x1B[0m", self)
    }

    fn reveal(&self) -> String {
        self.hide()
    }

    fn remove_weight(&self) -> String {
        format!("\x1B[22m{}", self)
    }

    fn remove_underline(&self) -> String {
        format!("\x1B[24m{}", self)
    }

    fn remove_blink(&self) -> String {
        format!("\x1B[25m{}", self)
    }

    fn fg_default(&self) -> String {
        // todo
        format!("\x1B[39m{}", self)
    }

    fn bg_default(&self) -> String {
        // todo
        format!("\x1B[49m{}", self)
    }

    fn overline(&self) -> String {
        // todo
        format!("\x1B[53m{}\x1B[55m", self)
    }

    fn fg_rgb(&self, r: u8, g: u8, b: u8) -> String {
        format!("\x1B[38;2;{};{};{}m{}\x1B[39m", r, g, b, self)
    }

    fn bg_rgb(&self, r: u8, g: u8, b: u8) -> String {
        format!("\x1B[48;2;{};{};{}m{}\x1B[49m", r, g, b, self)
    }

    fn fg_hex(&self, hex: &str) -> String {
        let (r, g, b) = hex_to_rgb(hex).unwrap_or((0, 0, 0));
        format!("\x1B[38;2;{};{};{}m{}\x1B[39m", r, g, b, self)
    }
    fn bg_hex(&self, hex: &str) -> String {
        let (r, g, b) = hex_to_rgb(hex).unwrap_or((0, 0, 0));
        format!("\x1B[48;2;{};{};{}m{}\x1B[49m", r, g, b, self)
    }

    fn fg_gradient(&self, from: (u8, u8, u8), to: (u8, u8, u8)) -> String {
        let mut result = String::new();
        let steps = self.to_string().len();
        let mut colors = vec![];
        for i in 0..steps {
            let r = from.0 as i32 + i as i32 * (to.0 as i32 - from.0 as i32) / (steps - 1) as i32;
            let g = from.1 as i32 + i as i32 * (to.1 as i32 - from.1 as i32) / (steps - 1) as i32;
            let b = from.2 as i32 + i as i32 * (to.2 as i32 - from.2 as i32) / (steps - 1) as i32;

            colors.push((r as u8, g as u8, b as u8));
        }
        for (i, c) in self.to_string().chars().enumerate() {
            let (r, g, b) = colors.get(i).unwrap();
            result.push_str(&c.fg_rgb(*r, *g, *b));
        }
        result
    }

    fn bg_gradient(&self, from: (u8, u8, u8), to: (u8, u8, u8)) -> String {
        let mut result = String::new();
        let steps = self.to_string().len();
        let mut colors = vec![];
        for i in 0..steps {
            let r = from.0 as i32 + i as i32 * (to.0 as i32 - from.0 as i32) / (steps - 1) as i32;
            let g = from.1 as i32 + i as i32 * (to.1 as i32 - from.1 as i32) / (steps - 1) as i32;
            let b = from.2 as i32 + i as i32 * (to.2 as i32 - from.2 as i32) / (steps - 1) as i32;

            colors.push((r as u8, g as u8, b as u8));
        }
        for (i, c) in self.to_string().chars().enumerate() {
            let (r, g, b) = colors.get(i).unwrap();
            result.push_str(&c.bg_rgb(*r, *g, *b));
        }
        result
    }

    fn fg_hex_gradient(&self, from: &str, to: &str) -> String {
        let from = hex_to_rgb(from).unwrap();
        let to = hex_to_rgb(to).unwrap();
        let mut result = String::new();
        let steps = self.to_string().len();
        let mut colors = vec![];
        for i in 0..steps {
            let r = from.0 as i32 + i as i32 * (to.0 as i32 - from.0 as i32) / (steps - 1) as i32;
            let g = from.1 as i32 + i as i32 * (to.1 as i32 - from.1 as i32) / (steps - 1) as i32;
            let b = from.2 as i32 + i as i32 * (to.2 as i32 - from.2 as i32) / (steps - 1) as i32;

            colors.push((r as u8, g as u8, b as u8));
        }
        for (i, c) in self.to_string().chars().enumerate() {
            let (r, g, b) = colors.get(i).unwrap();
            result.push_str(&c.fg_rgb(*r, *g, *b));
        }
        result
    }

    fn bg_hex_gradient(&self, from: &str, to: &str) -> String {
        let from = hex_to_rgb(from).unwrap();
        let to = hex_to_rgb(to).unwrap();
        let mut result = String::new();
        let steps = self.to_string().len();
        let mut colors = vec![];
        for i in 0..steps {
            let r = from.0 as i32 + i as i32 * (to.0 as i32 - from.0 as i32) / (steps - 1) as i32;
            let g = from.1 as i32 + i as i32 * (to.1 as i32 - from.1 as i32) / (steps - 1) as i32;
            let b = from.2 as i32 + i as i32 * (to.2 as i32 - from.2 as i32) / (steps - 1) as i32;

            colors.push((r as u8, g as u8, b as u8));
        }
        for (i, c) in self.to_string().chars().enumerate() {
            let (r, g, b) = colors.get(i).unwrap();
            result.push_str(&c.bg_rgb(*r, *g, *b));
        }
        result
    }
}

fn hex_to_rgb(hex_color: &str) -> Option<(u8, u8, u8)> {
    let hex = hex_color.trim_start_matches('#');
    if hex.len() != 6 {
        return None;
    }

    let mut rgb = [0u8; 3];
    for i in 0..3 {
        match u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16) {
            Ok(val) => rgb[i] = val,
            Err(_) => return None,
        }
    }

    Some((rgb[0], rgb[1], rgb[2]))
}
