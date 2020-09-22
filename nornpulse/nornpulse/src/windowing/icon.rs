pub fn ds_icon_bmp<'a>() -> sdl2::surface::Surface<'a> {
    sdl2::surface::Surface::from_data(
        unsafe { &mut DOCKING_STATION_ICON },
        0x20,
        0x20,
        0x80,
        sdl2::pixels::PixelFormatEnum::RGB888,
    )
    .expect("Could not convert window icon to SDL2 surface.")
}

static mut DOCKING_STATION_ICON: [u8; 4096] = [
    0x1c, 0x1c, 0x1c, 0xff, 0x0c, 0x0c, 0x0c, 0xff, 0x0c, 0x0c, 0x0c, 0xff, 0x0c, 0x0c, 0x0c, 0xff,
    0x0c, 0x0c, 0x0c, 0xff, 0x16, 0x16, 0x16, 0xff, 0x1c, 0x1c, 0x1c, 0xff, 0x1c, 0x1c, 0x1c, 0xff,
    0x1c, 0x1c, 0x1c, 0xff, 0x33, 0x33, 0x00, 0xff, 0x22, 0x22, 0x22, 0xff, 0x22, 0x22, 0x22, 0xff,
    0x33, 0x33, 0x00, 0xff, 0x33, 0x33, 0x00, 0xff, 0x33, 0x33, 0x00, 0xff, 0x33, 0x33, 0x00, 0xff,
    0x33, 0x33, 0x00, 0xff, 0x33, 0x33, 0x00, 0xff, 0x33, 0x33, 0x00, 0xff, 0x33, 0x33, 0x00, 0xff,
    0x33, 0x33, 0x00, 0xff, 0x33, 0x33, 0x00, 0xff, 0x1c, 0x1c, 0x1c, 0xff, 0x16, 0x16, 0x16, 0xff,
    0x16, 0x16, 0x16, 0xff, 0x11, 0x11, 0x11, 0xff, 0x11, 0x11, 0x11, 0xff, 0x0c, 0x0c, 0x0c, 0xff,
    0x0c, 0x0c, 0x0c, 0xff, 0x0c, 0x0c, 0x0c, 0xff, 0x0c, 0x0c, 0x0c, 0xff, 0x1c, 0x1c, 0x1c, 0xff,
    0x04, 0x04, 0x04, 0xff, 0x04, 0x04, 0x04, 0xff, 0x0c, 0x0c, 0x0c, 0xff, 0x00, 0x33, 0x33, 0xff,
    0x00, 0x33, 0x66, 0xff, 0x33, 0x66, 0x66, 0xff, 0x33, 0x66, 0x66, 0xff, 0x33, 0x66, 0x66, 0xff,
    0x33, 0x33, 0x66, 0xff, 0x33, 0x33, 0x33, 0xff, 0x22, 0x22, 0x22, 0xff, 0x22, 0x22, 0x22, 0xff,
    0x29, 0x29, 0x29, 0xff, 0x29, 0x29, 0x29, 0xff, 0x33, 0x33, 0x00, 0xff, 0x33, 0x33, 0x00, 0xff,
    0x33, 0x33, 0x00, 0xff, 0x33, 0x33, 0x00, 0xff, 0x33, 0x33, 0x00, 0xff, 0x33, 0x33, 0x00, 0xff,
    0x22, 0x22, 0x22, 0xff, 0x29, 0x29, 0x29, 0xff, 0x33, 0x33, 0x33, 0xff, 0x33, 0x33, 0x33, 0xff,
    0x1c, 0x1c, 0x1c, 0xff, 0x08, 0x08, 0x08, 0xff, 0x04, 0x04, 0x04, 0xff, 0x04, 0x04, 0x04, 0xff,
    0x04, 0x04, 0x04, 0xff, 0x04, 0x04, 0x04, 0xff, 0x04, 0x04, 0x04, 0xff, 0x04, 0x04, 0x04, 0xff,
    0x0c, 0x0c, 0x0c, 0xff, 0x00, 0x33, 0x66, 0xff, 0x66, 0x99, 0x99, 0xff, 0xb2, 0xb2, 0xb2, 0xff,
    0xcc, 0xcc, 0x99, 0xff, 0xcc, 0xcc, 0x99, 0xff, 0x99, 0x99, 0x66, 0xff, 0x42, 0x42, 0x42, 0xff,
    0x29, 0x29, 0x29, 0xff, 0x33, 0x66, 0x66, 0xff, 0x33, 0x66, 0x99, 0xff, 0x66, 0x66, 0x99, 0xff,
    0x66, 0x66, 0x99, 0xff, 0x66, 0x66, 0x99, 0xff, 0x33, 0x66, 0x99, 0xff, 0x33, 0x66, 0x66, 0xff,
    0x33, 0x66, 0x99, 0xff, 0x33, 0x66, 0x66, 0xff, 0x33, 0x66, 0x66, 0xff, 0x33, 0x66, 0x66, 0xff,
    0x66, 0x66, 0x99, 0xff, 0x66, 0x66, 0x99, 0xff, 0x80, 0x80, 0x80, 0xff, 0x77, 0x77, 0x77, 0xff,
    0x55, 0x55, 0x55, 0xff, 0x00, 0x33, 0x66, 0xff, 0x11, 0x11, 0x11, 0xff, 0x04, 0x04, 0x04, 0xff,
    0x04, 0x04, 0x04, 0xff, 0x04, 0x04, 0x04, 0xff, 0x04, 0x04, 0x04, 0xff, 0x0c, 0x0c, 0x0c, 0xff,
    0x16, 0x16, 0x16, 0xff, 0x99, 0x99, 0x99, 0xff, 0xea, 0xea, 0xea, 0xff, 0xff, 0xec, 0xcc, 0xff,
    0xf0, 0xca, 0xa6, 0xff, 0xcc, 0x99, 0x99, 0xff, 0x99, 0x66, 0x33, 0xff, 0x66, 0x66, 0x33, 0xff,
    0x66, 0x33, 0x00, 0xff, 0x66, 0x66, 0x33, 0xff, 0x99, 0x66, 0x33, 0xff, 0x99, 0x66, 0x66, 0xff,
    0x99, 0x66, 0x66, 0xff, 0x99, 0x66, 0x66, 0xff, 0x66, 0x66, 0x33, 0xff, 0x77, 0x77, 0x77, 0xff,
    0x99, 0x66, 0x66, 0xff, 0x99, 0x99, 0x66, 0xff, 0x99, 0x99, 0x66, 0xff, 0xa4, 0xa0, 0xa0, 0xff,
    0xcc, 0x99, 0x99, 0xff, 0xcc, 0xcc, 0x99, 0xff, 0xcc, 0xcc, 0x99, 0xff, 0xcc, 0xcc, 0x66, 0xff,
    0xcc, 0x99, 0x33, 0xff, 0x99, 0x99, 0x66, 0xff, 0x39, 0x39, 0x39, 0xff, 0x08, 0x08, 0x08, 0xff,
    0x04, 0x04, 0x04, 0xff, 0x04, 0x04, 0x04, 0xff, 0x04, 0x04, 0x04, 0xff, 0x0c, 0x0c, 0x0c, 0xff,
    0x1c, 0x1c, 0x1c, 0xff, 0x77, 0x77, 0x77, 0xff, 0xcc, 0x99, 0x66, 0xff, 0x99, 0x99, 0x66, 0xff,
    0xcc, 0x99, 0x66, 0xff, 0xcc, 0x99, 0x66, 0xff, 0x99, 0x99, 0x33, 0xff, 0x99, 0x66, 0x33, 0xff,
    0x99, 0x66, 0x33, 0xff, 0x99, 0x66, 0x33, 0xff, 0x66, 0x66, 0x33, 0xff, 0x66, 0x66, 0x33, 0xff,
    0x4d, 0x4d, 0x4d, 0xff, 0x42, 0x42, 0x42, 0xff, 0x4d, 0x4d, 0x4d, 0xff, 0x4d, 0x4d, 0x4d, 0xff,
    0x55, 0x55, 0x55, 0xff, 0x66, 0x66, 0x33, 0xff, 0x99, 0x66, 0x33, 0xff, 0x99, 0x66, 0x33, 0xff,
    0x99, 0x66, 0x33, 0xff, 0x99, 0x99, 0x66, 0xff, 0xcc, 0x99, 0x66, 0xff, 0xcc, 0x99, 0x66, 0xff,
    0xcc, 0x99, 0x66, 0xff, 0x77, 0x77, 0x77, 0xff, 0x66, 0x66, 0x33, 0xff, 0x33, 0x33, 0x00, 0xff,
    0x08, 0x08, 0x08, 0xff, 0x04, 0x04, 0x04, 0xff, 0x04, 0x04, 0x04, 0xff, 0x0c, 0x0c, 0x0c, 0xff,
    0x0c, 0x0c, 0x0c, 0xff, 0x39, 0x39, 0x39, 0xff, 0x55, 0x55, 0x55, 0xff, 0x99, 0x99, 0x33, 0xff,
    0xcc, 0xcc, 0x66, 0xff, 0xcc, 0x99, 0x66, 0xff, 0xcc, 0x99, 0x66, 0xff, 0x66, 0x66, 0x33, 0xff,
    0x66, 0x33, 0x33, 0xff, 0x33, 0x33, 0x66, 0xff, 0x00, 0x33, 0x66, 0xff, 0x00, 0x33, 0x66, 0xff,
    0x00, 0x33, 0x66, 0xff, 0x00, 0x33, 0x66, 0xff, 0x00, 0x33, 0x66, 0xff, 0x33, 0x33, 0x66, 0xff,
    0x00, 0x33, 0x99, 0xff, 0x00, 0x33, 0x99, 0xff, 0x00, 0x33, 0x99, 0xff, 0x00, 0x33, 0x66, 0xff,
    0x00, 0x33, 0x66, 0xff, 0x39, 0x39, 0x39, 0xff, 0x99, 0x99, 0x66, 0xff, 0xff, 0xcc, 0x99, 0xff,
    0xff, 0xcc, 0x99, 0xff, 0x99, 0x66, 0x66, 0xff, 0x33, 0x66, 0x66, 0xff, 0x33, 0x33, 0x66, 0xff,
    0x00, 0x33, 0x66, 0xff, 0x1c, 0x1c, 0x1c, 0xff, 0x04, 0x04, 0x04, 0xff, 0x0c, 0x0c, 0x0c, 0xff,
    0x0c, 0x0c, 0x0c, 0xff, 0x16, 0x16, 0x16, 0xff, 0x66, 0x66, 0x33, 0xff, 0x80, 0x80, 0x80, 0xff,
    0x99, 0x66, 0x33, 0xff, 0x99, 0x99, 0x66, 0xff, 0x99, 0x66, 0x33, 0xff, 0x33, 0x33, 0x33, 0xff,
    0x00, 0x33, 0x66, 0xff, 0x33, 0x33, 0x66, 0xff, 0x39, 0x39, 0x39, 0xff, 0x66, 0x66, 0x33, 0xff,
    0x55, 0x55, 0x55, 0xff, 0x55, 0x55, 0x55, 0xff, 0x55, 0x55, 0x55, 0xff, 0x4d, 0x4d, 0x4d, 0xff,
    0x33, 0x33, 0x66, 0xff, 0x33, 0x33, 0x66, 0xff, 0x00, 0x33, 0x99, 0xff, 0x00, 0x33, 0x99, 0xff,
    0x00, 0x33, 0x99, 0xff, 0x00, 0x33, 0x66, 0xff, 0xcc, 0x99, 0x99, 0xff, 0xf0, 0xca, 0xa6, 0xff,
    0xcc, 0xcc, 0x99, 0xff, 0x99, 0x99, 0x66, 0xff, 0x11, 0x11, 0x11, 0xff, 0x66, 0x33, 0x33, 0xff,
    0x66, 0x33, 0x33, 0xff, 0x42, 0x42, 0x42, 0xff, 0x11, 0x11, 0x11, 0xff, 0x0c, 0x0c, 0x0c, 0xff,
    0x0c, 0x0c, 0x0c, 0xff, 0x04, 0x04, 0x04, 0xff, 0x33, 0x33, 0x00, 0xff, 0x5f, 0x5f, 0x5f, 0xff,
    0x66, 0x66, 0x33, 0xff, 0xcc, 0x99, 0x66, 0xff, 0x66, 0x66, 0x33, 0xff, 0x33, 0x33, 0x66, 0xff,
    0x39, 0x39, 0x39, 0xff, 0x66, 0x66, 0x33, 0xff, 0x99, 0x99, 0x66, 0xff, 0x99, 0x99, 0x66, 0xff,
    0x5f, 0x5f, 0x5f, 0xff, 0x16, 0x16, 0x16, 0xff, 0x16, 0x16, 0x16, 0xff, 0x1c, 0x1c, 0x1c, 0xff,
    0x29, 0x29, 0x29, 0xff, 0x29, 0x29, 0x29, 0xff, 0x33, 0x33, 0x00, 0xff, 0x33, 0x33, 0x33, 0xff,
    0x66, 0x33, 0x00, 0xff, 0x16, 0x16, 0x16, 0xff, 0xcb, 0xcb, 0xcb, 0xff, 0xf0, 0xca, 0xa6, 0xff,
    0xcc, 0x99, 0x66, 0xff, 0x66, 0x66, 0x33, 0xff, 0x39, 0x39, 0x39, 0xff, 0x33, 0x33, 0x66, 0xff,
    0x33, 0x33, 0x66, 0xff, 0x66, 0x66, 0x33, 0xff, 0x33, 0x33, 0x00, 0xff, 0x0c, 0x0c, 0x0c, 0xff,
    0x11, 0x11, 0x11, 0xff, 0x08, 0x08, 0x08, 0xff, 0x11, 0x11, 0x11, 0xff, 0x55, 0x55, 0x55, 0xff,
    0x99, 0x99, 0x66, 0xff, 0x99, 0x66, 0x33, 0xff, 0x66, 0x66, 0x33, 0xff, 0x00, 0x33, 0x66, 0xff,
    0x66, 0x66, 0x33, 0xff, 0xcc, 0x99, 0x66, 0xff, 0xcc, 0xcc, 0x33, 0xff, 0x99, 0x99, 0x33, 0xff,
    0x33, 0x33, 0x33, 0xff, 0x22, 0x22, 0x22, 0xff, 0x66, 0x33, 0x33, 0xff, 0x66, 0x66, 0x33, 0xff,
    0x66, 0x66, 0x33, 0xff, 0x66, 0x66, 0x33, 0xff, 0x55, 0x55, 0x55, 0xff, 0x55, 0x55, 0x55, 0xff,
    0x33, 0x66, 0x66, 0xff, 0x33, 0x33, 0x66, 0xff, 0xcc, 0xcc, 0xcc, 0xff, 0xff, 0xff, 0x99, 0xff,
    0xff, 0xcc, 0x99, 0xff, 0x77, 0x77, 0x77, 0xff, 0x00, 0x33, 0x66, 0xff, 0x29, 0x29, 0x29, 0xff,
    0x29, 0x29, 0x29, 0xff, 0x33, 0x33, 0x00, 0xff, 0x16, 0x16, 0x16, 0xff, 0x0c, 0x0c, 0x0c, 0xff,
    0x11, 0x11, 0x11, 0xff, 0x08, 0x08, 0x08, 0xff, 0x11, 0x11, 0x11, 0xff, 0x39, 0x39, 0x39, 0xff,
    0x99, 0x99, 0x66, 0xff, 0x99, 0x66, 0x33, 0xff, 0x42, 0x42, 0x42, 0xff, 0x42, 0x42, 0x42, 0xff,
    0x99, 0x99, 0x33, 0xff, 0xcc, 0x99, 0x33, 0xff, 0x99, 0x99, 0x33, 0xff, 0x99, 0x66, 0x33, 0xff,
    0x33, 0x33, 0x00, 0xff, 0x66, 0x33, 0x33, 0xff, 0x66, 0x66, 0x33, 0xff, 0x55, 0x55, 0x55, 0xff,
    0x33, 0x33, 0x66, 0xff, 0x00, 0x33, 0x66, 0xff, 0x33, 0x33, 0x66, 0xff, 0x39, 0x39, 0x39, 0xff,
    0x39, 0x39, 0x39, 0xff, 0x66, 0x66, 0x99, 0xff, 0xdd, 0xdd, 0xdd, 0xff, 0xcc, 0xcc, 0x99, 0xff,
    0xcc, 0xcc, 0x99, 0xff, 0x33, 0x66, 0x66, 0xff, 0x66, 0x33, 0x00, 0xff, 0x66, 0x66, 0x00, 0xff,
    0x66, 0x33, 0x00, 0xff, 0x33, 0x33, 0x00, 0xff, 0x0c, 0x0c, 0x0c, 0xff, 0x11, 0x11, 0x11, 0xff,
    0x11, 0x11, 0x11, 0xff, 0x0c, 0x0c, 0x0c, 0xff, 0x16, 0x16, 0x16, 0xff, 0x39, 0x39, 0x39, 0xff,
    0x80, 0x80, 0x80, 0xff, 0x99, 0x66, 0x33, 0xff, 0x42, 0x42, 0x42, 0xff, 0x55, 0x55, 0x55, 0xff,
    0x99, 0x99, 0x33, 0xff, 0x99, 0x99, 0x33, 0xff, 0xcc, 0x99, 0x33, 0xff, 0xcc, 0x99, 0x66, 0xff,
    0x66, 0x33, 0x33, 0xff, 0x33, 0x33, 0x00, 0xff, 0x29, 0x29, 0x29, 0xff, 0x00, 0x33, 0x99, 0xff,
    0x33, 0x66, 0x66, 0xff, 0x80, 0x80, 0x00, 0xff, 0x99, 0x66, 0x00, 0xff, 0x99, 0x66, 0x00, 0xff,
    0x99, 0x66, 0x00, 0xff, 0xa4, 0xa0, 0xa0, 0xff, 0xdd, 0xdd, 0xdd, 0xff, 0xcc, 0xcc, 0x99, 0xff,
    0x99, 0x99, 0x66, 0xff, 0x55, 0x55, 0x55, 0xff, 0x66, 0x66, 0x00, 0xff, 0x66, 0x33, 0x00, 0xff,
    0x33, 0x33, 0x00, 0xff, 0x11, 0x11, 0x11, 0xff, 0x0c, 0x0c, 0x0c, 0xff, 0x11, 0x11, 0x11, 0xff,
    0x11, 0x11, 0x11, 0xff, 0x11, 0x11, 0x11, 0xff, 0x1c, 0x1c, 0x1c, 0xff, 0x39, 0x39, 0x39, 0xff,
    0x77, 0x77, 0x77, 0xff, 0x99, 0x66, 0x33, 0xff, 0x39, 0x39, 0x39, 0xff, 0x66, 0x66, 0x33, 0xff,
    0xcc, 0x99, 0x66, 0xff, 0xcc, 0xcc, 0x33, 0xff, 0xff, 0xcc, 0x66, 0xff, 0x99, 0x99, 0x33, 0xff,
    0x99, 0x99, 0x66, 0xff, 0x66, 0x66, 0x33, 0xff, 0x22, 0x22, 0x22, 0xff, 0x08, 0x08, 0x08, 0xff,
    0x1c, 0x1c, 0x1c, 0xff, 0x5f, 0x5f, 0x5f, 0xff, 0xcc, 0x99, 0x66, 0xff, 0xcc, 0x99, 0x33, 0xff,
    0xcc, 0x99, 0x33, 0xff, 0xc0, 0xc0, 0xc0, 0xff, 0xff, 0xec, 0xcc, 0xff, 0xff, 0xcc, 0x99, 0xff,
    0x77, 0x77, 0x77, 0xff, 0x66, 0x66, 0x33, 0xff, 0x66, 0x66, 0x00, 0xff, 0x33, 0x33, 0x00, 0xff,
    0x33, 0x33, 0x00, 0xff, 0x16, 0x16, 0x16, 0xff, 0x0c, 0x0c, 0x0c, 0xff, 0x11, 0x11, 0x11, 0xff,
    0x1c, 0x1c, 0x1c, 0xff, 0x11, 0x11, 0x11, 0xff, 0x1c, 0x1c, 0x1c, 0xff, 0x39, 0x39, 0x39, 0xff,
    0x66, 0x66, 0x66, 0xff, 0x99, 0x66, 0x33, 0xff, 0x33, 0x33, 0x66, 0xff, 0x66, 0x66, 0x33, 0xff,
    0xcc, 0x99, 0x33, 0xff, 0xcc, 0x99, 0x33, 0xff, 0x99, 0x66, 0x33, 0xff, 0xcc, 0xcc, 0x66, 0xff,
    0xff, 0xff, 0x99, 0xff, 0x99, 0x99, 0x66, 0xff, 0x66, 0x66, 0x33, 0xff, 0x16, 0x16, 0x16, 0xff,
    0x1c, 0x1c, 0x1c, 0xff, 0xb2, 0xb2, 0xb2, 0xff, 0xb2, 0xb2, 0xb2, 0xff, 0x66, 0x66, 0x99, 0xff,
    0x66, 0x66, 0x66, 0xff, 0xb2, 0xb2, 0xb2, 0xff, 0xcc, 0xcc, 0x99, 0xff, 0xcc, 0xcc, 0x99, 0xff,
    0x33, 0x66, 0x66, 0xff, 0x66, 0x66, 0x00, 0xff, 0x66, 0x66, 0x00, 0xff, 0x33, 0x33, 0x00, 0xff,
    0x33, 0x33, 0x00, 0xff, 0x16, 0x16, 0x16, 0xff, 0x0c, 0x0c, 0x0c, 0xff, 0x16, 0x16, 0x16, 0xff,
    0x1c, 0x1c, 0x1c, 0xff, 0x11, 0x11, 0x11, 0xff, 0x1c, 0x1c, 0x1c, 0xff, 0x33, 0x33, 0x00, 0xff,
    0x5f, 0x5f, 0x5f, 0xff, 0x66, 0x66, 0x33, 0xff, 0x33, 0x33, 0x66, 0xff, 0x66, 0x66, 0x33, 0xff,
    0xcc, 0x99, 0x33, 0xff, 0x66, 0x66, 0x66, 0xff, 0x33, 0x66, 0x66, 0xff, 0x77, 0x77, 0x77, 0xff,
    0x33, 0x66, 0x66, 0xff, 0x33, 0x66, 0x66, 0xff, 0x33, 0x66, 0x99, 0xff, 0x33, 0x33, 0x66, 0xff,
    0x1c, 0x1c, 0x1c, 0xff, 0xb2, 0xb2, 0xb2, 0xff, 0xf8, 0xf8, 0xf8, 0xff, 0xcc, 0xcc, 0xcc, 0xff,
    0x66, 0x66, 0x99, 0xff, 0x77, 0x77, 0x77, 0xff, 0xcc, 0x99, 0x66, 0xff, 0x99, 0x99, 0x66, 0xff,
    0x55, 0x55, 0x55, 0xff, 0x66, 0x66, 0x00, 0xff, 0x66, 0x66, 0x00, 0xff, 0x33, 0x33, 0x00, 0xff,
    0x33, 0x33, 0x00, 0xff, 0x16, 0x16, 0x16, 0xff, 0x0c, 0x0c, 0x0c, 0xff, 0x16, 0x16, 0x16, 0xff,
    0x1c, 0x1c, 0x1c, 0xff, 0x11, 0x11, 0x11, 0xff, 0x33, 0x33, 0x00, 0xff, 0x33, 0x33, 0x00, 0xff,
    0x66, 0x66, 0x33, 0xff, 0x4d, 0x4d, 0x4d, 0xff, 0x33, 0x33, 0x66, 0xff, 0x80, 0x80, 0x00, 0xff,
    0x99, 0x66, 0x00, 0xff, 0x99, 0x66, 0x00, 0xff, 0x66, 0x33, 0x33, 0xff, 0x33, 0x33, 0x00, 0xff,
    0x66, 0x66, 0x00, 0xff, 0x99, 0x66, 0x33, 0xff, 0x99, 0x66, 0x33, 0xff, 0x66, 0x66, 0x33, 0xff,
    0x33, 0x33, 0x66, 0xff, 0x00, 0x33, 0x66, 0xff, 0x66, 0x99, 0x99, 0xff, 0xdd, 0xdd, 0xdd, 0xff,
    0xcc, 0xcc, 0xcc, 0xff, 0x66, 0x66, 0x66, 0xff, 0x99, 0x66, 0x33, 0xff, 0x55, 0x55, 0x55, 0xff,
    0x66, 0x66, 0x66, 0xff, 0x99, 0x66, 0x00, 0xff, 0x66, 0x66, 0x00, 0xff, 0x33, 0x33, 0x00, 0xff,
    0x33, 0x33, 0x00, 0xff, 0x16, 0x16, 0x16, 0xff, 0x11, 0x11, 0x11, 0xff, 0x16, 0x16, 0x16, 0xff,
    0x16, 0x16, 0x16, 0xff, 0x11, 0x11, 0x11, 0xff, 0x33, 0x33, 0x00, 0xff, 0x33, 0x33, 0x00, 0xff,
    0x66, 0x66, 0x00, 0xff, 0x66, 0x66, 0x33, 0xff, 0x29, 0x29, 0x29, 0xff, 0x99, 0x66, 0x00, 0xff,
    0x99, 0x66, 0x00, 0xff, 0x99, 0x66, 0x00, 0xff, 0x99, 0x66, 0x00, 0xff, 0xcc, 0x99, 0x00, 0xff,
    0xcc, 0xcc, 0x33, 0xff, 0xcc, 0xcc, 0x66, 0xff, 0xff, 0xff, 0x99, 0xff, 0xff, 0xcc, 0x66, 0xff,
    0xcc, 0xcc, 0x33, 0xff, 0x66, 0x66, 0x33, 0xff, 0x33, 0x33, 0x66, 0xff, 0x33, 0x66, 0x66, 0xff,
    0x99, 0x99, 0x99, 0xff, 0xa4, 0xa0, 0xa0, 0xff, 0x5f, 0x5f, 0x5f, 0xff, 0x5f, 0x5f, 0x5f, 0xff,
    0x99, 0x99, 0x66, 0xff, 0x99, 0x99, 0x33, 0xff, 0x66, 0x66, 0x00, 0xff, 0x33, 0x33, 0x00, 0xff,
    0x33, 0x33, 0x00, 0xff, 0x16, 0x16, 0x16, 0xff, 0x11, 0x11, 0x11, 0xff, 0x16, 0x16, 0x16, 0xff,
    0x16, 0x16, 0x16, 0xff, 0x11, 0x11, 0x11, 0xff, 0x33, 0x33, 0x00, 0xff, 0x33, 0x33, 0x00, 0xff,
    0x66, 0x33, 0x00, 0xff, 0x80, 0x80, 0x00, 0xff, 0x66, 0x66, 0x00, 0xff, 0x99, 0x66, 0x00, 0xff,
    0x99, 0x66, 0x00, 0xff, 0x99, 0x66, 0x00, 0xff, 0x99, 0x66, 0x00, 0xff, 0xcc, 0x99, 0x00, 0xff,
    0xcc, 0xcc, 0x33, 0xff, 0xcc, 0xff, 0x33, 0xff, 0xff, 0xff, 0x99, 0xff, 0xff, 0xff, 0x99, 0xff,
    0xcc, 0xff, 0x33, 0xff, 0xcc, 0xcc, 0x33, 0xff, 0x99, 0x99, 0x66, 0xff, 0x29, 0x29, 0x29, 0xff,
    0x5f, 0x5f, 0x5f, 0xff, 0x96, 0x96, 0x96, 0xff, 0x99, 0x99, 0x99, 0xff, 0x55, 0x55, 0x55, 0xff,
    0x42, 0x42, 0x42, 0xff, 0x99, 0x99, 0x66, 0xff, 0x99, 0x66, 0x33, 0xff, 0x33, 0x33, 0x00, 0xff,
    0x33, 0x33, 0x00, 0xff, 0x16, 0x16, 0x16, 0xff, 0x11, 0x11, 0x11, 0xff, 0x16, 0x16, 0x16, 0xff,
    0x16, 0x16, 0x16, 0xff, 0x11, 0x11, 0x11, 0xff, 0x16, 0x16, 0x16, 0xff, 0x33, 0x33, 0x00, 0xff,
    0x33, 0x33, 0x00, 0xff, 0xb2, 0xb2, 0xb2, 0xff, 0x96, 0x96, 0x96, 0xff, 0x80, 0x80, 0x00, 0xff,
    0x99, 0x66, 0x00, 0xff, 0x99, 0x66, 0x00, 0xff, 0x99, 0x66, 0x00, 0xff, 0xcc, 0x99, 0x00, 0xff,
    0xcc, 0x99, 0x33, 0xff, 0xcc, 0xcc, 0x33, 0xff, 0xcc, 0xff, 0x33, 0xff, 0xcc, 0xff, 0x33, 0xff,
    0xcc, 0xff, 0x33, 0xff, 0xcc, 0xcc, 0x66, 0xff, 0x66, 0x66, 0x33, 0xff, 0x99, 0x99, 0x33, 0xff,
    0xcc, 0x99, 0x66, 0xff, 0x55, 0x55, 0x55, 0xff, 0x96, 0x96, 0x96, 0xff, 0x80, 0x80, 0x80, 0xff,
    0x5f, 0x5f, 0x5f, 0xff, 0x99, 0x66, 0x33, 0xff, 0x99, 0x99, 0x66, 0xff, 0x66, 0x66, 0x33, 0xff,
    0x33, 0x33, 0x00, 0xff, 0x16, 0x16, 0x16, 0xff, 0x0c, 0x0c, 0x0c, 0xff, 0x16, 0x16, 0x16, 0xff,
    0x16, 0x16, 0x16, 0xff, 0x11, 0x11, 0x11, 0xff, 0x16, 0x16, 0x16, 0xff, 0x33, 0x33, 0x00, 0xff,
    0x42, 0x42, 0x42, 0xff, 0xcc, 0xcc, 0xcc, 0xff, 0xcc, 0x99, 0x99, 0xff, 0x99, 0x66, 0x33, 0xff,
    0x99, 0x66, 0x00, 0xff, 0x99, 0x66, 0x00, 0xff, 0xcc, 0x99, 0x00, 0xff, 0xcc, 0x99, 0x00, 0xff,
    0xcc, 0x99, 0x33, 0xff, 0xcc, 0xcc, 0x33, 0xff, 0xcc, 0xcc, 0x33, 0xff, 0xcc, 0xff, 0x33, 0xff,
    0xcc, 0xcc, 0x66, 0xff, 0x42, 0x42, 0x42, 0xff, 0x66, 0x33, 0x33, 0xff, 0xcc, 0x99, 0x66, 0xff,
    0x96, 0x96, 0x96, 0xff, 0x33, 0x33, 0x66, 0xff, 0x33, 0x33, 0x66, 0xff, 0x99, 0x66, 0x66, 0xff,
    0x99, 0x66, 0x66, 0xff, 0x42, 0x42, 0x42, 0xff, 0x66, 0x66, 0x33, 0xff, 0x99, 0x99, 0x66, 0xff,
    0x42, 0x42, 0x42, 0xff, 0x16, 0x16, 0x16, 0xff, 0x0c, 0x0c, 0x0c, 0xff, 0x11, 0x11, 0x11, 0xff,
    0x11, 0x11, 0x11, 0xff, 0x0c, 0x0c, 0x0c, 0xff, 0x16, 0x16, 0x16, 0xff, 0x33, 0x33, 0x00, 0xff,
    0x42, 0x42, 0x42, 0xff, 0xcb, 0xcb, 0xcb, 0xff, 0xcc, 0x99, 0x99, 0xff, 0x66, 0x66, 0x66, 0xff,
    0x99, 0x99, 0x33, 0xff, 0x99, 0x99, 0x66, 0xff, 0x99, 0x99, 0x66, 0xff, 0x99, 0x99, 0x66, 0xff,
    0xcc, 0x99, 0x33, 0xff, 0xcc, 0xcc, 0x33, 0xff, 0xcc, 0xcc, 0x33, 0xff, 0xcc, 0x99, 0x33, 0xff,
    0x39, 0x39, 0x39, 0xff, 0x66, 0x66, 0x33, 0xff, 0xcc, 0xcc, 0x66, 0xff, 0x77, 0x77, 0x77, 0xff,
    0x42, 0x42, 0x42, 0xff, 0x66, 0x66, 0x00, 0xff, 0x66, 0x66, 0x66, 0xff, 0x4d, 0x4d, 0x4d, 0xff,
    0x99, 0x66, 0x33, 0xff, 0x5f, 0x5f, 0x5f, 0xff, 0x99, 0x66, 0x33, 0xff, 0xcc, 0xcc, 0x66, 0xff,
    0x77, 0x77, 0x77, 0xff, 0x22, 0x22, 0x22, 0xff, 0x0c, 0x0c, 0x0c, 0xff, 0x11, 0x11, 0x11, 0xff,
    0x11, 0x11, 0x11, 0xff, 0x0c, 0x0c, 0x0c, 0xff, 0x11, 0x11, 0x11, 0xff, 0x33, 0x33, 0x00, 0xff,
    0x42, 0x42, 0x42, 0xff, 0xb2, 0xb2, 0xb2, 0xff, 0xa4, 0xa0, 0xa0, 0xff, 0x80, 0x80, 0x80, 0xff,
    0xcc, 0x99, 0x99, 0xff, 0x96, 0x96, 0x96, 0xff, 0x55, 0x55, 0x55, 0xff, 0x33, 0x33, 0x33, 0xff,
    0x1c, 0x1c, 0x1c, 0xff, 0x66, 0x66, 0x33, 0xff, 0x99, 0x66, 0x33, 0xff, 0x1c, 0x1c, 0x1c, 0xff,
    0x66, 0x33, 0x33, 0xff, 0xcc, 0x99, 0x66, 0xff, 0x96, 0x96, 0x96, 0xff, 0x42, 0x42, 0x42, 0xff,
    0x66, 0x66, 0x00, 0xff, 0x99, 0x99, 0x00, 0xff, 0xcc, 0x99, 0x00, 0xff, 0x66, 0x66, 0x66, 0xff,
    0x4d, 0x4d, 0x4d, 0xff, 0x99, 0x66, 0x33, 0xff, 0x99, 0x66, 0x33, 0xff, 0x99, 0x66, 0x33, 0xff,
    0x99, 0x66, 0x33, 0xff, 0x66, 0x66, 0x33, 0xff, 0x0c, 0x0c, 0x0c, 0xff, 0x11, 0x11, 0x11, 0xff,
    0x11, 0x11, 0x11, 0xff, 0x08, 0x08, 0x08, 0xff, 0x11, 0x11, 0x11, 0xff, 0x16, 0x16, 0x16, 0xff,
    0x39, 0x39, 0x39, 0xff, 0xb2, 0xb2, 0xb2, 0xff, 0xff, 0xec, 0xcc, 0xff, 0xcc, 0x99, 0x99, 0xff,
    0x99, 0x66, 0x33, 0xff, 0x66, 0x33, 0x33, 0xff, 0x66, 0x33, 0x33, 0xff, 0x5f, 0x5f, 0x5f, 0xff,
    0x80, 0x80, 0x80, 0xff, 0xb2, 0xb2, 0xb2, 0xff, 0xb2, 0xb2, 0xb2, 0xff, 0x99, 0x66, 0x66, 0xff,
    0x99, 0x99, 0x66, 0xff, 0x66, 0x66, 0x99, 0xff, 0x33, 0x66, 0x66, 0xff, 0x66, 0x66, 0x00, 0xff,
    0xcc, 0x99, 0x00, 0xff, 0x99, 0x99, 0x33, 0xff, 0x99, 0x66, 0x00, 0xff, 0x99, 0x99, 0x33, 0xff,
    0x66, 0x66, 0x66, 0xff, 0x66, 0x66, 0x33, 0xff, 0x99, 0x99, 0x66, 0xff, 0xcc, 0x99, 0x66, 0xff,
    0xff, 0xcc, 0x66, 0xff, 0xcc, 0x99, 0x66, 0xff, 0x22, 0x22, 0x22, 0xff, 0x0c, 0x0c, 0x0c, 0xff,
    0x0c, 0x0c, 0x0c, 0xff, 0x08, 0x08, 0x08, 0xff, 0x0c, 0x0c, 0x0c, 0xff, 0x16, 0x16, 0x16, 0xff,
    0x33, 0x66, 0x66, 0xff, 0xdd, 0xdd, 0xdd, 0xff, 0xcc, 0xcc, 0x99, 0xff, 0x66, 0x66, 0x33, 0xff,
    0x33, 0x33, 0x33, 0xff, 0x1c, 0x1c, 0x1c, 0xff, 0x4d, 0x4d, 0x4d, 0xff, 0xdd, 0xdd, 0xdd, 0xff,
    0xf8, 0xf8, 0xf8, 0xff, 0xd7, 0xd7, 0xd7, 0xff, 0xcc, 0x99, 0x99, 0xff, 0x99, 0x66, 0x66, 0xff,
    0x39, 0x39, 0x39, 0xff, 0x00, 0x33, 0x33, 0xff, 0x33, 0x66, 0x66, 0xff, 0x99, 0x99, 0x33, 0xff,
    0x99, 0x99, 0x33, 0xff, 0x66, 0x66, 0x33, 0xff, 0x66, 0x66, 0x33, 0xff, 0x66, 0x66, 0x33, 0xff,
    0x99, 0x99, 0x66, 0xff, 0x99, 0x66, 0x66, 0xff, 0x99, 0x66, 0x66, 0xff, 0xcc, 0x99, 0x66, 0xff,
    0xcc, 0x99, 0x66, 0xff, 0x99, 0x99, 0x66, 0xff, 0x66, 0x66, 0x33, 0xff, 0x16, 0x16, 0x16, 0xff,
    0x0c, 0x0c, 0x0c, 0xff, 0x04, 0x04, 0x04, 0xff, 0x08, 0x08, 0x08, 0xff, 0x33, 0x33, 0x66, 0xff,
    0xcc, 0xcc, 0xcc, 0xff, 0xd7, 0xd7, 0xd7, 0xff, 0x66, 0x66, 0x33, 0xff, 0x1c, 0x1c, 0x1c, 0xff,
    0x33, 0x33, 0x33, 0xff, 0x33, 0x33, 0x33, 0xff, 0x99, 0x66, 0x66, 0xff, 0xf0, 0xca, 0xa6, 0xff,
    0xcc, 0x99, 0x66, 0xff, 0x99, 0x99, 0x66, 0xff, 0x55, 0x55, 0x55, 0xff, 0x33, 0x33, 0x00, 0xff,
    0x66, 0x33, 0x33, 0xff, 0x99, 0x99, 0x66, 0xff, 0x96, 0x96, 0x96, 0xff, 0x96, 0x96, 0x96, 0xff,
    0x66, 0x66, 0x99, 0xff, 0x66, 0x66, 0x99, 0xff, 0x77, 0x77, 0x77, 0xff, 0x66, 0x66, 0x99, 0xff,
    0xcb, 0xcb, 0xcb, 0xff, 0xcb, 0xcb, 0xcb, 0xff, 0xcc, 0x99, 0x99, 0xff, 0xcc, 0x99, 0x66, 0xff,
    0x99, 0x99, 0x66, 0xff, 0x99, 0x99, 0x66, 0xff, 0x66, 0x66, 0x66, 0xff, 0x33, 0x33, 0x00, 0xff,
    0x0c, 0x0c, 0x0c, 0xff, 0x04, 0x04, 0x04, 0xff, 0x16, 0x16, 0x16, 0xff, 0x99, 0x99, 0x99, 0xff,
    0xf8, 0xf8, 0xf8, 0xff, 0xcc, 0x99, 0x99, 0xff, 0x33, 0x33, 0x00, 0xff, 0x39, 0x39, 0x39, 0xff,
    0x66, 0x66, 0x33, 0xff, 0x66, 0x66, 0x33, 0xff, 0x99, 0x66, 0x33, 0xff, 0x99, 0x66, 0x33, 0xff,
    0x33, 0x66, 0x66, 0xff, 0x33, 0x66, 0x66, 0xff, 0x00, 0x33, 0x66, 0xff, 0x00, 0x33, 0x66, 0xff,
    0x39, 0x39, 0x39, 0xff, 0x99, 0x99, 0x66, 0xff, 0xcb, 0xcb, 0xcb, 0xff, 0xea, 0xea, 0xea, 0xff,
    0xcc, 0xcc, 0xcc, 0xff, 0xb2, 0xb2, 0xb2, 0xff, 0xa4, 0xa0, 0xa0, 0xff, 0xd7, 0xd7, 0xd7, 0xff,
    0xea, 0xea, 0xea, 0xff, 0xea, 0xea, 0xea, 0xff, 0xf0, 0xca, 0xa6, 0xff, 0xff, 0xff, 0x99, 0xff,
    0xff, 0xff, 0x99, 0xff, 0xff, 0xcc, 0x99, 0xff, 0x77, 0x77, 0x77, 0xff, 0x33, 0x33, 0x00, 0xff,
    0x0c, 0x0c, 0x0c, 0xff, 0x04, 0x04, 0x04, 0xff, 0x00, 0x33, 0x33, 0xff, 0xa4, 0xa0, 0xa0, 0xff,
    0xff, 0xff, 0x99, 0xff, 0x99, 0x99, 0x66, 0xff, 0x66, 0x66, 0x66, 0xff, 0x99, 0x66, 0x66, 0xff,
    0x99, 0x99, 0x66, 0xff, 0x99, 0x66, 0x33, 0xff, 0x99, 0x66, 0x33, 0xff, 0x33, 0x33, 0x66, 0xff,
    0x33, 0x33, 0x66, 0xff, 0x33, 0x33, 0x00, 0xff, 0x66, 0x66, 0x00, 0xff, 0x66, 0x66, 0x33, 0xff,
    0x33, 0x66, 0x66, 0xff, 0x00, 0x33, 0x66, 0xff, 0x29, 0x29, 0x29, 0xff, 0x66, 0x66, 0x66, 0xff,
    0xc0, 0xc0, 0xc0, 0xff, 0xcc, 0xcc, 0xcc, 0xff, 0xea, 0xea, 0xea, 0xff, 0xf8, 0xf8, 0xf8, 0xff,
    0xf8, 0xf8, 0xf8, 0xff, 0xff, 0xec, 0xcc, 0xff, 0xcc, 0x99, 0x66, 0xff, 0xcc, 0x99, 0x66, 0xff,
    0xcc, 0xcc, 0x99, 0xff, 0xcc, 0x99, 0x66, 0xff, 0x5f, 0x5f, 0x5f, 0xff, 0x66, 0x33, 0x00, 0xff,
    0x0c, 0x0c, 0x0c, 0xff, 0x04, 0x04, 0x04, 0xff, 0x1c, 0x1c, 0x1c, 0xff, 0x99, 0x99, 0x66, 0xff,
    0x99, 0x66, 0x33, 0xff, 0x99, 0x99, 0x66, 0xff, 0x99, 0x99, 0x66, 0xff, 0x99, 0x99, 0x66, 0xff,
    0x99, 0x99, 0x66, 0xff, 0x99, 0x66, 0x33, 0xff, 0x33, 0x66, 0x66, 0xff, 0x33, 0x33, 0x66, 0xff,
    0x66, 0x33, 0x00, 0xff, 0x80, 0x80, 0x00, 0xff, 0x66, 0x66, 0x33, 0xff, 0x99, 0x66, 0x33, 0xff,
    0x99, 0x66, 0x00, 0xff, 0x66, 0x66, 0x33, 0xff, 0x00, 0x33, 0x66, 0xff, 0x4d, 0x4d, 0x4d, 0xff,
    0x66, 0x66, 0x33, 0xff, 0x5f, 0x5f, 0x5f, 0xff, 0x99, 0x99, 0x66, 0xff, 0xcc, 0xcc, 0x99, 0xff,
    0xcc, 0xcc, 0x99, 0xff, 0xcc, 0xcc, 0x99, 0xff, 0xff, 0xcc, 0x99, 0xff, 0xff, 0xcc, 0x99, 0xff,
    0xcc, 0xcc, 0x66, 0xff, 0xcc, 0x99, 0x66, 0xff, 0x55, 0x55, 0x55, 0xff, 0x33, 0x33, 0x00, 0xff,
    0x0c, 0x0c, 0x0c, 0xff, 0x04, 0x04, 0x04, 0xff, 0x08, 0x08, 0x08, 0xff, 0x55, 0x55, 0x55, 0xff,
    0x55, 0x55, 0x55, 0xff, 0x33, 0x66, 0x66, 0xff, 0x33, 0x66, 0x66, 0xff, 0x5f, 0x5f, 0x5f, 0xff,
    0x33, 0x66, 0x66, 0xff, 0x33, 0x66, 0x66, 0xff, 0x33, 0x33, 0x66, 0xff, 0x66, 0x33, 0x00, 0xff,
    0x66, 0x66, 0x00, 0xff, 0x66, 0x66, 0x00, 0xff, 0x33, 0x33, 0x00, 0xff, 0x66, 0x66, 0x00, 0xff,
    0x66, 0x66, 0x00, 0xff, 0x80, 0x80, 0x00, 0xff, 0x99, 0x66, 0x00, 0xff, 0x55, 0x55, 0x55, 0xff,
    0x33, 0x66, 0x66, 0xff, 0x66, 0x66, 0x66, 0xff, 0x66, 0x66, 0x33, 0xff, 0x99, 0x66, 0x33, 0xff,
    0x99, 0x66, 0x33, 0xff, 0xcc, 0x99, 0x33, 0xff, 0xff, 0xcc, 0x66, 0xff, 0xff, 0xcc, 0x66, 0xff,
    0xff, 0xcc, 0x66, 0xff, 0x99, 0x99, 0x66, 0xff, 0x42, 0x42, 0x42, 0xff, 0x33, 0x33, 0x00, 0xff,
    0x0c, 0x0c, 0x0c, 0xff, 0x04, 0x04, 0x04, 0xff, 0x04, 0x04, 0x04, 0xff, 0x33, 0x33, 0x00, 0xff,
    0x33, 0x33, 0x00, 0xff, 0x33, 0x33, 0x00, 0xff, 0x33, 0x33, 0x00, 0xff, 0x33, 0x33, 0x00, 0xff,
    0x33, 0x33, 0x00, 0xff, 0x66, 0x33, 0x00, 0xff, 0x33, 0x33, 0x00, 0xff, 0x66, 0x33, 0x00, 0xff,
    0x33, 0x33, 0x00, 0xff, 0x33, 0x33, 0x00, 0xff, 0x33, 0x33, 0x00, 0xff, 0x33, 0x33, 0x00, 0xff,
    0x33, 0x33, 0x00, 0xff, 0x33, 0x33, 0x00, 0xff, 0x66, 0x33, 0x00, 0xff, 0x66, 0x66, 0x00, 0xff,
    0x66, 0x66, 0x33, 0xff, 0x33, 0x66, 0x66, 0xff, 0x77, 0x77, 0x77, 0xff, 0x99, 0x99, 0x66, 0xff,
    0x99, 0x99, 0x33, 0xff, 0x66, 0x66, 0x33, 0xff, 0x99, 0x66, 0x33, 0xff, 0x99, 0x99, 0x33, 0xff,
    0x99, 0x99, 0x66, 0xff, 0x33, 0x66, 0x66, 0xff, 0x33, 0x33, 0x00, 0xff, 0x33, 0x33, 0x00, 0xff,
    0x0c, 0x0c, 0x0c, 0xff, 0x04, 0x04, 0x04, 0xff, 0x04, 0x04, 0x04, 0xff, 0x04, 0x04, 0x04, 0xff,
    0x11, 0x11, 0x11, 0xff, 0x33, 0x33, 0x00, 0xff, 0x33, 0x33, 0x00, 0xff, 0x33, 0x33, 0x00, 0xff,
    0x33, 0x33, 0x00, 0xff, 0x33, 0x33, 0x00, 0xff, 0x33, 0x33, 0x00, 0xff, 0x33, 0x33, 0x00, 0xff,
    0x22, 0x22, 0x22, 0xff, 0x33, 0x33, 0x00, 0xff, 0x33, 0x33, 0x00, 0xff, 0x33, 0x33, 0x00, 0xff,
    0x33, 0x33, 0x00, 0xff, 0x33, 0x33, 0x00, 0xff, 0x33, 0x33, 0x00, 0xff, 0x33, 0x33, 0x00, 0xff,
    0x33, 0x33, 0x00, 0xff, 0x66, 0x66, 0x00, 0xff, 0x42, 0x42, 0x42, 0xff, 0x33, 0x33, 0x66, 0xff,
    0x33, 0x66, 0x66, 0xff, 0x33, 0x66, 0x66, 0xff, 0x33, 0x33, 0x66, 0xff, 0x33, 0x66, 0x66, 0xff,
    0x33, 0x33, 0x66, 0xff, 0x33, 0x33, 0x00, 0xff, 0x33, 0x33, 0x00, 0xff, 0x16, 0x16, 0x16, 0xff,
    0x04, 0x04, 0x04, 0xff, 0x04, 0x04, 0x04, 0xff, 0x04, 0x04, 0x04, 0xff, 0x04, 0x04, 0x04, 0xff,
    0x04, 0x04, 0x04, 0xff, 0x04, 0x04, 0x04, 0xff, 0x04, 0x04, 0x04, 0xff, 0x08, 0x08, 0x08, 0xff,
    0x08, 0x08, 0x08, 0xff, 0x0c, 0x0c, 0x0c, 0xff, 0x11, 0x11, 0x11, 0xff, 0x16, 0x16, 0x16, 0xff,
    0x1c, 0x1c, 0x1c, 0xff, 0x33, 0x33, 0x00, 0xff, 0x33, 0x33, 0x00, 0xff, 0x33, 0x33, 0x00, 0xff,
    0x33, 0x33, 0x00, 0xff, 0x33, 0x33, 0x00, 0xff, 0x16, 0x16, 0x16, 0xff, 0x16, 0x16, 0x16, 0xff,
    0x11, 0x11, 0x11, 0xff, 0x16, 0x16, 0x16, 0xff, 0x33, 0x33, 0x00, 0xff, 0x66, 0x33, 0x00, 0xff,
    0x33, 0x33, 0x00, 0xff, 0x33, 0x33, 0x00, 0xff, 0x29, 0x29, 0x29, 0xff, 0x33, 0x33, 0x00, 0xff,
    0x33, 0x33, 0x00, 0xff, 0x33, 0x33, 0x00, 0xff, 0x0c, 0x0c, 0x0c, 0xff, 0x04, 0x04, 0x04, 0xff,
    0x1c, 0x1c, 0x1c, 0xff, 0x0c, 0x0c, 0x0c, 0xff, 0x0c, 0x0c, 0x0c, 0xff, 0x0c, 0x0c, 0x0c, 0xff,
    0x0c, 0x0c, 0x0c, 0xff, 0x0c, 0x0c, 0x0c, 0xff, 0x0c, 0x0c, 0x0c, 0xff, 0x0c, 0x0c, 0x0c, 0xff,
    0x11, 0x11, 0x11, 0xff, 0x11, 0x11, 0x11, 0xff, 0x16, 0x16, 0x16, 0xff, 0x16, 0x16, 0x16, 0xff,
    0x1c, 0x1c, 0x1c, 0xff, 0x33, 0x33, 0x00, 0xff, 0x33, 0x33, 0x00, 0xff, 0x33, 0x33, 0x00, 0xff,
    0x33, 0x33, 0x00, 0xff, 0x33, 0x33, 0x00, 0xff, 0x1c, 0x1c, 0x1c, 0xff, 0x1c, 0x1c, 0x1c, 0xff,
    0x16, 0x16, 0x16, 0xff, 0x16, 0x16, 0x16, 0xff, 0x16, 0x16, 0x16, 0xff, 0x16, 0x16, 0x16, 0xff,
    0x33, 0x33, 0x00, 0xff, 0x33, 0x33, 0x00, 0xff, 0x33, 0x33, 0x00, 0xff, 0x33, 0x33, 0x00, 0xff,
    0x33, 0x33, 0x00, 0xff, 0x11, 0x11, 0x11, 0xff, 0x0c, 0x0c, 0x0c, 0xff, 0x1c, 0x1c, 0x1c, 0xff,
];