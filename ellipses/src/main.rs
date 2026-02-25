use owo_colors::OwoColorize;
use soloud::*;

fn main() {
    // Clear screen
    clearscreen::clear().expect("Failed to clear screen.");

    // AugeAgora
    println!("{}", "AugeAgora".bright_yellow());

    // Play sound
    let sl = Soloud::default().unwrap();
    let mut wav = audio::Wav::default();
    wav.load_mem(include_bytes!("../aa.wav")).unwrap();
    sl.play(&wav);
    while sl.voice_count() > 0 {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    // Load next sound
    wav.load_mem(include_bytes!("../dot.wav")).unwrap();

    // Clear screen
    clearscreen::clear().expect("Failed to clear screen.");

    // Little animated elipses :]
    println!("{}", ". ".red());
    sl.play(&wav);
    std::thread::sleep(std::time::Duration::from_millis(500));
    println!("{}", ". ".red());
    sl.play(&wav);
    std::thread::sleep(std::time::Duration::from_millis(500));
    println!("{}", ".".red());
    sl.play(&wav);
    while sl.voice_count() > 0 {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    std::thread::sleep(std::time::Duration::from_millis(500));

    // Message
    let m = "
        Alô!\n
        This is a little project I'm toying around with to familiarize myself\n
        with Rust and Helix. So far I've been quite fond of both. All the goofy\n
        sounds and music are made by me! I hope to create much more interesting\n
        and wackier stuff soon...";

    clearscreen::clear().expect("Failed to clear screen.");
    println!("{}", m.bright_cyan());

    // Load and play song
    wav.load_mem(include_bytes!("../ellipses.wav")).unwrap();
    sl.play(&wav);
    while sl.voice_count() > 0 {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    // Wait for input to close
    std::thread::sleep(std::time::Duration::from_millis(5000));
}
