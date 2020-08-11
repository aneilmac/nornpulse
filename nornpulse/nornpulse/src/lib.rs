use c2ers;

#[no_mangle]
extern "C" fn nornpulse_main(p1 : u32, p2 : u32, p3 : u32 ) -> u32 {
    return c2ers::startup(p1, p2, p3);
}
