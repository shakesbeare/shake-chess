use macroquad::texture::Texture2D;
use resvg::{
    tiny_skia,
    usvg::{self, fontdb, Options, TreeParsing, TreeTextToPath},
};

pub async fn load_svg(filename: &str, square_size: f32) -> Texture2D {
    // resvg::Tree own all the required data and does not require
    // the input file, usvg::Tree or anything else.
    let scale_factor = square_size / 45.0;
    let rtree = {
        let opt = Options {
            resources_dir: std::fs::canonicalize(filename)
                .ok()
                .and_then(|p| p.parent().map(|p| p.to_path_buf())),
            ..Default::default()
        };

        let mut fontdb = fontdb::Database::new();
        fontdb.load_system_fonts();

        let svg_data = std::fs::read(filename).unwrap();
        let mut tree = usvg::Tree::from_data(&svg_data, &opt).unwrap();
        tree.convert_text(&fontdb);
        resvg::Tree::from_usvg(&tree)
    };

    let pixmap_size = rtree.size.to_int_size();
    let mut pixmap = tiny_skia::Pixmap::new(
        pixmap_size.width() * scale_factor as u32,
        pixmap_size.height() * scale_factor as u32,
    )
    .unwrap();
    let transform =
        tiny_skia::Transform::default().pre_scale(scale_factor, scale_factor);
    rtree.render(transform, &mut pixmap.as_mut());

    let bytes = pixmap.encode_png().unwrap();

    Texture2D::from_file_with_format(&bytes, Some(macroquad::prelude::ImageFormat::Png))
}
