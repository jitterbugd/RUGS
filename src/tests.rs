use std::time::SystemTime;
use crate::*;

pub struct TimingDebugger {
    _start_time: SystemTime
}
impl TimingDebugger {
    pub fn new() -> TimingDebugger {
        TimingDebugger {_start_time: SystemTime::now()}
    }

    pub fn breakpoint() {
        println!("[?] Press 'ENTER' when you're ready to continue.");
        io::stdin().read_line(&mut String::new()).unwrap();
    }

    pub fn checkpoint(&mut self, identifier: &str) {
        let duration = SystemTime::now().duration_since(self._start_time).expect("Time went backwards");
        self._start_time = SystemTime::now();

        println!("[{}] Duration since last checkpoint: {}", identifier.to_ascii_uppercase(), duration.as_millis());
        TimingDebugger::breakpoint();
    }
}

pub fn demonstration() {
    
    let img = image::open("artifacts/test.png").unwrap();

    let (width, height) = img.dimensions();
    println!("Width: {}, Height: {}", width, height);

    let rgba_image: ImageBuffer<image::Rgba<u8>, Vec<u8>> = img.to_rgba8();
    let raw_image_data = rgba_image.into_raw();

    let mut image_data: Vec<Rgba> = vec![];
    for chunk in raw_image_data.chunks(4) {
        image_data.push(Rgba::from_bytes(chunk));
    }

    let mut new_image = Image {
        width,
        height,
        image_data,
        lossy_compressed: false
    };

    if let Err(e) = new_image.lossy_compress(implementation::ComperssionAmnt::MIN) {
        panic!("[-] Lossy compression failed: {}", e)
    };

    std::fs::write("artifacts/output.rugs", new_image.deserialize()).unwrap();

    let file_data: Vec<u8> = std::fs::read("artifacts/output.rugs").unwrap();
    let new_image = match implementation::Image::serialize(file_data) {
        Ok(image) => image,
        Err(e) => panic!("[-] Unable to serialize image: {}", e),
    };
        
    let image_bytes = new_image.image_bytes();
    let image = ImageView::new(ImageInfo::rgba8(new_image.width, new_image.height), &image_bytes);
    let window = create_window("Image", Default::default()).unwrap();
    window.set_image("Image", image).unwrap();
    
    TimingDebugger::breakpoint();
}