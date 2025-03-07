use std::{env, path::PathBuf};

use image::DynamicImage;

use pdfium_render::prelude::*;

pub enum PDFQuality {
    High,
    Medium,
    Low,
}
fn initialize_pdfium() -> Box<dyn PdfiumLibraryBindings> {
    let out_path = env!("OUT_DIR");
    let pdfium_lib_path =
        PathBuf::from(&out_path).join(Pdfium::pdfium_platform_library_name());
    let bindings = Pdfium::bind_to_library(
        #[cfg(target_os = "android")]
        Pdfium::pdfium_platform_library_name_at_path("./"),
        #[cfg(not(target_os = "android"))]
        pdfium_lib_path.to_str().unwrap(),
    )
    .or_else(|_| Pdfium::bind_to_system_library());

    match bindings {
        Ok(binding) => binding,
        Err(e) => {
            panic!("{:?}", e)
        }
    }
}
pub fn render_preview_page(data: &[u8], quailty: PDFQuality) -> DynamicImage {
    let render_cfg = PdfBitmapConfig::new();
    let render_cfg = match quailty {
        PDFQuality::High => render_cfg.set_target_width(2000),
        PDFQuality::Medium => render_cfg,
        PDFQuality::Low => render_cfg.thumbnail(50),
    }
    .rotate_if_landscape(PdfBitmapRotation::Degrees90, true);
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

#[test]
fn test_multi_pdf_generate() {
    use tempdir::TempDir;
    let dir = TempDir::new("arklib_test").unwrap();
    let tmp_path = dir.path();
    println!("temp path: {}", tmp_path.display());
    for i in 0..2 {
        use std::{fs::File, io::Read};
        let mut pdf_reader = File::open("tests/test.pdf").unwrap();

        let mut bytes = Vec::new();
        pdf_reader.read_to_end(&mut bytes).unwrap();

        println!("Rendering {}", &i);
        let img = render_preview_page(bytes.as_slice(), PDFQuality::High);

        img.save(tmp_path.join(format!("test{}.png", &i)))
            .expect("cannot save image");
    }
}
