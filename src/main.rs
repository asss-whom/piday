use std::f64::consts::TAU;
use std::io::Error;
use std::mem;
use std::thread::sleep;
use std::time::Duration;

use windows::Win32::Foundation::POINT;
use windows::Win32::UI::Input::KeyboardAndMouse::{SendInput, INPUT};
use windows::Win32::UI::Input::KeyboardAndMouse::{
    INPUT_MOUSE, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP,
};
use windows::Win32::UI::WindowsAndMessaging::{GetCursorPos, SetCursorPos};

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    assert!(
        args.len() >= 3,
        "Usage: ./piday.exe [radius: int] [side: int]"
    );

    let radius = args[1].parse::<i32>().unwrap();
    let side = args[2].parse::<i32>().unwrap();
    assert!(
        radius.is_positive(),
        "❌ Error: radius needs to be positive!"
    );
    assert!(side.is_positive(), "❌ Error: side needs to be positive!");

    println!("Task will start after 5 seconds!");
    sleep(Duration::from_secs(5));
    if let Err(e) = draw_circle(radius, side) {
        eprintln!("{}", e)
    }
}

fn leftdown() -> windows::core::Result<()> {
    let mut input = INPUT {
        r#type: INPUT_MOUSE,
        ..Default::default()
    };
    input.Anonymous.mi.dwFlags = MOUSEEVENTF_LEFTDOWN;

    if unsafe { SendInput(&[input], mem::size_of::<INPUT>() as i32) } == 1 {
        Ok(())
    } else {
        Err(windows::core::Error::from(Error::last_os_error()))
    }
}

fn leftup() -> windows::core::Result<()> {
    let mut input = INPUT {
        r#type: INPUT_MOUSE,
        ..Default::default()
    };
    input.Anonymous.mi.dwFlags = MOUSEEVENTF_LEFTUP;

    if unsafe { SendInput(&[input], mem::size_of::<INPUT>() as i32) } == 1 {
        Ok(())
    } else {
        Err(windows::core::Error::from(Error::last_os_error()))
    }
}

fn getpos() -> windows::core::Result<POINT> {
    let mut point = POINT::default();
    unsafe { GetCursorPos(&mut point)? };
    Ok(point)
}

fn setpos(x: i32, y: i32) -> windows::core::Result<()> {
    unsafe { SetCursorPos(x, y) }
}

fn draw_circle(radius: i32, side: i32) -> windows::core::Result<()> {
    let point = getpos()?;
    setpos(point.x + radius, point.y)?;

    leftdown()?;
    for i in 0..=side {
        let f = (TAU * i as f64 / side as f64).sin_cos();
        let x = (radius as f64 * f.1) as i32;
        let y = (radius as f64 * f.0) as i32;
        setpos(point.x + x, point.y + y)?;
        sleep(Duration::from_millis(10));
    }
    leftup()?;

    Ok(())
}
