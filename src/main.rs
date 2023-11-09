use std::{path::PathBuf, str::FromStr};

use clap::Parser;
use usbhid::{
    keyboard, mouse,
    prelude::{Device, KeyCodes, KeyCombination, KeyMods, Keyboard},
};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Parser)]
#[command(
    name = "pi0ducky",
    version = VERSION,
    author = "Sammwy",
    about = "Raspberry PI Zero W Duckyscript Payload interpreter",
    long_about = "Raspberry PI Zero W Duckyscript Payload interpreter using USBHID to execute the payload",
)]
struct Args {
    file: Option<String>,

    #[arg(
        short,
        long,
        help = "Interactive mode, you can type the payload directly in the terminal",
        default_value_t = false
    )]
    interactive: bool,

    #[arg(short, long, help = "Duckyscript payload to execute")]
    payload: Option<String>,
}

fn get_key(original: &str) -> Option<KeyCodes> {
    let original = original.to_lowercase().replace("_", "");

    if original == "del" || original == "delete" {
        return Some(KeyCodes::KeyDelete);
    } else if original == "esc" || original == "escape" {
        return Some(KeyCodes::KeyEscape);
    } else if original == "enter" || original == "return" {
        return Some(KeyCodes::KeyEnter);
    } else if original == "uparrow" || original == "up" {
        return Some(KeyCodes::KeyUp);
    } else if original == "downarrow" || original == "down" {
        return Some(KeyCodes::KeyDown);
    } else if original == "leftarrow" || original == "left" {
        return Some(KeyCodes::KeyLeft);
    } else if original == "rightarrow" || original == "right" {
        return Some(KeyCodes::KeyRight);
    } else {
        return KeyCodes::from_str(format!("Key{}", original.to_uppercase()).as_str()).ok();
    }
}

fn get_mod(original: &str) -> Option<KeyMods> {
    let original = original.to_lowercase().replace("_", "");

    if original == "gui" || original == "windows" {
        return Some(KeyMods::ModLeftGui);
    } else if original == "ctrl" || original == "control" {
        return Some(KeyMods::ModLeftCtrl);
    } else {
        return KeyMods::from_str(format!("Mod{}", original.to_uppercase()).as_str()).ok();
    }
}

fn get_keys(list: Vec<&str>) -> KeyCombination {
    let mut keys: Vec<KeyCodes> = Vec::new();
    let mut modifiers: Vec<KeyMods> = Vec::new();

    for code in list {
        let code = code.to_lowercase();
        let code = code.as_str();

        let key = get_key(code);

        if key.is_none() {
            let modifier = get_mod(code);
            if modifier.is_none() {
                panic!("Key/Mod not found for {}", code);
            } else {
                modifiers.push(modifier.unwrap());
            }
        } else {
            keys.push(key.unwrap());
        }
    }

    KeyCombination { keys, modifiers }
}

fn execute_command(cmd: String, keyboard: &mut keyboard::Keyboard, mouse: &mut mouse::Mouse) {
    if cmd.starts_with("#") || cmd.is_empty() {
        return;
    }

    let mut parts = cmd.split(" ");
    let cmd = parts.next().unwrap().to_lowercase();
    let cmd = cmd.as_str();
    let args = parts.collect::<Vec<&str>>();

    match cmd {
        "delay" => {
            let delay = args[0].parse::<u64>().unwrap();
            std::thread::sleep(std::time::Duration::from_millis(delay));
        }
        "string" => {
            let string = args.join(" ");
            keyboard.type_string(string.as_str());
        }
        "stringln" => {
            let string = args.join(" ");
            keyboard.type_string_nl(string.as_str());
        }
        "mouse_move" => {
            let x = args[0].parse::<i8>().unwrap();
            let y = args[1].parse::<i8>().unwrap();
            mouse.move_relative(x, y);
        }
        "mouse_zero" => {
            mouse.move_zero();
        }
        "mouse_to" => {
            let x = args[0].parse::<u16>().unwrap();
            let y = args[1].parse::<u16>().unwrap();
            mouse.move_to(x, y);
        }
        "mouse_left_click" => {
            mouse.left_click();
        }
        "mouse_right_click" => {
            mouse.right_click();
        }
        _ => {
            let args_with_cmd = vec![cmd]
                .into_iter()
                .chain(args.into_iter())
                .collect::<Vec<&str>>();
            let keys = get_keys(args_with_cmd);
            keyboard.press_combination(keys);
            keyboard.release_keys();
        }
    }
}

fn execute_payload(payload: String, keyboard: &mut keyboard::Keyboard, mouse: &mut mouse::Mouse) {
    let lines = payload.split("\n");
    for line in lines {
        let cmd = line.to_string();
        execute_command(cmd, keyboard, mouse);
    }
}

fn start_file(file: PathBuf, keyboard: &mut keyboard::Keyboard, mouse: &mut mouse::Mouse) {
    let content = std::fs::read_to_string(file).unwrap();
    execute_payload(content, keyboard, mouse);
}

fn start_interactive(keyboard: &mut keyboard::Keyboard, mouse: &mut mouse::Mouse) {
    let mut buffer = String::new();
    loop {
        print!("\n> ");
        std::io::stdin().read_line(&mut buffer).unwrap();
        execute_payload(buffer.clone(), keyboard, mouse);
        buffer.clear();
    }
}

fn main() {
    let args = Args::parse();

    let kb_device = Device::new("/dev/hidg0");
    let mut keyboard = Keyboard::new(kb_device, "us");
    let mouse_device = Device::new("/dev/hidg1");
    let mut mouse = mouse::Mouse::new(mouse_device);

    if args.interactive {
        println!("Pi0ducky v{} -- Interactive mode", VERSION);
        start_interactive(&mut keyboard, &mut mouse);
    } else if args.file.is_some() {
        println!("Pi0ducky v{} -- File mode", VERSION);
        start_file(PathBuf::from(args.file.unwrap()), &mut keyboard, &mut mouse);
    } else if args.payload.is_some() {
        println!("Pi0ducky v{} -- Payload mode", VERSION);
        execute_payload(args.payload.unwrap(), &mut keyboard, &mut mouse);
    } else {
        println!("Pi0ducky v{} -- No mode selected", VERSION);
        println!("Please select a mode with -i, -p or pass a file name")
    }
}
