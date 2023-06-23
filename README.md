# notion-api

current API version: 2022-06-28

## notion-model

- [x] [blocks](#notion-model-blocks)
- [x] pagination
- [ ] [page properties](#notion-model-page-properties)
    - [x] objects
    - [ ] pagination
- [ ] [database properties](#notion-model-database-properties)
    - [ ] objects
    - [ ] pagination
- [ ] capabilities
- [ ] rate limiting
- [ ] property size limits

### notion (api)

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
  - [x] Retrieve a page GET
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

## notion-model database properties
- [ ] checkbox
- [ ] created_by
- [ ] created_time
- [ ] date
- [ ] email
- [ ] files
- [ ] formula
- [ ] last_edited_by
- [ ] last_edited_time
- [ ] multi_select
- [ ] number
- [ ] people
- [ ] phone_number
- [ ] relation
- [ ] rich_text
- [ ] rollup
- [ ] select
- [ ] status
- [ ] title
- [ ] unique_id (undocumented)
- [ ] url

## notion-model page properties
- [x] checkbox
- [x] created_by
- [x] created_time
- [x] date
- [x] email
- [x] files
- [x] formula
- [x] last_edited_by
- [x] last_edited_time
- [x] multi_select
- [ ] number
- [x] people
- [x] phone_number
- [x] relation
- [ ] rollup
- [x] rich_text
- [x] select
- [x] status
- [x] title
- [x] unique_id (undocumented)
- [x] url

## notion-model blocks
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

