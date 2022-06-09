use std::{
    env,
    path::PathBuf,
    sync::{Arc, Mutex, Once},
};

use image::DynamicImage;

use pdfium_render::prelude::*;

lazy_static! {
    static ref EVENT_QUEUE: Mutex<bool> = Mutex::new(false);
    static ref ONCE: Once = Once::new();
}

pub enum PDFQuailty {
    High,
    Medium,
    Low,
}
fn initialize_pdfium() -> Box<dyn PdfiumLibraryBindings> {
    let out_path = env::var("OUT_DIR").unwrap_or("".to_string());

    let pdfium_libpath =
        PathBuf::from(&out_path).join(Pdfium::pdfium_platform_library_name());
    let bindings = Pdfium::bind_to_library(pdfium_libpath.display())
        .or_else(|_| Pdfium::bind_to_system_library())
        .unwrap();
    bindings
}
pub fn render_preview_page(
    data: &[u8],
    quailty: PDFQuailty,
    // font_path: Option<String>,
) -> DynamicImage {
    // EVENT_QUEUE.lock().unwrap();

    let render_cfg = PdfBitmapConfig::new();
    Pdfium::new(initialize_pdfium())
        .load_pdf_from_bytes(data, None)
        .unwrap()
        .pages()
        .get(0)
        .unwrap()
        .get_bitmap_with_config(&render_cfg)
        .unwrap()
        .as_image()
}

// #[test]
// fn test_pdf_generate() {
//     use std::{fs::File, io::Read};
//     let mut pdf_reader = File::open("tests/test.pdf").unwrap();

//     let mut bytes = Vec::new();
//     pdf_reader.read_to_end(&mut bytes).unwrap();

//     let img = render_preview_page(bytes.as_slice(), PDFQuailty::Low, None);
//     img.save("tests/test.png")
//         .expect("cannot save image");
// }

#[test]
fn test_multi_pdf_generate() {
    for i in 0..2 {
        use std::{fs::File, io::Read};
        let mut pdf_reader = File::open("tests/test.pdf").unwrap();

        let mut bytes = Vec::new();
        pdf_reader.read_to_end(&mut bytes).unwrap();
        println!("Rendering {}", &i);
        let img = render_preview_page(bytes.as_slice(), PDFQuailty::Low, None);
        img.save(format!("tests/test{}.png", &i))
            .expect("cannot save image");
    }
}