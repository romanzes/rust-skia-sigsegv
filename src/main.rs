extern crate skia_safe;

use skia_safe::textlayout::{ParagraphStyle, TextStyle, FontCollection, ParagraphBuilder};
use skia_safe::FontMgr;

fn main() {
    let mut style = ParagraphStyle::new();
    style.set_text_style(&TextStyle::new());
    let mut font_collection = FontCollection::new();
    font_collection.set_default_font_manager(FontMgr::default(), None);
    let mut paragraph_builder = ParagraphBuilder::new(&style, font_collection);
    paragraph_builder.add_text("Lorem ipsum dolor sit amet\n");
    let mut paragraph = paragraph_builder.build();
    paragraph.layout(100.0);

    let line_metrics = &paragraph.get_line_metrics()[0];
    line_metrics.get_style_metrics(line_metrics.start_index..line_metrics.end_index);
}
