extern crate winapi;
extern crate kernel32;

pub mod core;
pub mod address;
extern crate game;

use crate::game::patches;
use crate::game::utils;
use std::fs::OpenOptions;
use std::io::prelude::*;

extern "system" fn exception_filter(exception: *mut winapi::winnt::EXCEPTION_POINTERS) -> i32 
{
    unsafe 
	{ 
        let ctx: *mut winapi::winnt::CONTEXT = (*exception).ContextRecord;
		let mut file = OpenOptions::new()
			.write(true)
			.append(true)
			.create(true)
			.open("crashlog.txt")
			.unwrap();
		writeln!(file, "{}", format_args!("Exception At Address: 0x{:X}; EAX: 0x{:X}; EBX: 0x{:X}; EDX: 0x{:X}\r\n", (*ctx).Eip, (*ctx).Eax, (*ctx).Ebx, (*ctx).Edx)).unwrap();
	}
	0
}

fn initialize_client()
{
	unsafe { kernel32::AllocConsole(); }
	if utils::check_valid_version() == false {
		panic!("GTA SA incorrect version. SA Network supports only 1.0 US/EU");
	}
	else {
		println!("U GAME SA version {}", utils::find_game_version());
	}
	
	patches::apply_preloading_game_patches();

	loop
	{
		if utils::get_game_state() == 7
		{
			core::start_game();
			break;
		}
		std::thread::sleep(std::time::Duration::from_millis(5));
	}
}

#[no_mangle]
pub extern "stdcall" fn DllMain(hinst: winapi::HINSTANCE, reason: u32, _:winapi::LPVOID)
{
	const DLL_PROCESS_DETACH: winapi::DWORD = 0;
	const DLL_PROCESS_ATTACH: winapi::DWORD = 1;
	const DLL_THREAD_ATTACH: winapi::DWORD = 2;
	const DLL_THREAD_DETACH: winapi::DWORD = 3;
	
	match reason
	{
		DLL_PROCESS_ATTACH =>
		{
			unsafe 
			{ 
				kernel32::DisableThreadLibraryCalls(hinst);
				kernel32::SetUnhandledExceptionFilter(Some(exception_filter));
			}
			std::thread::spawn(initialize_client);
		}
		DLL_PROCESS_DETACH => {}
		DLL_THREAD_ATTACH => {}
		DLL_THREAD_DETACH => {}
		_ => {}
	}
}
