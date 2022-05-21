use crate::menu;
use druid::text::{AttributesAdder, RichText, RichTextBuilder};
use druid::widget::prelude::*;
use druid::widget::{Controller, LineBreaking, RawLabel, Scroll, Split, TextBox};
use druid::{
    Color, Command, DelegateCtx, FontFamily, FontStyle, FontWeight, Handled, Lens, Selector,
    Target, Widget, WidgetExt,
};
use menu::GeneralState;
use pulldown_cmark::{Event as ParseEvent, Options, Parser, Tag};

/// Size of the spacing between lines.
const SPACER_SIZE: f64 = 8.0;

/// Colors of the quotes in Markdown.
const BLOCKQUOTE_COLOR: Color = Color::grey8(0x88);

/// Colors of the links in Markdown.
const LINK_COLOR: Color = Color::rgb8(0, 0, 0xEE);

/// Command for opening links in markdown.
const OPEN_LINK: Selector<String> = Selector::new("druid-example.open-link");

pub struct RichTextRebuilder;

impl<W: Widget<GeneralState>> Controller<GeneralState, W> for RichTextRebuilder {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut GeneralState,
        env: &Env,
    ) {
        let pre_data = data.raw.to_owned();
        child.event(ctx, event, data, env);
        if !data.raw.same(&pre_data) {
            data.rendered = rebuild_rendered_text(&data.raw);
        }
    }
}

pub fn rebuild_rendered_text(text: &str) -> RichText {
    let mut current_pos = 0;
    let mut builder = RichTextBuilder::new();
    let mut tag_stack = Vec::new();

    let parser = Parser::new_ext(text, Options::ENABLE_STRIKETHROUGH);

    for event in parser {
        match event {
            ParseEvent::Start(tag) => {
                tag_stack.push((current_pos, tag));
            }
            ParseEvent::Text(txt) => {
                builder.push(&txt);
                current_pos += txt.len();
            }
            ParseEvent::End(end_tag) => {
                let (start_off, tag) = tag_stack
                    .pop()
                    .expect("Parser does not return unbalanced tags");
                assert_eq!(end_tag, tag, "mismatched tags?");
                add_attribute_for_tag(
                    &tag,
                    builder.add_attributes_for_range(start_off..current_pos),
                );
                if add_newline_after_tag(&tag) {
                    builder.push("\n\n");
                    current_pos += 2;
                }
            }
            ParseEvent::Code(txt) => {
                builder.push(&txt).font_family(FontFamily::MONOSPACE);
                current_pos += txt.len();
            }
            ParseEvent::Html(txt) => {
                builder
                    .push(&txt)
                    .font_family(FontFamily::MONOSPACE)
                    .text_color(BLOCKQUOTE_COLOR);
                current_pos += txt.len();
            }
            ParseEvent::HardBreak => {
                builder.push("\n\n");
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
        _ => (),
    }
}

/// This function determines if there is a need to add a new line after a tag.
/// it does this by comparing the tag with its enumarators. if the tag has an emphasis, is strongi has strikethroug, or is a link, it will return true.
fn add_newline_after_tag(tag: &Tag) -> bool {
    !matches! {
        tag,
        Tag::Emphasis | Tag::Strong | Tag::Strikethrough | Tag::Link(..)
    }
}
