use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;

use log::debug;
use phf::phf_map;
use resvg;
use tiny_skia;
use usvg;

struct SvgData {
    text_x: i32,
    text_y: i32,
    font_size: i32,
}

macro_rules! svg_template {
    () => {
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 256 256\"><circle cx=\"128\" cy=\"128\" r=\"128\" fill=\"#{}\" /><text x=\"{}\" y=\"{}\" fill=\"{}\" font-size=\"{}px\" font-family=\"monospace\">{}</text></svg>"
    };
}

static SVG_OPTIONS: phf::Map<u32, SvgData> = phf_map! {
    1_u32 => SvgData {
        text_x: 100,
        text_y: 166,
        font_size: 96,
    },
    2_u32 => SvgData {
        text_x: 72,
        text_y: 166,
        font_size: 96,
    },
    3_u32 => SvgData {
        text_x: 40,
        text_y: 166,
        font_size: 96,
    },
    4_u32 => SvgData {
        text_x: 20,
        text_y: 160,
        font_size: 90,
    },
    5_u32 => SvgData {
        text_x: 20,
        text_y: 154,
        font_size: 72,
    },
};

const SVG_COLORS: &'static [&'static str] = &[
    "3F3B6C", "624F82", "624F82", "FD841F", "3E6D9C", "FFACC7", "B3FFAE", "82CD47",
];

pub async fn generate_image(name: &str) -> Vec<u8> {
    let mut s = DefaultHasher::new();
    name.hash(&mut s);
    let h: u64 = s.finish();
    let color = SVG_COLORS[(h as usize) % SVG_COLORS.len()];
    let text = name
        .split(' ')
        .filter(|s| !s.is_empty())
        .take(SVG_OPTIONS.len())
        .map(|s| s.chars().next().unwrap())
        .collect::<String>();
    let svg_data = SVG_OPTIONS.get(&(text.len() as u32)).unwrap();
    let svg = format!(
        svg_template!(),
        color, svg_data.text_x, svg_data.text_y, "white", svg_data.font_size, text
    );

    let mut opt = usvg::Options::default();
    opt.fontdb.load_system_fonts();
    // opt.fontdb.load_font_file(path)
    debug!("{}", svg);
    let rtree = usvg::Tree::from_str(&svg, &opt.to_ref()).unwrap();

    let pixmap_size = rtree.svg_node().size.to_screen_size();
    let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();
    resvg::render(
        &rtree,
        usvg::FitTo::Original,
        tiny_skia::Transform::default(),
        pixmap.as_mut(),
    )
    .unwrap();
    pixmap.encode_png().unwrap()
}
