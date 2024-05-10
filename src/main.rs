use clap::Parser;
use image::Luma;
use qrcode::QrCode;
use std::io::Write;

#[derive(Parser)]
struct Cli {
    /// The input data to encode into the QR code
    input: String,

    /// The output file to save the QR code as a PNG image
    output: Option<String>,
}

fn main() {
    let args = Cli::parse();
    let input = args.input;
    let code = QrCode::new(input).unwrap();
    let string = code
        .render::<char>()
        .quiet_zone(false)
        .module_dimensions(2, 1)
        .build();
    if let Some(output) = args.output {
        // Render the QR code as a PNG image
        let image = code
            .render::<Luma<u8>>()
            .dark_color(image::Luma([0u8]))
            .light_color(image::Luma([255u8]))
            .build();
        image.save(output).unwrap();
    } else {
        // Write the ASCII QR code to stdout
        let stdout = std::io::stdout();
        let mut handle = stdout.lock();
        handle.write_all(string.as_bytes()).unwrap();
        handle.write_all(b"\n").unwrap();
        handle.flush().unwrap();
    }
}
