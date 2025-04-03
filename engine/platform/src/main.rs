mod platform;
mod platform_win32;


fn main() {
    let mut platform = platform_win32::PlatformState::new();

    platform_win32.platform_startup("Rusty Window 🚀", 100, 100, 800, 600);

    while platform_win32.platform_pump_messages() != 0 {
        // You can do game/app logic here
    }

    platform_win32.platform_shutdown();
}
