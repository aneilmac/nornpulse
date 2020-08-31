macro_rules! replace_call {
    ($address:expr, $func_name:path) => {
        #[allow(unused_unsafe)]
        unsafe {
            use winapi::shared::minwindef as win32;
            use winapi::um::memoryapi::VirtualProtect;
            use winapi::um::winnt;

            let page_size = 5; // 1 bytes for opcode. 4 bytes for address to call.
            let offset = 5; // TODO: Would be nice to calculate automatically.

            // First we must make the region writable.
            let mut old_protection: win32::DWORD = 0;
            VirtualProtect(
                $address as win32::LPVOID,
                page_size,
                winnt::PAGE_WRITECOPY,
                &mut old_protection as win32::PDWORD,
            );

            // Insert our replacement function. Relative jump to given address.
            let jump_target: u32 = $func_name as u32 - $address as u32 - offset;

            *std::mem::transmute::<u32, *mut u8>($address) = 0xE9;
            *std::mem::transmute::<u32, *mut u32>($address + 1) = jump_target;

            // Make the region executable once again.
            VirtualProtect(
                $address as win32::LPVOID,
                page_size,
                winnt::PAGE_EXECUTE,
                &mut old_protection as win32::PDWORD,
            );

            // Finally let's flush the instruction cache so we don't run old
            // code from memory.
            use winapi::um::processthreadsapi;
            processthreadsapi::FlushInstructionCache(
                processthreadsapi::GetCurrentProcess(),
                0 as win32::LPCVOID,
                0,
            );
        }
    };
}
