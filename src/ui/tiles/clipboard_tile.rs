use gtk4::{builders, prelude::*, ListBoxRow};
use regex::Regex;
use std::collections::HashMap;
use meval::eval_str;

use crate::launcher::Launcher;

use super::util::{get_builder, insert_attrs, TileBuilder};
use super::{calc_tile, Tile};

impl Tile {
    pub fn clipboard_tile(
        launcher: &Launcher,
        index: i32,
        clipboard_content: &String,
        keyword: &String,
    ) -> (i32, Vec<ListBoxRow>) {
        let mut results: Vec<ListBoxRow> = Default::default();
        let mut is_valid: i32 = 0;

        //TODO implement searchstring before clipboard content
        if clipboard_content.contains(keyword) {
            let mut builder = TileBuilder::new();
            let mut name = "";
            let mut method = "";
            let mut icon = "";

            let known_pages = HashMap::from([
                ("google", "google"),
                ("chatgpt", "chat-gpt"),
                ("youtube", "sherlock-youtube"),
            ]);

            // Check if clipboard content is a url:
            let re_url_check =
                r"^(https?:\/\/)?(www.)?([\da-z\.-]+)\.([a-z]{2,6})([\/\w\.-]*)*\/?$";
            let re = Regex::new(re_url_check).unwrap();
            if let Some(captures) = re.captures(clipboard_content) {
                builder = get_builder("/dev/skxxtz/sherlock/ui/tile.ui", index, true);
                is_valid = 1;
                name = "Clipboard Web-Search";
                method = "web_launcher";
                let main_domain = captures.get(3).map_or("", |m| m.as_str());
                icon = known_pages.get(main_domain).map_or("google", |m| m);
            } else {

                if let Ok(r) = eval_str(clipboard_content){
                    return Tile::calc_tile(launcher, index, clipboard_content, Some(r));
                }
            }

            if is_valid == 1 {
                if name.is_empty() {
                    builder.category.set_visible(false);
                }

                builder.category.set_text(name);
                builder.title.set_text(clipboard_content);
                builder.icon.set_icon_name(Some(&icon));

                let attrs: Vec<(&str, &str)> = vec![
                    ("method", method),
                    ("keyword", clipboard_content.as_str()),
                    ("engine", "plain"),
                ];

                insert_attrs(&builder.attrs, attrs);
                results.push(builder.object);
            }
        }

        return (index + is_valid, results);
    }
}
