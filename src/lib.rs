//! Send a mouse movement or click event to the system.

pub use platform::{move_mouse, press_mouse, release_mouse, move_click};

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Button {
	Left,
	Right,
	Middle,
}

#[cfg(target_os = "macos")]
mod platform {

}

#[cfg(target_os = "windows")]
mod platform {
	extern crate winapi;
	extern crate user32 as user32_sys;

	use std::mem::{size_of, zeroed};
	use self::winapi::{c_int, LONG};
	use self::winapi::{MOUSEEVENTF_ABSOLUTE, MOUSEEVENTF_MOVE};
	use self::winapi::{MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP};
	use self::winapi::{MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP};
	use self::winapi::{MOUSEEVENTF_MIDDLEDOWN, MOUSEEVENTF_MIDDLEUP};
	use self::winapi::{INPUT, MOUSEINPUT, INPUT_MOUSE};
	use self::winapi::{SM_CXSCREEN, SM_CYSCREEN};
	use self::user32_sys::{SendInput, GetSystemMetrics};

	use super::Button;

	/// Move to pixel (x,y).
	/// WARNING Windows implementation ignores pixels > 65535.
	pub fn move_mouse(x: usize, y: usize) {
		unsafe {
			// Windows absolute is [0, 65536);
			let xf = (((x as u64)*0x10000)/(GetSystemMetrics(SM_CXSCREEN) as u64)) as LONG;
			let yf = (((y as u64)*0x10000)/(GetSystemMetrics(SM_CYSCREEN) as u64)) as LONG;
			let mut input = INPUT {
				type_: INPUT_MOUSE,
				u: zeroed(),
			};
			*input.mi_mut() = MOUSEINPUT {
				dx: xf,
				dy: yf,
				mouseData: 0,
				dwFlags: MOUSEEVENTF_ABSOLUTE|MOUSEEVENTF_MOVE,
				time: 0,
				dwExtraInfo: 0,
			};
			SendInput(1, &mut input, size_of::<INPUT>() as c_int)
		};
	}

	pub fn press_mouse(b: Button) {
		let flag = match b {
			Button::Left => MOUSEEVENTF_LEFTDOWN,
			Button::Right => MOUSEEVENTF_RIGHTDOWN,
			Button::Middle => MOUSEEVENTF_MIDDLEDOWN,
		};
		unsafe {
		let mut input = INPUT {
			type_: INPUT_MOUSE,
			u: zeroed(),
		};
		*input.mi_mut() = MOUSEINPUT {
			dx: 0,
			dy: 0,
			mouseData: 0,
			dwFlags: flag,
			time: 0,
			dwExtraInfo: 0,
		};
		SendInput(1, &mut input, size_of::<INPUT>() as c_int)};
	}

	pub fn release_mouse(b: Button) {
		let flag = match b {
			Button::Left => MOUSEEVENTF_LEFTUP,
			Button::Right => MOUSEEVENTF_RIGHTUP,
			Button::Middle => MOUSEEVENTF_MIDDLEUP,
		};
		unsafe {
		let mut input = INPUT {
			type_: INPUT_MOUSE,
			u: zeroed(),
		};
		*input.mi_mut() = MOUSEINPUT {
			dx: 0,
			dy: 0,
			mouseData: 0,
			dwFlags: flag,
			time: 0,
			dwExtraInfo: 0,
		};
		SendInput(1, &mut input, size_of::<INPUT>() as c_int)};
	}

	pub fn move_click(x: usize, y: usize) {
		move_mouse(x, y);
		press_mouse(Button::Left);
		release_mouse(Button::Left);
	}
}

#[cfg(test)]
mod tests {
	use super::{move_mouse};

	#[test]
	fn test_move_mouse() {
		move_mouse(640, 480);
	}
}
