use serde_json;
use std::{fs::File, io::Read};
use toml;

struct ZedTheme {
    name: String,
    author: Author,
    themes: Vec<Theme>,
}

struct Author {
    name: String,
    github_username: Option<String>,
}

enum Appearance {
    Light,
    Dark,
}

struct Theme {
    name: String,
    appearance: Appearance,
    style: Style,
}

struct Style {
    background: Option<String>,
    border: Option<String>,
    border_disabled: Option<String>,
    border_focused: Option<String>,
    border_selected: Option<String>,
    border_transparent: Option<String>,
    border_variant: Option<String>,
    conflict: Option<String>,
    conflict_background: Option<String>,
    conflict_border: Option<String>,
    created: Option<String>,
    created_background: Option<String>,
    created_border: Option<String>,
    deleted: Option<String>,
    deleted_background: Option<String>,
    deleted_border: Option<String>,
    drop_target_background: Option<String>,
    editor_active_line_background: Option<String>,
    editor_active_line_number: Option<String>,
    editor_active_wrap_guide: Option<String>,
    editor_background: Option<String>,
    editor_document_highlight_read_background: Option<String>,
    editor_document_highlight_write_background: Option<String>,
    editor_foreground: Option<String>,
    editor_gutter_background: Option<String>,
    editor_highlighted_line_background: Option<String>,
    editor_invisible: Option<String>,
    editor_line_number: Option<String>,
    editor_subheader_background: Option<String>,
    editor_wrap_guide: Option<String>,
    element_active: Option<String>,
    element_background: Option<String>,
    element_disabled: Option<String>,
    element_hover: Option<String>,
    element_selected: Option<String>,
    elevated_surface_background: Option<String>,
    error: Option<String>,
    error_background: Option<String>,
    error_border: Option<String>,
    ghost_element_active: Option<String>,
    ghost_element_background: Option<String>,
    ghost_element_disabled: Option<String>,
    ghost_element_hover: Option<String>,
    ghost_element_selected: Option<String>,
    hidden: Option<String>,
    hidden_background: Option<String>,
    hidden_border: Option<String>,
    hint: Option<String>,
    hint_background: Option<String>,
    hint_border: Option<String>,
    icon: Option<String>,
    icon_accent: Option<String>,
    icon_disabled: Option<String>,
    icon_muted: Option<String>,
    icon_placeholder: Option<String>,
    ignored: Option<String>,
    ignored_background: Option<String>,
    ignored_border: Option<String>,
    info: Option<String>,
    info_background: Option<String>,
    info_border: Option<String>,
    link_text_hover: Option<String>,
    modified: Option<String>,
    modified_background: Option<String>,
    modified_border: Option<String>,
    pane_focused_border: Option<String>,
    panel_background: Option<String>,
    panel_focused_border: Option<String>,
    players: Option<Vec<Player>>,
    predictive: Option<String>,
    predictive_background: Option<String>,
    predictive_border: Option<String>,
    renamed: Option<String>,
    renamed_background: Option<String>,
    renamed_border: Option<String>,
    scrollbar_thumb_border: Option<String>,
    scrollbar_thumb_hover_background: Option<String>,
    scrollbar_track_background: Option<String>,
    scrollbar_track_border: Option<String>,
    scrollbar_thumb_background: Option<String>,
    search_match_background: Option<String>,
    status_bar_background: Option<String>,
    success: Option<String>,
    success_background: Option<String>,
    success_border: Option<String>,
    surface_background: Option<String>,
    syntax: Option<Vec<SyntaxObject>>,
    tab_active_background: Option<String>,
    tab_inactive_background: Option<String>,
    tab_bar_background: Option<String>,
    terminal_ansi_black: Option<String>,
    terminal_ansi_blue: Option<String>,
    terminal_ansi_bright_black: Option<String>,
    terminal_ansi_bright_blue: Option<String>,
    terminal_ansi_bright_cyan: Option<String>,
    terminal_ansi_bright_green: Option<String>,
    terminal_ansi_bright_magenta: Option<String>,
    terminal_ansi_bright_red: Option<String>,
    terminal_ansi_bright_white: Option<String>,
    terminal_ansi_bright_yellow: Option<String>,
    terminal_ansi_cyan: Option<String>,
    terminal_ansi_dim_black: Option<String>,
    terminal_ansi_dim_blue: Option<String>,
    terminal_ansi_dim_cyan: Option<String>,
    terminal_ansi_dim_green: Option<String>,
    terminal_ansi_dim_magenta: Option<String>,
    terminal_ansi_dim_red: Option<String>,
    terminal_ansi_dim_white: Option<String>,
    terminal_ansi_dim_yellow: Option<String>,
    terminal_ansi_green: Option<String>,
    terminal_ansi_magenta: Option<String>,
    terminal_ansi_red: Option<String>,
    terminal_ansi_white: Option<String>,
    terminal_ansi_yellow: Option<String>,
    terminal_background: Option<String>,
    terminal_bright_foreground: Option<String>,
    terminal_dim_foreground: Option<String>,
    terminal_foreground: Option<String>,
    text: Option<String>,
    text_accent: Option<String>,
    text_disabled: Option<String>,
    text_muted: Option<String>,
    text_placeholder: Option<String>,
    title_bar_background: Option<String>,
    toolbar_background: Option<String>,
    unreachable: Option<String>,
    unreachable_background: Option<String>,
    unreachable_border: Option<String>,
    warning: Option<String>,
    warning_background: Option<String>,
    warning_border: Option<String>,
}

struct Player {
    background: String,
    cursor: String,
    selection: String,
}

enum FontStyle {
    Normal,
    Italic,
    Oblique,
}

enum FontWeight {
    A = 100,
    B = 200,
    C = 300,
    D = 400,
    E = 500,
    F = 600,
    G = 700,
    H = 800,
    I = 900,
}

struct SyntaxObject {
    color: Option<String>,
    font_style: Option<FontStyle>,
    font_weight: Option<FontWeight>,
}

// struct HxTheme {
// attribute: Option<String>,
// comment: Option<String>,
// constant: Option<String>,
// constant_numeric: Option<String>,
// constant_builtin: Option<String>,
// constant_builtin_boolean: Option<String>,
// constant_character_escape: Option<String>,
// constructor: Option<String>,
// function: Option<String>,
// function_builtin: Option<String>,
// function_method: Option<String>,
// function_macro: Option<String>,
// keyword: Option<String>,
// label: Option<String>,
// namespace: Option<String>,
// operator: Option<String>,
// puncuation: Option<String>,
// special: Option<String>,
// string: Option<String>,
// type: Option<String>,
// variable_builtin: Option<String>,
// variable_parameter: Option<String>,
// variable_other_member: Option<String>,
// markup_heading: Option<String>,
// markup_raw_inline: Option<String>,
// markup_bold: Option<String>,
// markup_italic: Option<String>,
// markup_strikethrough: Option<String>,
// markup_list: Option<String>,
// markup_quote: Option<String>,
// markup_link_url: Option<String>,
// markup_link_text: Option<String>,
// diff_plus: Option<String>,
// diff_delta: Option<String>,
// diff_minus: Option<String>,
// diagnostic_info: Option<String>,
// diagnostic_hint: Option<String>,
// diagnostic_warning: Option<String>,
// diagnostic_error: Option<String>,
// info: Option<String>,
// hint: Option<String>,
// warning: Option<String>,
// error: Option<String>,
// ui_background: Option<String>,
// ui_gutter: Option<String>,
// ui_virtual: Option<String>,
// ui_virtual_indent_guide: Option<String>,
// ui_virtual_whitespace: Option<String>,
// ui_virtual_ruler: Option<String>,
// ui_virtual_inlay_hint: Option<String>,
// ui_cursor: Option<String>,
// ui_cursor_primary: Option<String>,
// ui_cursor_match: Option<String>,
// ui_cursor_insert: Option<String>,
// ui_selection: Option<String>,
// ui_selection_primary: Option<String>,
// ui_cursorline_primary: Option<String>,
// ui_highlight: Option<String>,
// ui_highlight_frameline: Option<String>,
// ui_linenr: Option<String>,
// ui_linenr_selected: Option<String>,
// ui_statusline: Option<String>,
// ui_statusline_inactive: Option<String>,
// ui_statusline_normal: Option<String>,
// ui_statusline_insert: Option<String>,
// ui_statusline_select: Option<String>,
// ui_text: Option<String>,
// ui_text_focus: Option<String>,
// ui_help: Option<String>,
// ui_popup: Option<String>,
// ui_window: Option<String>,
// ui_menu: Option<String>,
// ui_menu_selected: Option<String>,
// ui_menu_scroll: Option<String>,
// ui_debug: Option<String>

// }

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

    let js: serde_json::Value = serde_json::from_reader(json_handler).unwrap();

    let a = toml::Value::String("e".to_string());
    let b = serde_json::Value::String("e".to_string());

    if a.as_str() == b.as_str() {
        println!("yes");
    }

    let last = serde_json::json!({
        "name": "Placeholder",
        "author": "Placeholder",
        "themes": [
        {
            "name": "Placeholder in themes",
            "appearance": "Placeholder",
            "style": "Placeholder Set",
        },
        ]
    });
}
