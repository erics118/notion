# notion-api

current API version: 2022-06-28

## notion-model

- [x] all blocks
    <details>
    <summary>click to see list</summary>

    - [x] bookmark
    - [x] breadcrumb
    - [x] bulleted_list_item
    - [x] callout
    - [x] child_database
    - [x] child_page
    - [x] code
    - [x] column_list
    - [x] column
    - [x] divider
    - [x] embed
    - [x] file
    - [x] heading_1
    - [x] heading_2
    - [x] heading_3
    - [x] image
    - [x] link_preview
    - [x] numbered_list_item
    - [x] paragraph
    - [x] pdf
    - [x] quote
    - [x] synced_block
    - [x] table_of_contents
    - [x] table_row
    - [x] table
    - [x] template
    - [x] to_do
    - [x] toggle
    - [x] video

    </details>

- [x] pagination
- [ ] page/database properties
    - [ ] pagination of it
- [ ] capabilities
- [ ] rate limiting
- [ ] property size limits

### notion

- Authentication
  - [ ] Create a token POST
- Blocks
  - [x] Append block children PATCH
  - [x] Retrieve a block GET
  - [x] Retrieve block children GET
  - [x] Update a block PATCH
  - [x] Delete a block DELETE
- Pages
  - [ ] Create a page POST
  - [ ] Retrieve a page GET
  - [ ] Retrieve a page property item GET
  - [ ] Update page properties PATCH
  - [ ] Archive a page DELETE
- Databases
  - [ ] Create a database POST
  - [ ] Filter database entries
  - [ ] Sort database entries
  - [ ] Query a database POST
  - [ ] Retrieve a database GET
  - [ ] Update a database PATCH
  - [ ] Update database properties
- Users
  - [ ] List all users GET
  - [ ] Retrieve a user GET
  - [ ] Retrieve your token's bot user GET
- Comments
  - [ ] Create comment POST
  - [ ] Retrieve comments GET
- Search
  - [ ] Search by title POST

