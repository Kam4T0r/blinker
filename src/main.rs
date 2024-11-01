use std::io::Write;
use std::thread::{self};
use std::time::Duration;

use windows::Win32::UI::Input::KeyboardAndMouse::{self, SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYBD_EVENT_FLAGS, KEYEVENTF_KEYUP, VK_CAPITAL};

fn main()
{
    print!(">");
    let _ = std::io::stdout().flush();
    let mut msg_input = String::new();
    std::io::stdin().read_line(&mut msg_input).unwrap();

    let slices: Vec<_> = msg_input.split_whitespace().collect();
    let slices: Vec<_> = slices.iter().map(|x| x.trim().to_lowercase()).collect();
    let letters: Vec<_> = slices.iter().map(|x| x.chars()).collect();
    let mut letters_two: Vec<char> = Vec::new();

    for i in letters
    {
        for j in i
        {
            letters_two.push(j);
        }
    }

    for i in letters_two
    {
        match i
        {
            'a'=>
            {
                short();
                long();
                thread::sleep(Duration::from_millis(50));
            },
            'b'=>
            {
                long();
                short();
                short();
                short();
                thread::sleep(Duration::from_millis(50));
            },
            'c'=>
            {
                long();
                short();
                long();
                short();
                thread::sleep(Duration::from_millis(50));
            },
            'd'=>
            {
                long();
                short();
                short();
                thread::sleep(Duration::from_millis(50));
            },
            'e'=>
            {
                short();
                thread::sleep(Duration::from_millis(50));
            },
            'f'=>
            {
                short();
                short();
                long();
                short();
                thread::sleep(Duration::from_millis(50));
            },
            'g'=>
            {
                long();
                long();
                short();
                thread::sleep(Duration::from_millis(50));
            },
            'h'=>
            {
                short();
                short();
                short();
                long();
                thread::sleep(Duration::from_millis(50));
            },
            'i'=>
            {
                short();
                short();
                thread::sleep(Duration::from_millis(50));
            },
            'j'=>
            {
                short();
                long();
                long();
                long();
                thread::sleep(Duration::from_millis(50));
            },
            'k'=>
            {
                long();
                short();
                long();
                thread::sleep(Duration::from_millis(50));
            },
            'l'=>
            {
                short();
                long();
                short();
                short();
                thread::sleep(Duration::from_millis(50));
            },
            'm'=>
            {
                long();
                long();
                thread::sleep(Duration::from_millis(50));
            },
            'n'=>
            {
                long();
                short();
                thread::sleep(Duration::from_millis(50));
            },
            'o'=>
            {
                long();
                long();
                long();
                thread::sleep(Duration::from_millis(50));
            },
            'p'=>
            {
                short();
                long();
                long();
                short();
                thread::sleep(Duration::from_millis(50));
            },
            'q'=>
            {
                long();
                long();
                short();
                long();
                thread::sleep(Duration::from_millis(50));
            },
            'r'=>
            {
                short();
                long();
                short();
                thread::sleep(Duration::from_millis(50));
            },
            's'=>
            {
                short();
                short();
                short();
                thread::sleep(Duration::from_millis(50));
            },
            't'=>
            {
                long();
                thread::sleep(Duration::from_millis(50));
            },
            'u'=>
            {
                short();
                short();
                long();
                thread::sleep(Duration::from_millis(50));
            },
            'v'=>
            {
                short();
                short();
                short();
                long();
                thread::sleep(Duration::from_millis(50));
            },
            'w'=>
            {
                short();
                long();
                long();
                thread::sleep(Duration::from_millis(50));
            },
            'x'=>
            {
                long();
                short();
                short();
                long();
                thread::sleep(Duration::from_millis(50));
            },
            'y'=>
            {
                long();
                short();
                long();
                long();
                thread::sleep(Duration::from_millis(50));
            },
            'z'=>
            {
                long();
                long();
                short();
                short();
                thread::sleep(Duration::from_millis(50));
            },
            '1'=>
            {
                short();
                long();
                long();
                long();
                long();
                thread::sleep(Duration::from_millis(50));
            },
            '2'=>
            {
                short();
                short();
                long();
                long();
                thread::sleep(Duration::from_millis(50));
            },
            '3'=>
            {
                short();
                short();
                short();
                long();
                long();
                thread::sleep(Duration::from_millis(50));
            },
            '4'=>
            {
                short();
                short();
                short();
                long();
                thread::sleep(Duration::from_millis(50));
            },
            '5'=>
            {
                short();
                short();
                short();
                short();
                short();
                thread::sleep(Duration::from_millis(50));
            },
            '6'=>
            {
                long();
                short();
                short();
                short();
                short();
                thread::sleep(Duration::from_millis(50));
            },
            '7'=>
            {
                long();
                long();
                short();
                short();
                short();
                thread::sleep(Duration::from_millis(50));
            },
            '8'=>
            {
                long();
                long();
                long();
                short();
                short();
                thread::sleep(Duration::from_millis(50));
            },
            '9'=>
            {
                long();
                long();
                long();
                long();
                short();
                thread::sleep(Duration::from_millis(50));
            },
            '0'=>
            {
                long();
                long();
                long();
                long();
                long();
                thread::sleep(Duration::from_millis(50));
            }
            _=>{}
        }
    }
}

fn short()
{
    let mut input = INPUT
    {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0
        {
            ki: KEYBDINPUT
            {
                wVk: VK_CAPITAL,
                wScan: 0,
                dwFlags: KEYBD_EVENT_FLAGS(0),
                time: 0,
                dwExtraInfo: 0,
            },
        },
    };

    unsafe
    {
        SendInput(&[input], std::mem::size_of::<INPUT>() as i32);
    }
    input.Anonymous.ki.dwFlags = KEYEVENTF_KEYUP;
    thread::sleep(Duration::from_millis(80));
    unsafe
    {
        SendInput(&[input], std::mem::size_of::<INPUT>() as i32);
    }

    unsafe {
        KeyboardAndMouse::keybd_event(0x14, 0, KeyboardAndMouse::KEYBD_EVENT_FLAGS(0), 0); // Press Caps Lock
        thread::sleep(Duration::from_millis(80));
        KeyboardAndMouse::keybd_event(0x14, 0, KeyboardAndMouse::KEYBD_EVENT_FLAGS(2), 0); // Release Caps Lock
    };
}

fn long()
{
    let mut input = INPUT
    {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0
        {
            ki: KEYBDINPUT
            {
                wVk: VK_CAPITAL,
                wScan: 0,
                dwFlags: KEYBD_EVENT_FLAGS(0),
                time: 0,
                dwExtraInfo: 0,
            },
        },
    };

    unsafe
    {
        SendInput(&[input], std::mem::size_of::<INPUT>() as i32);
    }
    input.Anonymous.ki.dwFlags = KEYEVENTF_KEYUP;
    thread::sleep(Duration::from_millis(250));
    unsafe
    {
        SendInput(&[input], std::mem::size_of::<INPUT>() as i32);
    }

    unsafe {
        KeyboardAndMouse::keybd_event(0x14, 0, KeyboardAndMouse::KEYBD_EVENT_FLAGS(0), 0); // Press Caps Lock
        thread::sleep(Duration::from_millis(250));
        KeyboardAndMouse::keybd_event(0x14, 0, KeyboardAndMouse::KEYBD_EVENT_FLAGS(2), 0); // Release Caps Lock
    }
}