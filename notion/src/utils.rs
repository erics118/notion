use std::str::FromStr;

use notion_model::ids::{BlockId, PageId, WorkspaceId};

pub fn get_block_id_from_url(url: &str) -> Option<BlockId> {
    println!("{:#?}", url);
    let split_url: Vec<&str> = url.split('#').collect();
    println!("{:#?}", split_url);
    if split_url.len() == 2 {
        return split_url.get(1).and_then(|s| BlockId::from_str(s).ok());
    }
    None
}

pub fn get_page_id_from_url(url: &str) -> Option<PageId> {
    let split_url: Vec<&str> = url.split(&['/', '?']).collect();

    for i in 0..split_url.len() {
        if vec![
            "notion.so",
            "notion.site",
            "www.notion.so",
            "www.notion.site",
        ]
        .contains(&split_url[i])
        {
            if i + 1 < split_url.len() {
                return split_url.get(i + 2).and_then(|s| PageId::from_str(s).ok());
            }
        }
    }

    None
}

pub fn get_workspace_from_url(url: &str) -> Option<WorkspaceId> {
    let split_url: Vec<&str> = url.split('/').collect();

    for i in 0..split_url.len() {
        if vec![
            "notion.so",
            "notion.site",
            "www.notion.so",
            "www.notion.site",
        ]
        .contains(&split_url[i])
        {
            if i + 2 < split_url.len() {
                return split_url
                    .get(i + 1)
                    .and_then(|s| WorkspaceId::from_str(s).ok());
            }
        }
    }

    None
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn a() {
        let url = "https://www.notion.so/erics118/67ace61a7fd24ab78e892b1dc9b252e4?pvs=4#6e9612c81c7d4356ba9153eab009e6f4";
        let block_id = get_block_id_from_url(url).unwrap_or_default();
        println!("Block ID: {}", block_id);
    }

    #[test]
    pub fn block_full() {
        assert_eq!(
            get_block_id_from_url(
                "https://www.notion.so/erics118/67ace61a7fd24ab78e892b1dc9b252e4?pvs=4#6e9612c81c7d4356ba9153eab009e6f4"
            ),
            Some(BlockId::from_str_unchecked(
                "6e9612c81c7d4356ba9153eab009e6f4"
            ))
        );
    }

    #[test]
    pub fn block_no_https() {
        assert_eq!(
            get_block_id_from_url(
                "www.notion.so/erics118/67ace61a7fd24ab78e892b1dc9b252e4?pvs=4#6e9612c81c7d4356ba9153eab009e6f4"
            ),
            Some(BlockId::from_str_unchecked(
                "6e9612c81c7d4356ba9153eab009e6f4"
            ))
        );
    }

    #[test]
    pub fn block_no_www() {
        assert_eq!(
            get_block_id_from_url(
                "https://notion.so/erics118/67ace61a7fd24ab78e892b1dc9b252e4?pvs=4#6e9612c81c7d4356ba9153eab009e6f4"
            ),
            Some(BlockId::from_str_unchecked(
                "6e9612c81c7d4356ba9153eab009e6f4"
            ))
        );
    }

    #[test]
    pub fn block_no_https_no_www() {
        assert_eq!(
            get_block_id_from_url(
                "notion.so/erics118/67ace61a7fd24ab78e892b1dc9b252e4?pvs=4#6e9612c81c7d4356ba9153eab009e6f4"
            ),
            Some(BlockId::from_str_unchecked(
                "6e9612c81c7d4356ba9153eab009e6f4"
            ))
        );
    }

    #[test]
    pub fn block_no_params() {
        assert_eq!(
            get_block_id_from_url(
                "https://www.notion.so/erics118/67ace61a7fd24ab78e892b1dc9b252e4#6e9612c81c7d4356ba9153eab009e6f4"
            ),
            Some(BlockId::from_str_unchecked(
                "6e9612c81c7d4356ba9153eab009e6f4"
            ))
        );
    }

    #[test]
    pub fn block_none() {
        assert_eq!(
            get_block_id_from_url(
                "https://www.notion.so/erics118/67ace61a7fd24ab78e892b1dc9b252e4"
            ),
            None
        );
    }

    #[test]
    pub fn page_full() {
        assert_eq!(
            get_page_id_from_url(
                "https://www.notion.so/erics118/67ace61a7fd24ab78e892b1dc9b252e4?pvs=4#6e9612c81c7d4356ba9153eab009e6f4"
            ),
            Some(PageId::from_str_unchecked(
                "67ace61a7fd24ab78e892b1dc9b252e4"
            ))
        );
    }

    #[test]
    pub fn page_no_https() {
        assert_eq!(
            get_page_id_from_url(
                "www.notion.so/erics118/67ace61a7fd24ab78e892b1dc9b252e4?pvs=4#6e9612c81c7d4356ba9153eab009e6f4"
            ),
            Some(PageId::from_str_unchecked(
                "67ace61a7fd24ab78e892b1dc9b252e4"
            ))
        );
    }

    #[test]
    pub fn page_no_www() {
        assert_eq!(
            get_page_id_from_url(
                "https://notion.so/erics118/67ace61a7fd24ab78e892b1dc9b252e4?pvs=4#6e9612c81c7d4356ba9153eab009e6f4"
            ),
            Some(PageId::from_str_unchecked(
                "67ace61a7fd24ab78e892b1dc9b252e4"
            ))
        );
    }

    #[test]
    pub fn page_no_https_no_www() {
        assert_eq!(
            get_page_id_from_url(
                "notion.so/erics118/67ace61a7fd24ab78e892b1dc9b252e4?pvs=4#6e9612c81c7d4356ba9153eab009e6f4"
            ),
            Some(PageId::from_str_unchecked(
                "67ace61a7fd24ab78e892b1dc9b252e4"
            ))
        );
    }

    #[test]
    pub fn page_no_block() {
        assert_eq!(
            get_page_id_from_url("https://www.notion.so/erics118/67ace61a7fd24ab78e892b1dc9b252e4"),
            Some(PageId::from_str_unchecked(
                "67ace61a7fd24ab78e892b1dc9b252e4"
            ))
        );
    }

    #[test]
    pub fn page_none() {
        assert_eq!(get_page_id_from_url("https://www.notion.so/erics118"), None);
    }

    #[test]
    pub fn workspace_full() {
        assert_eq!(
            get_workspace_from_url(
                "https://www.notion.so/erics118/67ace61a7fd24ab78e892b1dc9b252e4?pvs=4#6e9612c81c7d4356ba9153eab009e6f4"
            ),
            Some(WorkspaceId::from_str_unchecked("erics118"))
        );
    }

    #[test]
    pub fn workspace_no_https() {
        assert_eq!(
            get_workspace_from_url(
                "www.notion.so/erics118/67ace61a7fd24ab78e892b1dc9b252e4?pvs=4#6e9612c81c7d4356ba9153eab009e6f4"
            ),
            Some(WorkspaceId::from_str_unchecked("erics118"))
        );
    }

    #[test]
    pub fn workspace_no_www() {
        assert_eq!(
            get_workspace_from_url(
                "https://notion.so/erics118/67ace61a7fd24ab78e892b1dc9b252e4?pvs=4#6e9612c81c7d4356ba9153eab009e6f4"
            ),
            Some(WorkspaceId::from_str_unchecked("erics118"))
        );
    }

    #[test]
    pub fn workspace_no_https_no_www() {
        assert_eq!(
            get_workspace_from_url(
                "notion.so/erics118/67ace61a7fd24ab78e892b1dc9b252e4?pvs=4#6e9612c81c7d4356ba9153eab009e6f4"
            ),
            Some(WorkspaceId::from_str_unchecked("erics118"))
        );
    }

    #[test]
    pub fn workspace_https_no_www() {
        assert_eq!(
            get_workspace_from_url(
                "www.notion.so/erics118/67ace61a7fd24ab78e892b1dc9b252e4?pvs=4#6e9612c81c7d4356ba9153eab009e6f4"
            ),
            Some(WorkspaceId::from_str_unchecked("erics118"))
        );
    }
}
