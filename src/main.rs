use clap::Parser;
use crossterm::QueueableCommand;
use crossterm::style::{Attribute, Color, SetAttribute};

#[derive(Parser, Debug)]
#[command()]
struct Args {
    /// If set, will ignore the `NO_COLOR` environment variable
    #[arg(long, short = 'i')]
    ignore_no_color: bool,

    /// If set, will not check if stdout is a TTY
    /// (for instance, it won't be if you're piping the output to another command).
    /// If not set, this program will exit right away if it detects that it's in a tty.
    #[arg(long, short = 't')]
    skip_tty_check: bool,

    /// List of commands. Everything here is case-insensitive.
    /// Commands can be separated by semicolons in an argument,
    /// or separated into multiple arguments
    ///
    /// List of commands:
    ///
    /// reset/norm/normal
    ///
    /// bold
    ///
    /// nobold/notbold Inconsistent, prefer normalintensity
    ///
    /// dim/faint
    ///
    /// italic Not widely supported, sometimes treated as inverse or blink
    ///
    /// noitalic/notitalic
    ///
    /// normalintensity (no bold and no italic)
    ///
    /// underline/underlined
    ///
    /// doubleunderline/doubleunderlined Sometimes disables bold intensity instead on some terminals
    ///
    /// undercurl/undercurled
    ///
    /// underdot/underdotted
    ///
    /// underdash/underdahsed
    ///
    /// nounderline/nounderlined/notunderline/notunderlined
    ///
    /// slowblink Not widely supported, sometimes treated as inverse
    ///
    /// rapidblink Not widely supported
    ///
    /// noblink/notblink
    ///
    /// reverse/invert/inverse
    ///
    /// hide/hidden/conceal/concealed Not widely supported
    ///
    /// nohide/nohidden/noconceal/noconcealed/nothide/nothidden/notconceal/notconcealed/reveal/revealed
    ///
    /// cross/crossed/crossout/crossedout
    ///
    /// nocross/nocrossed/nocrossout/nocrossedout/notcross/notcrossed/notcrossout/notcrossedout
    ///
    /// fraktur/gothic Rarely supported
    ///
    /// frame/framed Not widely supported
    ///
    /// encircle/encircled
    ///
    /// noframeorencircle/noframedorencircled/notframeorencircle/notframedorencircled
    ///
    /// overline/overlined
    ///
    /// nooverline/nooverlined/notoverline/notoverlined
    ///
    /// <name>      Same as fg=<name>
    ///
    /// <u8>        Same as fg=<u8>
    ///
    /// <r>,<g>,<b> Same as fg=<r>,<g>,<b>
    ///
    /// #RRGGBB     Same as fg=#RRGGBB
    ///
    /// fg=<name> Set the foreground color to the named color
    /// (see later in the help for color names)
    ///
    /// fg=<u8> Set the foreground color to u8 where u8 is an unsigned 8 bit integer representing an ANSI color code.
    /// A table with a list of the numbers can be found at https://web.archive.org/web/20250529201746/https://www.ditig.com/256-colors-cheat-sheet
    ///
    /// fg=<r>,<g>,<b> Set the foreground color to an RGB value
    ///
    /// fg=#RRGGBB Set the foreground color to an RGB hexadecimal value
    ///
    /// bg=<name> Set the background color to the named color.
    /// (see later in the help for color names)
    ///
    /// bg=<u8> Set the background color to u8 where u8 is an unsigned 8 bit integer representing an ANSI color code.
    /// A table with a list of the numbers can be found at https://web.archive.org/web/20250529201746/https://www.ditig.com/256-colors-cheat-sheet
    ///
    /// bg=<r>,<g>,<b> Set the background color to an RGB value
    ///
    /// bg=#RRGBB Set the background color to an RGB hexadecimal value
    ///
    ///
    ///
    /// For color names, like everything else in commands, case doesn't matter. You have your standard colors, sometimes also called "dark" color:
    ///
    /// black
    ///
    /// red
    ///
    /// green
    ///
    /// yellow
    ///
    /// blue
    ///
    /// magenta
    ///
    /// cyan
    ///
    /// white
    ///
    ///
    ///
    /// And then for each of those, a "bright" variant:
    ///
    /// brightblack
    ///
    /// brightred
    ///
    /// brightgreen
    ///
    /// brightyellow
    ///
    /// brightblue
    ///
    /// brightmagenta
    ///
    /// brightcyan
    ///
    /// brightwhite
    ///
    ///
    ///
    /// Along with a few convenience aliases:
    ///
    /// grey/gray -> brightblack
    ///
    /// maroon/brightmaroon -> red/brightred
    ///
    /// darklime/lime -> green/brightgreen
    ///
    /// olive/brightolive -> yellow/brightyellow
    ///
    /// navy/brightnavy -> blue/brightblue
    ///
    /// purple/brightpurple -> magenta/brightmagenta
    ///
    /// darkfuchsia/fuchsia -> magenta/brightmagenta
    ///
    /// teal/brightteal -> cyan/brightcyan
    ///
    /// darkaqua/aqua -> brightcyan
    ///
    /// silver -> white
    commands: Vec<String>,
}

fn main() {
    let args = Args::parse();
    let mut stdout = std::io::stdout().lock();
    if !args.skip_tty_check && !crossterm::tty::IsTty::is_tty(&stdout) {
        return;
    }
    if args.ignore_no_color {
        crossterm::style::Colored::set_ansi_color_disabled(false);
    }
    let commands: Vec<String> = args
        .commands
        .iter()
        .flat_map(|s| s.split(';'))
        .map(|s| s.to_lowercase())
        .collect();
    for command in commands {
        if let Ok(color) = parse_color(&command) {
            stdout
                .queue(crossterm::style::SetForegroundColor(color))
                .unwrap();
        } else if command.starts_with("fg") {
            if !command.starts_with("fg=") {
                panic!("= expected after fg")
            }
            let fg_color = command.split_once('=').expect("Should not fail since we just verified there's an =. If you're seeing this, something has gone quite wrong.").1;
            let fg_color = parse_color(fg_color).unwrap();
            stdout
                .queue(crossterm::style::SetForegroundColor(fg_color))
                .unwrap();
        } else if command.starts_with("bg") {
            if !command.starts_with("bg=") {
                panic!("= expected after bg")
            }
            let bg_color = command.split_once('=').expect("Should not fail since we just verified there's an =. If you're seeing this, something has gone quite wrong.").1;
            let bg_color = parse_color(bg_color).unwrap();
            stdout
                .queue(crossterm::style::SetBackgroundColor(bg_color))
                .unwrap();
        } else if command == "reset" || command == "norm" || command == "normal" {
            stdout.queue(SetAttribute(Attribute::Reset)).unwrap();
        } else if command == "bold" {
            stdout.queue(SetAttribute(Attribute::Bold)).unwrap();
        } else if command == "nobold" || command == "notbold" {
            stdout.queue(SetAttribute(Attribute::NoBold)).unwrap();
        } else if command == "dim" || command == "faint" {
            stdout.queue(SetAttribute(Attribute::Dim)).unwrap();
        } else if command == "italic" {
            stdout.queue(SetAttribute(Attribute::Italic)).unwrap();
        } else if command == "noitalic" || command == "notitalic" {
            stdout.queue(SetAttribute(Attribute::NoItalic)).unwrap();
        } else if command == "normalintensity" {
            stdout
                .queue(SetAttribute(Attribute::NormalIntensity))
                .unwrap();
        } else if command == "underline" || command == "underlined" {
            stdout.queue(SetAttribute(Attribute::Underlined)).unwrap();
        } else if command == "doubleunderline" || command == "doubleunderlined" {
            stdout
                .queue(SetAttribute(Attribute::DoubleUnderlined))
                .unwrap();
        } else if command == "undercurl" || command == "undercurled" {
            stdout.queue(SetAttribute(Attribute::Undercurled)).unwrap();
        } else if command == "underdot" || command == "underdotted" {
            stdout.queue(SetAttribute(Attribute::Underdotted)).unwrap();
        } else if command == "underdash" || command == "underdashed" {
            stdout.queue(SetAttribute(Attribute::Underdashed)).unwrap();
        } else if command == "nounderline"
            || command == "nounderlined"
            || command == "notunderline"
            || command == "notunderlined"
        {
            stdout.queue(SetAttribute(Attribute::NoUnderline)).unwrap();
        } else if command == "slowblink" {
            stdout.queue(SetAttribute(Attribute::SlowBlink)).unwrap();
        } else if command == "rapidblink" {
            stdout.queue(SetAttribute(Attribute::RapidBlink)).unwrap();
        } else if command == "noblink" || command == "notblink" {
            stdout.queue(SetAttribute(Attribute::NoBlink)).unwrap();
        } else if command == "reverse" || command == "invert" || command == "inverse" {
            stdout.queue(SetAttribute(Attribute::Reverse)).unwrap();
        } else if command == "hide"
            || command == "hidden"
            || command == "conceal"
            || command == "concealed"
        {
            stdout.queue(SetAttribute(Attribute::Hidden)).unwrap();
        } else if command == "nohide"
            || command == "nohidden"
            || command == "noconceal"
            || command == "noconcealed"
            || command == "nothide"
            || command == "nothidden"
            || command == "notconceal"
            || command == "notconcealed"
            || command == "reveal"
            || command == "revealed"
        {
            stdout.queue(SetAttribute(Attribute::NoHidden)).unwrap();
        } else if command == "cross"
            || command == "crossed"
            || command == "crossout"
            || command == "crossedout"
        {
            stdout.queue(SetAttribute(Attribute::CrossedOut)).unwrap();
        } else if command == "nocross"
            || command == "nocrossed"
            || command == "nocrossout"
            || command == "nocrossedout"
            || command == "notcross"
            || command == "notcrossed"
            || command == "notcrossout"
            || command == "notcrossedout"
        {
            stdout
                .queue(SetAttribute(Attribute::NotCrossedOut))
                .unwrap();
        } else if command == "fraktur" || command == "gothic" {
            stdout.queue(SetAttribute(Attribute::Fraktur)).unwrap();
        } else if command == "frame" || command == "framed" {
            stdout.queue(SetAttribute(Attribute::Framed)).unwrap();
        } else if command == "encircle" || command == "encircled" {
            stdout.queue(SetAttribute(Attribute::Encircled)).unwrap();
        } else if command == "noframeorencircle"
            || command == "noframedorencircled"
            || command == "notframeorencircle"
            || command == "notframedorencircled"
        {
            stdout
                .queue(SetAttribute(Attribute::NotFramedOrEncircled))
                .unwrap();
        } else if command == "overline" || command == "overlined" {
            stdout.queue(SetAttribute(Attribute::OverLined)).unwrap();
        } else if command == "nooverline"
            || command == "nooverlined"
            || command == "notoverline"
            || command == "notoverlined"
        {
            stdout.queue(SetAttribute(Attribute::NotOverLined)).unwrap();
        } else {
            panic!("Unknown tstyle command: {command}")
        }
    }
}

fn parse_color(color: &str) -> Result<Color, String> {
    if color.contains(',') {
        let comma_count = color.chars().filter(|c| *c == ',').count();
        if comma_count != 2 {
            let (only, was_were) = if comma_count == 1 {
                ("only", "was")
            } else {
                ("", "were")
            };
            panic!(
                "Found a comma in the color \"{color}\", which means it should be an RGB color, which means there should be exactly 2 commas in the color, but {only} {comma_count} {was_were} found."
            );
        }
        let (r, gb) = color.split_once(',').unwrap();
        let (g, b) = gb.split_once(',').unwrap();
        let r: u8 = r.parse().map_err(|_| {
            format!("Failed to parse {r} as a number between 0 and 255 (both inclusive)")
        })?;
        let g: u8 = g.parse().map_err(|_| {
            format!("Failed to parse {g} as a number between 0 and 255 (both inclusive)")
        })?;
        let b: u8 = b.parse().map_err(|_| {
            format!("Failed to parse {b} as a number between 0 and 255 (both inclusive)")
        })?;
        return Ok(crossterm::style::Color::Rgb { r, g, b });
    } else if color.chars().all(|c| c.is_ascii_digit()) {
        let ansi_value: u8 = color.parse().map_err(|_| {
            format!("Color number {color} is too big. Must be at most 255 (and at least 0)")
        })?;
        return Ok(Color::AnsiValue(ansi_value));
    } else if color.starts_with('#')
        && color.len() == 7
        && !color
            .chars()
            .skip(1)
            .any(|c| !matches!(c, '0'..='9' | 'a'..='f' | 'A'..='F'))
    {
        let r = u8::from_str_radix(&color[1..=2], 16).unwrap();
        let g = u8::from_str_radix(&color[3..=4], 16).unwrap();
        let b = u8::from_str_radix(&color[5..=6], 16).unwrap();
        return Ok(Color::Rgb { r, g, b });
    }
    match color {
        "black" => Ok(Color::Black),                                        // 0
        "red" | "maroon" => Ok(Color::DarkRed),                             // 1
        "green" | "darklime" => Ok(Color::DarkGreen),                       // 2
        "yellow" | "olive" => Ok(Color::DarkYellow),                        // 3
        "blue" | "navy" => Ok(Color::DarkBlue),                             // 4
        "magenta" | "purple" | "darkfuchsia" => Ok(Color::DarkMagenta),     // 5
        "cyan" | "teal" | "darkaqua" => Ok(Color::DarkCyan),                // 6
        "white" | "silver" => Ok(Color::Grey),                              // 7
        "brightblack" | "grey" | "gray" => Ok(Color::DarkGrey),             // 8
        "brightred" | "brightmaroon" => Ok(Color::Red),                     // 9
        "brightgreen" | "lime" => Ok(Color::Green),                         // 10
        "brightyellow" | "brightolive" => Ok(Color::Yellow),                // 11
        "brightblue" | "brightnavy" => Ok(Color::Blue),                     // 12
        "brightmagenta" | "brightpurple" | "fuchsia" => Ok(Color::Magenta), // 13
        "brightcyan" | "brightteal" | "aqua" => Ok(Color::Cyan),            // 14
        "brightwhite" => Ok(Color::White),                                  // 15

        _ => Err(format!("Unknown color: {color}")),
    }
}
