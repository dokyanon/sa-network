use crate::game::address::*;
use crate::game::utils;
use crate::game::patches;

pub fn start_game()
{
	patches::apply_global_game_patches();
	patches::apply_global_game_hooks();
	
	utils::set_game_state(8);
	
	unsafe
	{	
		utils::memset(ADDR_GAME_STARTED as *mut u32, 1, 1);
		utils::memset(ADDR_START_GAME as *mut u32, 0, 1);
		utils::memset(ADDR_GAME_MENU as *mut u32, 0, 1);
		utils::memset(0xB7CB49 as *mut u32, 0, 1);
		utils::memset(0xBA67A4 as *mut u32, 0, 1);
		
		if utils::is_game_loaded()
		{
		
		}
	}
}