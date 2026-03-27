#![no_std]
#![no_main]

#[no_mangle]
fn efi_main() {
    // println!("Hello, world!");
    let efi_graphics_output_protocol = locate_graphic_protocol(efi_system_table).unwrap();
    let vram_abbr = efi_graphics_output_protocol.mode.frame_buffer_base;
    let vram_byte_size = efi_graphics_output_protocol.mode.frame_buffer_size;
    let vram = unsafe {
        slice::from_raw_parts_mut(vram_abbr as *mnt u32, vram_byte_size / size_of::<u32>())
    };
    for e in vram {
        *e = 0xffffff;
    }
    loop {}
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
