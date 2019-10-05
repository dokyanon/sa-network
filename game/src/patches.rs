use crate::utils;
#[allow(overflowing_literals)]
static mut SKIPPING: bool = false;
static mut FRT: bool = false;
use std::fs::OpenOptions;
use std::io::prelude::*;

pub fn apply_preloading_game_patches() 
{
	unsafe
	{
		let mut file = OpenOptions::new()
			.write(true)
			.append(true)
			.create(true)
			.open("out.txt")
			.unwrap();
		
		//utils::memset(0x468EB5 as *mut u32, 0xEB, 1); 
		//utils::memset(0x468EB6 as *mut u32, 0x32, 1); 
		
		utils::unprotect(0x469F00 as *mut u32, 5);
		//writeln!(file, "{}", format_args!("VALUE BEFORE: {:X}\r\n", std::ptr::read((0x469F00 as *mut u32).offset(-1)))).unwrap();
		//writeln!(file, "{}", format_args!("PTR BEFORE: {:X}\r\n", std::ptr::read((0x469F00 as *mut u32)))).unwrap();
		
		//libc::memset(0x469F00 as *mut libc::c_void, 0xE8, 0);
//		let mut ptr: [u32; 2] = [0x90; 2];
//		ptr[0] = 0xE9;
//		ptr[1] = CRunningScript__Process_HOOK as u32 - (0x469F00 + 5);
//		writeln!(file, "{}", format_args!("ARR: {:X} {:X}\r\n", ptr[0], ptr[1])).unwrap();
		
		//*(0x469F00 as *mut u32).offset(-1) = 0xE9;//0xE8;
		//*(0x469F00 as *mut u32).offset(0) = CRunningScript__Process_HOOK as u32 - (0x469F00 + 5);

//		writeln!(file, "{}", format_args!("VAL: {:X}\r\n", u32::from_ne_bytes(ptr))).unwrap();
		
		//utils::memset(0x469F00 as *mut u32, to_u32(&ptr[0..1]), 6);
		//std::ptr::write_volatile((0x469F00 as *mut u32).offset(0), CRunningScript__Process_HOOK as u32 - (0x469F00 + 5));
		
		//writeln!(file, "{}", format_args!("VALUE AFTER: {:X}\r\n", std::ptr::read((0x469F00 as *mut u32).offset(-1)))).unwrap();
		//writeln!(file, "{}", format_args!("PTR AFTER: {:X}\r\n", std::ptr::read((0x469F00 as *mut u32)))).unwrap();
		
		//std::ptr::write_volatile((0x469F00 as *mut u32), CRunningScript__Process_HOOK as u32 - (0x469F00 + 5));
		utils::install_hook(0x469F00 as *mut u8, CRunningScript__Process_HOOK as u32 - (0x469F00 + 5));
		//utils::memset(std::mem::transmute::<u32, *mut u32>(0x469F00).offset(1), CRunningScript__Process_HOOK as u32 - (0x469F00 + 5), 1);
		
		//max FPS to 60	
		utils::memset(0xC1704C as *mut u32, 60, 1); 
	}
}

pub fn CRunningScript__Process_HOOK()
{
	unsafe 
	{
		asm!("pushad" : : : : "intel");

		asm!("popad
			retn" : : : : "intel");
	}
}

pub fn apply_global_game_patches() 
{
	unsafe
	{
		// DISABLE CGameLogic::Update
		utils::memset(0x442AD0 as *mut u32, 0xC3, 1);
		
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