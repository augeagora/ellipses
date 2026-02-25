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
    wav.load_mem(include_bytes!("../ellipses.wav")).unwrap();

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
    clearscreen::clear().expect("Failed to clear screen.");
    println!("Alô!");
    println!("This took me a tad bit of time to set up...");
    println!("Helix is super sweet honestly.");
    println!("They say you can rejoin online games BTW...");

    // Wait for input to close
    std::thread::sleep(std::time::Duration::from_millis(5000));
}
