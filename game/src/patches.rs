use crate::utils;
use crate::hooks;
#[allow(overflowing_literals)]
static mut SKIPPING: bool = false;
use std::fs::OpenOptions;
use std::io::prelude::*;

pub fn apply_preloading_game_patches() 
{
	unsafe
	{
		// DISABLE CGameLogic::Update
		utils::memset(0x442AD0 as *mut u32, 0xC3, 1);
		
		//DISABLE SCM
		//utils::memset(0x468EB5 as *mut u32, 0xEB, 1); 
		//utils::memset(0x468EB6 as *mut u32, 0x32, 1);
		
		//need fix crash after hook
		//utils::install_hook(0x469F00 as *mut u8, crunning_script_process_hook as u32);
		
		//max FPS to 60	
		utils::memset(0xC1704C as *mut u32, 60, 1); 
	}
}

pub fn crunning_script_process_hook()
{
	let mut file = OpenOptions::new()
		.write(true)
		.append(true)
		.create(true)
		.open("test.txt")
		.unwrap();
	writeln!(file, "YEAP\r\n").unwrap();
}

pub fn apply_global_game_patches() 
{
	unsafe
	{		
		// Run game instantly
		utils::memset(0x748AA8 as *mut u32, 0x90, 6);
		utils::memcpy(0x748AA8 as *mut u32, b"\xC7\x05\xC0\xD4\xC8\x00\x05\x00\x00\x00".as_ptr(), 10);
		utils::memset(0x748C23 as *mut u32, 0x90, 5);
		utils::memset(0x748C2B as *mut u32, 0x90, 5);

		// DISABLE CPopulation__AddToPopulation
		utils::memset(0x614720 as *mut u32, 0x32, 1);
		utils::memset(0x614721 as *mut u32, 0xC0, 1);
		utils::memset(0x614722 as *mut u32, 0xC3, 1);
		
		// DISABLE random cars
		utils::memset(0x6F2089 as *mut u32, 0x58, 5);
		utils::memset(0x6F208A as *mut u32, 0x90, 5);
		
		// Prevent deleting _any_ far away vehicles - will cause issues for population vehicles in the future
		utils::memset(0x42CD10 as *mut u32, 0xC3, 1);
		
		// Disable CPopulation::ManagePed
		utils::memset(0x611FC0 as *mut u32, 0xC3, 1);
		
		// Stop CPopulation::Update after ManagePopulation call
		utils::memset(0x616698 as *mut u32, 0x5E, 1);
		utils::memset(0x616699 as *mut u32, 0xC3, 1);
	}
}

pub fn d3d_graphics_loop()
{
	unsafe
	{
		if SKIPPING == false
		{
			std::mem::transmute::<u32, fn()>(0x5B1700)(); //skip intro
			SKIPPING = true;
		}
		std::mem::transmute::<u32, fn()>(0x0053E230)(); //call render2dstuff
	}
}

pub fn apply_global_game_hooks() 
{
	unsafe
	{
		utils::memset(0x0053EB13 as *mut u32, d3d_graphics_loop as u32 - 0x0053EB12 - 5, 1);
	}
}