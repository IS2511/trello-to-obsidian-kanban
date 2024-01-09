//! Field order somewhat preserved from a real export.
//! 
//! Last format update: `2023-08-24`.

use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Board {
    id: Id,
    node_id: NodeId,

    name: String,
    desc: String,
    /// Format unknown - skip.
    #[serde(skip)]
    desc_data: Option<DescData>,

    closed: bool,
    date_closed: Option<Date>,

    id_organization: Option<Id>,
    id_enterprise: Option<Id>,

    // limits: Limits,

    pinned: bool,
    starred: bool,

    url: String,

    // prefs: BoardPrefs,

    short_link: String,

    subscribed: bool,

    label_names: BoardLabels,

    // power_ups: Vec<PowerUp>,

    date_last_activity: Date,
    date_last_view: Date,

    short_url: String,

    id_tags: Vec<Id>,

    date_plugin_disable: Option<Date>,

    /// Format unknown - skip.
    #[serde(skip)]
    creation_method: Option<String>,

    /// Format guess: string with an integer number. Example: `63`
    ix_update: String,

    /// Format unknown - skip.
    #[serde(skip)]
    template_gallery: Option<String>,
    
    enterprise_owned: bool,

    /// Guess: board id of the template board
    id_board_source: Option<Id>,

    premium_features: Vec<String>,

    id_member_creator: Id,

    // actions: Vec<Action>,

    cards: Vec<Card>,
}

/// String of 24 characters, hexadecimal number. Example: `5f7ce5f392f71087a56b5f0e`.
pub type Id = String;

// Approx. format: `? ":" ? ":" "trello" "::" "board/" "workspace/" idOrganization "/" id`
// Example: `ari:cloud:trello::board/workspace/60b176b0d5db6e59bb8d2539/5f7ce5f392f71087a56b5f0e`
// Example: `ari:cloud:trello::card/workspace/60b176b0d5db6e59bb8d2539/6033edc5515ed38a4c8636c8`
pub type NodeId = String;

/// String with date and time in ISO 8601 format. Example: `2023-08-24T14:23:42.411Z`.
pub type Date = String;

/// String with color data, probably in hex format. Example: `#1e1e28`.
pub type Color = String;

// #[derive(Deserialize, Debug)]
// #[serde(rename_all = "camelCase")]
// pub struct DescData {
//     emoji: serde_json::Value,
// }

/// Probably an Object.
/// 
/// Seen in real exports:
/// ```json
/// { "emoji": {} }
/// ```
pub type DescData = serde_json::Value;

/// Empty string means unnamed
#[derive(Deserialize, Debug)]
pub struct BoardLabels {
    green: String,
    yellow: String,
    orange: String,
    red: String,
    purple: String,
    blue: String,
    sky: String,
    lime: String,
    pink: String,
    black: String,
    green_dark: String,
    yellow_dark: String,
    orange_dark: String,
    red_dark: String,
    purple_dark: String,
    blue_dark: String,
    sky_dark: String,
    lime_dark: String,
    pink_dark: String,
    black_dark: String,
    green_light: String,
    yellow_light: String,
    orange_light: String,
    red_light: String,
    purple_light: String,
    blue_light: String,
    sky_light: String,
    lime_light: String,
    pink_light: String,
    black_light: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Card {
    id: Id,

    /// Format unknown - skip.
    #[serde(skip)]
    address: Option<String>,

    badges: CardBadges,

    /// Format unknown - skip.
    #[serde(skip)]
    check_item_states: Vec<String>,

    closed: bool,

    /// Format unknown - skip.
    #[serde(skip)]
    coordinates: Option<String>,

    /// Format unknown - skip.
    #[serde(skip)]
    creation_method: Option<String>,

    due_complete: bool,
    
    date_last_activity: Date,

    desc: String,
    /// Format unknown - skip.
    #[serde(skip)]
    desc_data: Option<DescData>,

    /// Format unknown - skip.
    #[serde(skip)]
    due: Option<String>,
    /// `-1` seems to be a special value.
    /// Otherwise - format unknown.
    due_reminder: i64,

    /// Comment on a card via this email.
    /// 
    /// More info: https://support.atlassian.com/trello/docs/creating-cards-by-email/
    email: String,

    id_board: Id,
    id_checklists: Vec<Id>,
    id_labels: Vec<Id>,
    id_list: Id,
    id_members: Vec<Id>,
    id_members_voted: Vec<Id>,
    id_organization: Id,
    id_short: i64,
    id_attachment_cover: Option<Id>,

    labels: Vec<String>,

    // limits: Limits,

    location_name: Option<String>,

    manual_cover_attachment: bool,

    name: String,

    node_id: NodeId,

    pos: i64,

    short_link: String,
    short_url: String,

    static_map_url: Option<String>,

    /// Format unknown - skip.
    /// Guess: Date.
    #[serde(skip)]
    start: Option<Date>,

    subscribed: bool,

    url: String,

    cover: Cover,

    is_template: bool,

    /// Format unknown - skip.
    #[serde(skip)]
    card_role: Option<String>,

    attachments: Vec<Attachment>,

    /// Format unknown - skip.
    #[serde(skip)]
    plugin_data: Vec<String>,

    /// Format unknown - skip.
    #[serde(skip)]
    custom_field_items: Vec<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct CardBadges {
    /// Approx. format (from a real export):
    /// ```json
    /// {
    ///     "trello": {
    ///         "board": 0,
    ///         "card": 0
    ///     }
    /// }
    /// ```
    attachments_by_type: HashMap<String, serde_json::Value>,

    location: bool,

    votes: i64,

    viewing_member_voted: bool,

    subscribed: bool,

    /// Format unknown - skip.
    #[serde(skip)]
    fogbugz: String,

    check_items: i64,
    check_items_checked: i64,
    /// Format guess: Date
    check_items_earliest_due: Option<Date>,

    comments: i64,
    attachments: i64,

    description: bool,

    /// Format guess: Date
    due: Option<Date>,
    due_complete: bool,
    /// Format guess: Date
    start: Option<Date>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Cover {
    id_attachment: Option<Id>,

    color: Option<Color>,

    id_uploaded_background: Option<Id>,

    /// Example: `normal`
    size: String,
    /// Example: `light`
    brightness: String,

    scaled: Option<Vec<Image>>,

    edge_color: Option<Color>,

    id_plugin: Option<Id>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    /// Likely an internal field - skip.
    #[serde(skip)]
    _id: Id,

    id: Id,

    scaled: bool,

    url: String,

    bytes: i64,

    height: i64,
    width: i64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Attachment {
    id: Id,

    bytes: i64,

    date: Date,

    edge_color: Color,

    id_member: Id,

    is_upload: bool,

    mime_type: String,

    name: String,

    previews: Vec<Image>,

    url: String,

    pos: i64,

    file_name: String,
}
