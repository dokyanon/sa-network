extern crate libc;
use crate::address::*;

pub fn get_game_state() -> winapi::DWORD
{
	unsafe { std::ptr::read(ADDR_GAME_STATE as *const u32) }
}

pub fn set_game_state(val: winapi::DWORD)
{
	unsafe { std::ptr::write_volatile(ADDR_GAME_STATE as *mut u32, val); }
}

pub fn is_game_loaded() -> bool
{
	unsafe { std::ptr::read(ADDR_GAME_LOADED as *const u32) != 0 }
}

pub fn find_game_version() -> u32
{
	let uc_a: u8 = unsafe { std::ptr::read(0x748ADD as *const u8) };
	let uc_b: u8 = unsafe { std::ptr::read(0x748ADE as *const u8) };
	
	let mut dw_ret_val: winapi::DWORD = 0xFF;
	if uc_a == 0xFF && uc_b == 0x53
	{
		dw_ret_val = 11; // US_10
	}
	else if uc_a == 0x0F && uc_b == 0x84
	{
		dw_ret_val = 5; // EU_10
	}
	else if uc_a == 0xFE && uc_b == 0x10
	{
		dw_ret_val = 15; // > 1.0
	}
	
	dw_ret_val
}

pub fn check_valid_version() -> bool
{
	let version = find_game_version();
	version == 11 || version == 5
}

pub fn unprotect(address: *mut u32, size: isize)
{
	let mut oldprot = 0;
	unsafe { kernel32::VirtualProtect(address as *mut libc::c_void, size as winapi::SIZE_T, 0x40, &mut oldprot); }
}

pub unsafe extern fn memcpy(dest: *mut u32, src: *const u8, n: isize) {
	unprotect(dest, n);
	let mut i = 0;
	while i < n {
		std::ptr::write_volatile(dest.offset(i), src as u32);
		i += 1;
	}
}

pub unsafe extern fn memmove(dest: *mut u8, src: *const u8,
                             n: usize) -> *mut u8 {
    if src < dest as *const u8 { // copy from end
        let mut i = n;
        while i != 0 {
            i -= 1;
            *dest.offset(i as isize) = *src.offset(i as isize);
        }
    } else { // copy from beginning
        let mut i = 0;
        while i < n {
            *dest.offset(i as isize) = *src.offset(i as isize);
            i += 1;
        }
    }
    return dest;
}

pub unsafe extern fn memset(s: *mut u32, c: u32, n: isize) {
	unprotect(s, n);
	let mut i = 0;
	while i < n {
		std::ptr::write_volatile(s.offset(i), c);
		i += 1;
	}
}

pub unsafe extern fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    let mut i = 0;
    while i < n {
        let a = *s1.offset(i as isize);
        let b = *s2.offset(i as isize);
        if a != b {
            return a as i32 - b as i32
        }
        i += 1;
    }
    return 0;
}


pub unsafe extern fn install_hook(s1: *mut u8, s2: u32)
{
    let mut oldprot = 0;
	kernel32::VirtualProtect(s1 as *mut libc::c_void, 5 as winapi::SIZE_T, 0x40, &mut oldprot);

	let addr = s2 - ((s1 as u32) + 5);
	
    std::ptr::write_unaligned(s1, 0xE9);
    std::ptr::write_unaligned(s1.add(1), (addr & 0xff) as u8);
    std::ptr::write_unaligned(s1.add(2), ((addr >> 8) & 0xff) as u8);
    std::ptr::write_unaligned(s1.add(3), ((addr >> 16) & 0xff) as u8);
    std::ptr::write_unaligned(s1.add(4), ((addr >> 24) & 0xff) as u8);
}