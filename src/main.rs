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
                "background" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "border" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "border.disabled" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "border.focused" => tm["ui.selection.primary"]["bg"].as_str().unwrap(),
                "border.selected" => tm["ui.selection.primary"]["bg"].as_str().unwrap(),
                "border.transparent" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "border.variant" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "conflict" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "conflict.background" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "conflict.border" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "created" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "created.background" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "created.border" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "deleted" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "deleted.background" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "deleted.border" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "drop_target.background" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "editor.active_line.background" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "editor.active_line_number" => tm["palette"]["ui-text"].as_str().unwrap(),
                "editor.active_wrap_guide" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "editor.background" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "editor.document_highlight.read_background" => {
                    tm["ui.cursor.primary"]["fg"].as_str().unwrap()
                }
                "editor.document_highlight.write_background" => {
                    tm["ui.cursor.primary"]["fg"].as_str().unwrap()
                }
                "editor.foreground" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "editor.gutter.background" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "editor.highlighted_line.background" => {
                    tm["ui.cursor.primary"]["fg"].as_str().unwrap()
                }
                "editor.invisible" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "editor.line_number" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "editor.subheader.background" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "editor.wrap_guide" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "element.active" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "element.background" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "element.disabled" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "element.hover" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "element.selected" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "elevated_surface.background" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "error" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "error.background" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "error.border" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "ghost_element.active" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "ghost_element.background" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "ghost_element.disabled" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "ghost_element.hover" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "ghost_element.selected" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "hidden" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "hidden.background" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "hidden.border" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "hint" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "hint.background" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "hint.border" => tm["ui.selection.primary"]["bg"].as_str().unwrap(),
                "icon" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "icon.accent" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "icon.disabled" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "icon.muted" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "icon.placeholder" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "ignored" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "ignored.background" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "ignored.border" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "info" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "info.background" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "info.border" => tm["ui.selection.primary"]["bg"].as_str().unwrap(),
                "link_text.hover" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "modified" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "modified.background" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "modified.border" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "pane.focused_border" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "panel.background" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "panel.focused_border" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "players" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "predictive" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "predictive.background" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "predictive.border" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "renamed" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "renamed.background" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "renamed.border" => tm["ui.selection.primary"]["bg"].as_str().unwrap(),
                "scrollbar.thumb.background" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "scrollbar.thumb.border" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "scrollbar.thumb.hover_background" => {
                    tm["ui.cursor.primary"]["fg"].as_str().unwrap()
                }
                "scrollbar.track.background" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "scrollbar.track.border" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "search.match_background" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "status_bar.background" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "success" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "success.background" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "success.border" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "surface.background" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "syntax" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "tab.active_background" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "tab.inactive_background" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "tab_bar.background" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "terminal.ansi.black" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "terminal.ansi.blue" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "terminal.ansi.bright_black" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "terminal.ansi.bright_blue" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "terminal.ansi.bright_cyan" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "terminal.ansi.bright_green" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "terminal.ansi.bright_magenta" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "terminal.ansi.bright_red" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "terminal.ansi.bright_white" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "terminal.ansi.bright_yellow" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "terminal.ansi.cyan" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "terminal.ansi.dim_black" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "terminal.ansi.dim_blue" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "terminal.ansi.dim_cyan" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "terminal.ansi.dim_green" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "terminal.ansi.dim_magenta" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "terminal.ansi.dim_red" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "terminal.ansi.dim_white" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "terminal.ansi.dim_yellow" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "terminal.ansi.green" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "terminal.ansi.magenta" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "terminal.ansi.red" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "terminal.ansi.white" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "terminal.ansi.yellow" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "terminal.background" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "terminal.bright_foreground" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "terminal.dim_foreground" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "terminal.foreground" => tm["ui.text"]["fg"].as_str().unwrap(),
                "text" => tm["palette"]["ui-text"].as_str().unwrap(),
                "text.accent" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "text.disabled" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "text.muted" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "text.placeholder" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "title_bar.background" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "toolbar.background" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "unreachable" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "unreachable.background" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "unreachable.border" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "warning" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "warning.background" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                "warning.border" => tm["ui.cursor.primary"]["fg"].as_str().unwrap(),
                _ => "",
            };
            (
                key.to_string(),
                serde_json::to_value(value).expect("couldnt convert"),
            )
        })
        .collect();

    println!("{:#?}", style);
    println!("{:#?}", new_style);
}
