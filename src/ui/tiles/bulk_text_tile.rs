use gtk4::{Box, Label};

use crate::launcher::bulk_text_launcher::BulkText;
use crate::launcher::{Launcher, ResultItem};

use super::util::{AsyncOptions, TileBuilder};
use super::Tile;

impl Tile {
    pub fn bulk_text_tile_loader(
        launcher: &Launcher,
        keyword: &str,
        bulk_text: &BulkText,
    ) -> Option<(
        ResultItem,
        Option<Label>,
        Option<Label>,
        Option<AsyncOptions>,
        Box,
    )> {
        let builder = TileBuilder::new("/dev/skxxtz/sherlock/ui/bulk_text_tile.ui", 0, false);

        builder.category.set_text(&launcher.name);
        builder.icon.set_icon_name(Some(&bulk_text.icon));
        builder.icon.set_pixel_size(15);
        builder.content_title.set_text(keyword);
        builder.content_body.set_text("Loading...");
        builder.add_default_attrs(Some(&launcher.method), None, Some(keyword), None, None);

        let res = ResultItem {
            priority: launcher.priority as f32,
            row_item: builder.object,
        };
        return Some((
            res,
            Some(builder.content_title),
            Some(builder.content_body),
            None,
            builder.attrs,
        ));
    }
    pub fn bulk_text_tile(
        launcher: &Launcher,
        index: i32,
        keyword: &str,
        bulk_text: &BulkText,
    ) -> (i32, Vec<ResultItem>) {
        let builder = TileBuilder::new("/dev/skxxtz/sherlock/ui/bulk_text_tile.ui", index, false);

        builder.category.set_text(&launcher.name);
        builder.icon.set_icon_name(Some(&bulk_text.icon));
        builder.icon.set_pixel_size(15);
        builder.title.set_text(keyword);
        builder.add_default_attrs(Some(&launcher.method), None, Some(keyword), None, None);

        let res = ResultItem {
            priority: launcher.priority as f32,
            row_item: builder.object,
        };

        return (index + 1, vec![res]);
    }
}
