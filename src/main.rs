use serde_json;
use std::{fs::File, io::Read};
use toml;

fn main() {
    let mut toml_handler = File::open("zed_onedark.toml").expect("unable to open toml file");
    let json_handler = File::open("zed_onedark.json").expect("Unable to create json file");
    let mut toml_as_string = String::new();
    toml_handler
        .read_to_string(&mut toml_as_string)
        .expect("Error reading file");

    let toml = toml_as_string
        .parse::<toml::Table>()
        .expect("couldn't parse to toml");

    let tm = toml::map::Map::from(toml);

    let _colors = tm["palette"].as_table().unwrap().clone();

    let js: serde_json::Value = serde_json::from_reader(json_handler).unwrap();
    let style = js.as_object().unwrap()["themes"].as_array().unwrap()[0]
        .as_object()
        .unwrap()["style"]
        .as_object()
        .unwrap();

    let new_style: serde_json::Map<String, serde_json::Value> = style
        .keys()
        .map(|key| {
            let value = match key.as_str() {
                "background" => to_serde_json_value(&tm["ui.selection"]["bg"]),
                "border" => to_serde_json_value(&tm["ui.virtual.whitespace"]["fg"]),
                "border.disabled" => to_serde_json_value(&tm["ui.virtual.indent-guide"]["fg"]),
                "border.focused" => to_serde_json_value(&tm["ui.selection.primary"]["fg"]),
                "border.selected" => to_serde_json_value(&tm["ui.selection.primary"]["bg"]),
                "border.transparent" => to_serde_json_value("null"),
                "border.variant" => to_serde_json_value(&tm["ui.virtual.ruler"]["bg"]),
                "conflict" => to_serde_json_value(&tm["comment"]["fg"]),
                "conflict.background" => to_serde_json_value(&tm["diff.delta"]),
                "conflict.border" => to_serde_json_value(&tm["diff.delta"]),
                "created" => to_serde_json_value(&tm["comment"]["fg"]),
                "created.background" => to_serde_json_value(&tm["diff.plus"]),
                "created.border" => to_serde_json_value(&tm["diff.plus"]),
                "deleted" => to_serde_json_value(&tm["comment"]["fg"]),
                "deleted.background" => to_serde_json_value(&tm["diff.minus"]),
                "deleted.border" => to_serde_json_value(&tm["diff.minus"]),
                "drop_target.background" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"]),
                "editor.active_line.background" => {
                    to_serde_json_value(&tm["ui.cursor.primary"]["fg"])
                }
                "editor.active_line_number" => to_serde_json_value(&tm["palette"]["ui-text"]),
                "editor.active_wrap_guide" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"]),
                "editor.background" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"]),
                "editor.document_highlight.read_background" => {
                    to_serde_json_value(&tm["ui.cursor.primary"]["fg"])
                }
                "editor.document_highlight.write_background" => {
                    to_serde_json_value(&tm["ui.cursor.primary"]["fg"])
                }
                "editor.foreground" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"]),
                "editor.gutter.background" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"]),
                "editor.highlighted_line.background" => {
                    to_serde_json_value(&tm["ui.cursor.primary"]["fg"])
                }
                "editor.invisible" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"]),
                "editor.line_number" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"]),
                "editor.subheader.background" => {
                    to_serde_json_value(&tm["ui.cursor.primary"]["fg"])
                }
                "editor.wrap_guide" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"]),
                "element.active" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"]),
                "element.background" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"]),
                "element.disabled" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"]),
                "element.hover" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"]),
                "element.selected" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"]),
                "elevated_surface.background" => to_serde_json_value(&tm["ui.popup"]["bg"]),
                "error" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"]),
                "error.background" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"]),
                "error.border" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"]),
                "ghost_element.active" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"]),
                "ghost_element.background" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"]),
                "ghost_element.disabled" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"]),
                "ghost_element.hover" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"]),
                "ghost_element.selected" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"]),
                "hidden" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"]),
                "hidden.background" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"]),
                "hidden.border" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"]),
                "hint" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"]),
                "hint.background" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"]),
                "hint.border" => to_serde_json_value(&tm["ui.selection.primary"]["bg"]),
                "icon" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"]),
                "icon.accent" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"]),
                "icon.disabled" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"]),
                "icon.muted" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"]),
                "icon.placeholder" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"]),
                "ignored" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"]),
                "ignored.background" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"]),
                "ignored.border" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"]),
                "info" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"]),
                "info.background" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"]),
                "info.border" => to_serde_json_value(&tm["ui.selection.primary"]["bg"]),
                "link_text.hover" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"]),
                "modified" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"]),
                "modified.background" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"]),
                "modified.border" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"]),
                "pane.focused_border" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"]),
                "panel.background" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"]),
                "panel.focused_border" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"]),
                "players" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"]),
                "predictive" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"]),
                "predictive.background" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"]),
                "predictive.border" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"]),
                "renamed" => to_serde_json_value(&tm["info"]["fg"]),
                "renamed.background" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"]),
                "renamed.border" => {
                    to_serde_json_value(&tm["diagnostic.info"]["underline"]["color"])
                }
                "scrollbar.thumb.background" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"]),
                "scrollbar.thumb.border" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"]),
                "scrollbar.thumb.hover_background" => {
                    to_serde_json_value(&tm["ui.cursor.primary"]["fg"])
                }
                "scrollbar.track.background" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"]),
                "scrollbar.track.border" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"]),
                "search.match_background" => to_serde_json_value(&tm["ui.statusline.normal"]["bg"]),
                "status_bar.background" => to_serde_json_value(&tm["ui.background"]["bg"]),
                "success" => to_serde_json_value(&tm["hint"]["fg"]),
                "success.background" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"]),
                "success.border" => {
                    to_serde_json_value(&tm["diagnostic.hint"]["underline"]["color"])
                }
                "surface.background" => to_serde_json_value(&tm["ui.menu"]["bg"]),
                "syntax" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"]),
                "tab.active_background" => to_serde_json_value(&tm["ui.statusline.inactive"]["bg"]),
                "tab.inactive_background" => to_serde_json_value(&tm["ui.statusline.normal"]["bg"]),
                "tab_bar.background" => to_serde_json_value(&tm["ui.statusline"]["bg"]),
                "terminal.ansi.black" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"]),
                "terminal.ansi.blue" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"]),
                "terminal.ansi.bright_black" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"]),
                "terminal.ansi.bright_blue" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"]),
                "terminal.ansi.bright_cyan" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"]),
                "terminal.ansi.bright_green" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"]),
                "terminal.ansi.bright_magenta" => {
                    to_serde_json_value(&tm["ui.cursor.primary"]["fg"])
                }
                "terminal.ansi.bright_red" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"]),
                "terminal.ansi.bright_white" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"]),
                "terminal.ansi.bright_yellow" => {
                    to_serde_json_value(&tm["ui.cursor.primary"]["fg"])
                }
                "terminal.ansi.cyan" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"]),
                "terminal.ansi.dim_black" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"]),
                "terminal.ansi.dim_blue" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"]),
                "terminal.ansi.dim_cyan" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"]),
                "terminal.ansi.dim_green" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"]),
                "terminal.ansi.dim_magenta" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"]),
                "terminal.ansi.dim_red" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"]),
                "terminal.ansi.dim_white" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"]),
                "terminal.ansi.dim_yellow" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"]),
                "terminal.ansi.green" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"]),
                "terminal.ansi.magenta" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"]),
                "terminal.ansi.red" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"]),
                "terminal.ansi.white" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"]),
                "terminal.ansi.yellow" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"]),
                "terminal.background" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"]),
                "terminal.bright_foreground" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"]),
                "terminal.dim_foreground" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"]),
                "terminal.foreground" => to_serde_json_value(&tm["ui.text"]["fg"]),
                "text" => to_serde_json_value(&tm["ui.text"]["fg"]),
                "text.accent" => to_serde_json_value(&tm["info"]["fg"]),
                "text.disabled" => to_serde_json_value(&tm["ui.virtual.inlay-hint"]["fg"]),
                "text.muted" => to_serde_json_value(&tm["comment"]["fg"]),
                "text.placeholder" => to_serde_json_value(&tm["ui.virtual.inlay-hint"]["fg"]),
                "title_bar.background" => to_serde_json_value(&tm["ui.background"]["bg"]),
                "toolbar.background" => to_serde_json_value(&tm["ui.statusline"]["bg"]),
                "unreachable" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"]),
                "unreachable.background" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"]),
                "unreachable.border" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"]),
                "warning" => to_serde_json_value(&tm["warning"]["fg"]),
                "warning.background" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"]),
                "warning.border" => {
                    to_serde_json_value(&tm["diagnostic.warning"]["underline"]["color"])
                }
                _ => to_serde_json_value(""),
            };
            (key.to_string(), value)
        })
        .collect();

    // println!("{:#?}", style);
    println!("{:#?}", new_style);
}

fn to_serde_json_value<T: serde::ser::Serialize>(val: T) -> serde_json::Value {
    serde_json::to_value(val).expect("Unable to convert")
}
