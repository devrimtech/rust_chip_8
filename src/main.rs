use minifb::{Key, Window, WindowOptions};
// use bytes::{BytesMut, BufMut};

fn main() {
    const WIDTH: usize = 640;
    const HEIGHT: usize = 360;
    window(WIDTH, HEIGHT);
    let memory: &mut [u8];
}

fn window(width: usize, height: usize) {
    let mut buffer: Vec<u32> = vec![0; width * height];

    let mut window = Window::new(
        "Test - ESC to exit",
        width,
        height,
        WindowOptions::default(),
    )
    .unwrap();

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for i in buffer.iter_mut() {
            *i = 0; // write something more funny here!
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer, width, height).unwrap();
    }
}
