// {
// "object": "block",
// "id": "d3d710f9-7c87-4e6c-8e4d-9b2576a6fb29",
// "parent": {
// "type": "page_id",
// "page_id": "67ace61a-7fd2-4ab7-8e89-2b1dc9b252e4"
// },
// "created_time": "2023-06-11T01:40:00.000Z",
// "last_edited_time": "2023-06-11T14:31:00.000Z",
// "created_by": {
// "object": "user",
// "id": "3e1fc0f5-d02e-48ae-84c0-7ae06deece9f"
// },
// "last_edited_by": {
// "object": "user",
// "id": "3e1fc0f5-d02e-48ae-84c0-7ae06deece9f"
// },
// "has_children": false,
// "archived": false,
// "type": "paragraph",
// "paragraph": {
// "rich_text": [
// {
// "type": "text",
// "text": {
// "content": "retrieve_block",
// "link": null
// },
// "annotations": {
// "bold": false,
// "italic": false,
// "strikethrough": false,
// "underline": false,
// "code": false,
// "color": "default"
// },
// "plain_text": "retrieve_block",
// "href": null
// },
// {
// "type": "text",
// "text": {
// "content": " ",
// "link": null
// },
// "annotations": {
// "bold": true,
// "italic": false,
// "strikethrough": false,
// "underline": false,
// "code": false,
// "color": "default"
// },
// "plain_text": " ",
// "href": null
// },
// {
// "type": "text",
// "text": {
// "content": "bold",
// "link": null
// },
// "annotations": {
// "bold": true,
// "italic": false,
// "strikethrough": false,
// "underline": false,
// "code": false,
// "color": "brown"
// },
// "plain_text": "bold",
// "href": null
// }
// ],
// "color": "pink"
// }
// }

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default)]
pub struct Response<T> {
    pub object: String,
    pub results: T,
    // pub next_cursor: null ,
    pub has_more: bool,
    // pub type: "block"
    // pub block: {}
}
