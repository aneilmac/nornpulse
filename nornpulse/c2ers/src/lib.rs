#![feature(asm)]

const ADDRESS : u32 = 0x00477280;

#[cfg(any(target_arch = "x86"))]
pub fn startup(p1: u32, p2 : u32, p3 : u32) -> u32 {
    let ret_val:u32;
    unsafe {
        asm!("push {address};\
              push {p3};\
              push {p2};\
              push {p1};\
              call [esp+12];\
              mov {ret_val},EAX;",
            address = const ADDRESS,
            p3 = in(reg) p3,
            p2 = in(reg) p2,
            p1 = in(reg) p1,
            ret_val = out(reg) ret_val
            );
    }
    return ret_val;
}
