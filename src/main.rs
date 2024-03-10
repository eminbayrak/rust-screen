mod capture;

fn main() {
    if let Err(err) = capture::capture_screenshot("screenshot.png") {
        eprintln!("Error: {}", err);
    } else {
        println!("Screenshot saved successfully!");
    }
}
