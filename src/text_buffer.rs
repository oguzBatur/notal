use crate::{DynamicTextBufferTab, GeneralState, menu, Vector};
use druid::text::{AttributesAdder, RichText, RichTextBuilder};
use druid::widget::prelude::*;
use druid::widget::Controller;
use druid::{ Color, Data, FontFamily, FontStyle, FontWeight, Lens, Selector, Widget};
use pulldown_cmark::{Event as ParseEvent, Options, Parser, Tag};

//* Deneme
//TODO - Solve the paragrapgh break not working bug.

/// Size of the spacing between lines.
const _SPACER_SIZE: f64 = 8.0;


const OPEN_LINK: Selector<String> = Selector::new("druid-example.open-link");
/// Colors of the quotes in Markdown.
const BLOCKQUOTE_COLOR: Color = Color::grey8(0x88);
/// Colors of the links in Markdown.
const LINK_COLOR: Color = Color::rgb8(0, 0, 0xEE);

/// A struct that is used to re render a plain text to rich text.
#[derive(Clone)]
pub struct RichTextRebuilder;

/// Text Buffer Data that will be used in tab instancing.
#[derive(Clone, Data, Lens)]
pub struct TextBufferData {
    pub file_name: String,
    pub file_path: String,
    pub raw: String,
    pub rendered: RichText,
    pub is_live_preview_open: bool,
    pub key: usize
}
impl TextBufferData {
    fn open_preview(&mut self) {
        if !self.is_live_preview_open {
            self.is_live_preview_open = true;
        }
    }
    fn close_preview(&mut self) {
        if self.is_live_preview_open {
            self.is_live_preview_open = false;
        }
    }
}



impl<W: Widget<TextBufferData>> Controller<TextBufferData, W> for RichTextRebuilder {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut TextBufferData,
        env: &Env,
    ) {
        let pre_data = data.raw.to_owned();
        // Checks the keyboard event.
        child.event(ctx, event, data, env);
        if !data.raw.same(&pre_data) {
            data.rendered = rebuild_rendered_text(&data.raw);
            // println!("The rendered text: {:?}", &data.rendered);
        }
    }
}

pub fn rebuild_rendered_text(text: &str) -> RichText {
    let mut current_pos = 0;
    let mut builder = RichTextBuilder::new();
    let mut tag_stack = Vec::new();
    // println!("This is the raw txt: {}", text);

    let parser = Parser::new_ext(text, Options::ENABLE_STRIKETHROUGH);
    for event in parser {
        match event {
            ParseEvent::Start(tag) => {
                // println!(
                //     "Pushing to tag stack\nCurrent Pos: {}\nThe Tag: {:?}",
                //     current_pos, tag
                // );
                tag_stack.push((current_pos, tag));
                // println!("Start event");
            }
            ParseEvent::Text(txt) => {
                builder.push(&txt);
                current_pos += txt.len();
            }
            //* Iterator kullanıldığı için, her daim 3 event minimum dönüyor bu eventler sırası ile start, text, end eventleri. */
            ParseEvent::End(end_tag) => {
                // Starting position and the starting tag from tag_stack.
                // println!("End event.");
                let (start_off, tag) = tag_stack
                    .pop()
                    .expect("Parser does not return unbalanced tags");
                println!("The start_off: {}", start_off);
                assert_eq!(end_tag, tag, "mismatched tags?");
                add_attribute_for_tag(
                    &tag,
                    builder.add_attributes_for_range(start_off..current_pos),
                );

                if add_newline_after_tag(&tag) {
                    builder.push("\n");
                    current_pos += 1;
                }
            }
            ParseEvent::Code(txt) => {
                builder.push(&txt).font_family(FontFamily::MONOSPACE);
                current_pos += txt.len();
                println!("Code Event.");
            }
            ParseEvent::Html(txt) => {
                println!("HTML event.");
                builder
                    .push(&txt)
                    .font_family(FontFamily::MONOSPACE)
                    .text_color(BLOCKQUOTE_COLOR);
                current_pos += txt.len();
            }
            ParseEvent::HardBreak => {
                println!("HardBreak Event");
                builder.push("\n");
                current_pos += 2;
            }
            _ => (),
        }
    }
    builder.build()
}

fn add_attribute_for_tag(tag: &Tag, mut attrs: AttributesAdder) {
    match tag {
        Tag::Heading(lvl) => {
            let font_size = match lvl {
                1 => 38.,
                2 => 32.0,
                3 => 26.0,
                4 => 20.0,
                5 => 16.0,
                _ => 12.0,
            };
            attrs.size(font_size).weight(FontWeight::BOLD);
        }
        Tag::BlockQuote => {
            attrs.style(FontStyle::Italic).text_color(BLOCKQUOTE_COLOR);
        }
        Tag::CodeBlock(_) => {
            attrs.font_family(FontFamily::MONOSPACE);
        }
        Tag::Emphasis => {
            attrs.style(FontStyle::Italic);
        }
        Tag::Strong => {
            attrs.weight(FontWeight::BOLD);
        }
        Tag::Link(_link_ty, target, _title) => {
            attrs
                .underline(true)
                .text_color(LINK_COLOR)
                .link(OPEN_LINK.with(target.to_string()));
        }
        Tag::Paragraph=> {
            attrs.text_color(Color::BLUE);
        }
        _ => (),
    }
}

/// This function determines if there is a need to add a new line after a tag.
/// it does this by comparing the tag with its enumarators. if the tag has an emphasis, is strong has strikethrough, or is a link, it will return false.
fn add_newline_after_tag(tag: &Tag) -> bool {
    !matches! {
        tag,
        Tag::Emphasis | Tag::Strong | Tag::Strikethrough | Tag::Link(..)
    }
}
