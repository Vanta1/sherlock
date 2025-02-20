use std::collections::HashMap;

use gtk4::{self, prelude::*, Box, Builder, Image, Label, ListBoxRow};
use regex::Regex;

use super::util::insert_attrs;
use super::Tile;

impl Tile {
    pub fn clipboard_tile(
        index: i32,
        clipboard_content: &String,
        keyword: &String,
    ) -> (i32, Vec<ListBoxRow>) {
        let mut results: Vec<ListBoxRow> = Default::default();
        let mut index_ref = index;

        //TODO implement searchstring before clipboard content
        if clipboard_content.contains(keyword){
            let builder = Builder::from_resource("/dev/skxxtz/sherlock/ui/tile.ui");
            let holder: ListBoxRow = builder.object("holder").unwrap();
            let icon_obj: Image = builder.object("icon-name").unwrap();
            let title_obj: Label = builder.object("app-name").unwrap();
            let attr_holder: Box = builder.object("attrs-holder").unwrap();
            let launcher_type: Label = builder.object("launcher-type").unwrap();

            let mut name = "";
            let mut method = "";
            let mut icon = "";

            let known_pages = HashMap::from([
                ("google", "google"),
                ("chatgpt", "chat-gpt"),
                ("youtube", "sherlock-youtube")
            ]);

            // Check if clipboard content is a url:
            let re_url_check = r"^(https?:\/\/)?(www.)?([\da-z\.-]+)\.([a-z]{2,6})([\/\w\.-]*)*\/?$";
            let re = Regex::new(re_url_check).unwrap();
            if let Some(captures) = re.captures(clipboard_content){
                name = "Clipboard Web-Search";
                method = "web_launcher";
                let main_domain = captures.get(3).map_or("", |m| m.as_str());
                icon = known_pages.get(main_domain).map_or("google", |m| m);
                println!("{}", main_domain);

    
            }


            if name.is_empty() {
                launcher_type.set_visible(false);
            }

            launcher_type.set_text(name);
            title_obj.set_text(clipboard_content);
            icon_obj.set_icon_name(Some(&icon));

            let attrs: Vec<String> = vec![
                format!("{} | {}", "method", method),
                format!("{} | {}", "keyword", clipboard_content),
                format!("{} | {}", "engine", "plain"),
            ];

            insert_attrs(&attr_holder, attrs);
            index_ref += 1;
            results.push(holder);
        }

        return (index_ref + 1, results);
    }
}
