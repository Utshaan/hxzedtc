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

    let colors = tm["palette"].as_table().unwrap().clone();

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
                "background" => to_serde_json_value(&tm["ui.selection"]["bg"], &colors),
                "border" => to_serde_json_value(&tm["ui.virtual.whitespace"]["fg"], &colors),
                "border.disabled" => {
                    to_serde_json_value(&tm["ui.virtual.indent-guide"]["fg"], &colors)
                }
                "border.focused" => to_serde_json_value(&tm["ui.selection.primary"]["bg"], &colors),
                "border.selected" => {
                    to_serde_json_value(&tm["ui.selection.primary"]["bg"], &colors)
                }
                "border.transparent" => to_serde_json_value("#00000000", &colors),
                "border.variant" => to_serde_json_value(&tm["ui.virtual.ruler"]["bg"], &colors),
                "conflict" => to_serde_json_value(&tm["comment"]["fg"], &colors),
                "conflict.background" => to_serde_json_value(&tm["diff.delta"], &colors),
                "conflict.border" => to_serde_json_value(&tm["diff.delta"], &colors),
                "created" => to_serde_json_value(&tm["comment"]["fg"], &colors),
                "created.background" => to_serde_json_value(&tm["diff.plus"], &colors),
                "created.border" => to_serde_json_value(&tm["diff.plus"], &colors),
                "deleted" => to_serde_json_value(&tm["comment"]["fg"], &colors),
                "deleted.background" => to_serde_json_value(&tm["diff.minus"], &colors),
                "deleted.border" => to_serde_json_value(&tm["diff.minus"], &colors),
                "drop_target.background" => to_serde_json_value(&tm["ui.linenr"]["fg"], &colors),
                "editor.active_line.background" => {
                    to_serde_json_value(&tm["ui.cursorline.primary"]["bg"], &colors)
                }
                "editor.active_line_number" => {
                    to_serde_json_value(&tm["ui.linenr.selected"]["fg"], &colors)
                }
                "editor.active_wrap_guide" => {
                    to_serde_json_value(&tm["ui.linenr.selected"]["fg"], &colors)
                }
                "editor.background" => to_serde_json_value(&tm["ui.background"]["bg"], &colors),
                "editor.document_highlight.read_background" => {
                    to_serde_json_value(&tm["ui.menu.selected"]["bg"], &colors)
                }
                "editor.document_highlight.write_background" => {
                    to_serde_json_value(&tm["ui.menu.scroll"]["bg"], &colors)
                }
                "editor.foreground" => to_serde_json_value(&tm["ui.text"]["fg"], &colors),
                "editor.gutter.background" => to_serde_json_value(&tm["ui.gutter"]["bg"], &colors),
                "editor.highlighted_line.background" => {
                    to_serde_json_value(&tm["ui.cursorline.primary"]["bg"], &colors)
                }
                "editor.invisible" => {
                    to_serde_json_value(&tm["ui.virtual.whitespace"]["fg"], &colors)
                }
                "editor.line_number" => to_serde_json_value(&tm["ui.linenr"]["fg"], &colors),
                "editor.subheader.background" => {
                    to_serde_json_value(&tm["ui.popup"]["bg"], &colors)
                }
                "editor.wrap_guide" => {
                    to_serde_json_value(&tm["ui.linenr.selected"]["fg"], &colors)
                }
                "element.active" => to_serde_json_value(&tm["ui.selection"]["bg"], &colors),
                "element.background" => to_serde_json_value(&tm["ui.menu"]["bg"], &colors),
                "element.disabled" => to_serde_json_value(&tm["ui.menu"]["bg"], &colors),
                "element.hover" => to_serde_json_value(&tm["ui.selection.primary"]["bg"], &colors),
                "element.selected" => to_serde_json_value(&tm["ui.highlight"]["bg"], &colors),
                "elevated_surface.background" => {
                    to_serde_json_value(&tm["ui.popup"]["bg"], &colors)
                }
                "error" => to_serde_json_value(&tm["error"]["fg"], &colors),
                "error.background" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"], &colors),
                "error.border" => {
                    to_serde_json_value(&tm["diagnostic.error"]["underline"]["color"], &colors)
                }
                "ghost_element.active" => {
                    to_serde_json_value(&tm["ui.menu.selected"]["bg"], &colors)
                }
                "ghost_element.background" => to_serde_json_value("#00000000", &colors),
                "ghost_element.disabled" => to_serde_json_value(&tm["ui.popup"]["bg"], &colors),
                "ghost_element.hover" => {
                    to_serde_json_value(&tm["ui.menu.selected"]["bg"], &colors)
                }
                "ghost_element.selected" => {
                    to_serde_json_value(&tm["ui.menu.selected"]["bg"], &colors)
                }
                "hidden" => to_serde_json_value(&tm["ui.menu.scroll"]["fg"], &colors),
                "hidden.background" => to_serde_json_value(&tm["ui.menu.scroll"]["bg"], &colors),
                "hidden.border" => to_serde_json_value(&tm["ui.statusline"]["bg"], &colors),
                "hint" => to_serde_json_value(&tm["ui.virtual.inlay-hint"]["fg"], &colors),
                "hint.background" => {
                    to_serde_json_value(&tm["ui.virtual.inlay-hint"]["fg"], &colors)
                }
                "hint.border" => to_serde_json_value(&tm["ui.virtual.inlay-hint"]["fg"], &colors),
                "icon" => to_serde_json_value(&tm["ui.menu"]["fg"], &colors),
                "icon.accent" => to_serde_json_value(&tm["ui.menu.selected"]["bg"], &colors),
                "icon.disabled" => to_serde_json_value(&tm["ui.virtual"]["fg"], &colors),
                "icon.muted" => to_serde_json_value(&tm["ui.menu.scroll"]["bg"], &colors),
                "icon.placeholder" => to_serde_json_value(&tm["ui.menu.scroll"]["bg"], &colors),
                "ignored" => to_serde_json_value(&tm["ui.virtual"]["fg"], &colors),
                "ignored.background" => to_serde_json_value(&tm["ui.virtual"]["fg"], &colors),
                "ignored.border" => to_serde_json_value(&tm["ui.menu.scroll"]["bg"], &colors),
                "info" => to_serde_json_value(&tm["info"]["fg"], &colors),
                "info.background" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"], &colors),
                "info.border" => {
                    to_serde_json_value(&tm["diagnostic.info"]["underline"]["color"], &colors)
                }
                "link_text.hover" => to_serde_json_value(&tm["ui.menu.selected"]["bg"], &colors),
                "modified" => to_serde_json_value(&tm["diff.delta"], &colors),
                "modified.background" => {
                    to_serde_json_value(&tm["ui.cursor.primary"]["fg"], &colors)
                }
                "modified.border" => {
                    to_serde_json_value(&tm["diagnostic.warning"]["underline"]["color"], &colors)
                }
                "pane.focused_border" => to_serde_json_value("null", &colors),
                "panel.background" => {
                    to_serde_json_value(&tm["ui.virtual.indent-guide"]["fg"], &colors)
                }
                "panel.focused_border" => to_serde_json_value("null", &colors),
                "players" => to_serde_json_value(&tm["ui.cursor.primary"]["fg"], &colors),
                "predictive" => to_serde_json_value(&tm["ui.virtual"]["fg"], &colors),
                "predictive.background" => {
                    to_serde_json_value(&tm["ui.cursor.primary"]["fg"], &colors)
                }
                "predictive.border" => {
                    to_serde_json_value(&tm["diagnostic.hint"]["underline"]["color"], &colors)
                }
                "renamed" => to_serde_json_value(&tm["info"]["fg"], &colors),
                "renamed.background" => {
                    to_serde_json_value(&tm["ui.cursor.primary"]["fg"], &colors)
                }
                "renamed.border" => {
                    to_serde_json_value(&tm["diagnostic.info"]["underline"]["color"], &colors)
                }
                "scrollbar.thumb.background" => {
                    to_serde_json_value(&tm["ui.menu.scroll"]["fg"], &colors)
                }
                "scrollbar.thumb.border" => {
                    to_serde_json_value(&tm["ui.virtual.indent-guide"]["fg"], &colors)
                }
                "scrollbar.thumb.hover_background" => {
                    to_serde_json_value(&tm["ui.virtual.indent-guide"]["fg"], &colors)
                }
                "scrollbar.track.background" => to_serde_json_value("#00000000", &colors),
                "scrollbar.track.border" => to_serde_json_value(&tm["ui.gutter"]["bg"], &colors),
                "search.match_background" => {
                    to_serde_json_value(&tm["ui.statusline.normal"]["bg"], &colors)
                }
                "status_bar.background" => to_serde_json_value(&tm["ui.background"]["bg"], &colors),
                "success" => to_serde_json_value(&tm["hint"]["fg"], &colors),
                "success.background" => {
                    to_serde_json_value(&tm["ui.cursor.primary"]["fg"], &colors)
                }
                "success.border" => {
                    to_serde_json_value(&tm["diagnostic.hint"]["underline"]["color"], &colors)
                }
                "surface.background" => to_serde_json_value(&tm["ui.menu"]["bg"], &colors),
                "syntax" => to_serde_json_value("ar", &colors),
                "tab.active_background" => {
                    to_serde_json_value(&tm["ui.statusline.inactive"]["bg"], &colors)
                }
                "tab.inactive_background" => {
                    to_serde_json_value(&tm["ui.statusline.normal"]["bg"], &colors)
                }
                "tab_bar.background" => to_serde_json_value(&tm["ui.statusline"]["bg"], &colors),
                "terminal.ansi.black" => to_serde_json_value(&colors["black"], &colors),
                "terminal.ansi.blue" => to_serde_json_value(&colors["blue"], &colors),
                "terminal.ansi.bright_black" => to_serde_json_value(&colors["black"], &colors),
                "terminal.ansi.bright_blue" => to_serde_json_value(&colors["black"], &colors),
                "terminal.ansi.bright_cyan" => to_serde_json_value(&colors["cyan"], &colors),
                "terminal.ansi.bright_green" => to_serde_json_value(&colors["green"], &colors),
                "terminal.ansi.bright_magenta" => to_serde_json_value(&colors["purple"], &colors),
                "terminal.ansi.bright_red" => to_serde_json_value(&colors["red"], &colors),
                "terminal.ansi.bright_white" => to_serde_json_value(&colors["white"], &colors),
                "terminal.ansi.bright_yellow" => to_serde_json_value(&colors["yellow"], &colors),
                "terminal.ansi.cyan" => to_serde_json_value(&colors["cyan"], &colors),
                "terminal.ansi.dim_black" => to_serde_json_value(&colors["black"], &colors),
                "terminal.ansi.dim_blue" => to_serde_json_value(&colors["blue"], &colors),
                "terminal.ansi.dim_cyan" => to_serde_json_value(&colors["cyan"], &colors),
                "terminal.ansi.dim_green" => to_serde_json_value(&colors["green"], &colors),
                "terminal.ansi.dim_magenta" => to_serde_json_value(&colors["purple"], &colors),
                "terminal.ansi.dim_red" => to_serde_json_value(&colors["red"], &colors),
                "terminal.ansi.dim_white" => to_serde_json_value(&colors["white"], &colors),
                "terminal.ansi.dim_yellow" => to_serde_json_value(&colors["yellow"], &colors),
                "terminal.ansi.green" => to_serde_json_value(&colors["green"], &colors),
                "terminal.ansi.magenta" => to_serde_json_value(&colors["purple"], &colors),
                "terminal.ansi.red" => to_serde_json_value(&colors["red"], &colors),
                "terminal.ansi.white" => to_serde_json_value(&colors["white"], &colors),
                "terminal.ansi.yellow" => to_serde_json_value(&colors["yellow"], &colors),
                "terminal.background" => to_serde_json_value(&colors["ui-text-reversed"], &colors),
                "terminal.bright_foreground" => to_serde_json_value(&colors["ui-text"], &colors),
                "terminal.dim_foreground" => to_serde_json_value(&colors["gray"], &colors),
                "terminal.foreground" => to_serde_json_value(&colors["light-gray"], &colors),
                "text" => to_serde_json_value(&tm["ui.text"]["fg"], &colors),
                "text.accent" => to_serde_json_value(&tm["info"]["fg"], &colors),
                "text.disabled" => to_serde_json_value(&tm["ui.virtual.inlay-hint"]["fg"], &colors),
                "text.muted" => to_serde_json_value(&tm["comment"]["fg"], &colors),
                "text.placeholder" => {
                    to_serde_json_value(&tm["ui.virtual.inlay-hint"]["fg"], &colors)
                }
                "title_bar.background" => to_serde_json_value(&tm["ui.background"]["bg"], &colors),
                "toolbar.background" => to_serde_json_value(&tm["ui.statusline"]["bg"], &colors),
                "unreachable" => to_serde_json_value(&tm["comment"]["fg"], &colors),
                "unreachable.background" => {
                    to_serde_json_value(&tm["ui.cursor.primary"]["fg"], &colors)
                }
                "unreachable.border" => to_serde_json_value(&tm["comment"]["fg"], &colors),
                "warning" => to_serde_json_value(&tm["warning"]["fg"], &colors),
                "warning.background" => {
                    to_serde_json_value(&tm["ui.cursor.primary"]["fg"], &colors)
                }
                "warning.border" => {
                    to_serde_json_value(&tm["diagnostic.warning"]["underline"]["color"], &colors)
                }
                _ => to_serde_json_value("", &colors),
            };
            (key.to_string(), value)
        })
        .collect();

    println!("{:#?}", new_style);
}

fn to_serde_json_value<T>(
    val: T,
    palette: &toml::map::Map<String, toml::Value>,
) -> serde_json::Value
where
    T: serde::ser::Serialize + ToString,
{
    match palette.get((&val.to_string()).trim_matches('"')) {
        Some(true_val) => {
            return serde_json::to_value(true_val).expect("Unable to convert");
        }
        None => {
            return serde_json::to_value((&val.to_string()).trim_matches('"'))
                .expect("Unable to convert");
        }
    }
}
