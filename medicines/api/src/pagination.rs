use async_graphql::SimpleObject;

// Based upon: https://relay.dev/graphql/connections.htm#sec-undefined.PageInfo
#[SimpleObject]
#[derive(Debug, PartialEq)]
pub struct PageInfo {
    pub has_previous_page: bool,
    pub has_next_page: bool,
    pub start_cursor: String,
    pub end_cursor: String,
}

impl PageInfo {
    pub fn build(offset: i32, result_count: i32, total_count: i32) -> Self {
        let has_previous_page = offset != 0;
        let has_next_page = offset + result_count < total_count;
        let start_cursor = base64::encode(offset.to_string());
        let end_cursor =
            base64::encode(std::cmp::min(total_count, offset + result_count - 1).to_string());

        PageInfo {
            has_previous_page,
            has_next_page,
            start_cursor,
            end_cursor,
        }
    }
}

pub fn get_offset_or_default(skip: Option<i32>, after: Option<String>, default: i32) -> i32 {
    match (after, skip) {
        (Some(encoded), _) => match convert_after_to_offset(encoded) {
            Ok(a) => a,
            _ => default,
        },
        (None, Some(offset)) => offset,
        _ => default,
    }
}

pub fn convert_after_to_offset(encoded: String) -> Result<i32, anyhow::Error> {
    let bytes = base64::decode(encoded)?;
    let string = std::str::from_utf8(&bytes)?;
    Ok(string.parse::<i32>()? + 1)
}

#[macro_export]
macro_rules! pagination {
    ($name:ident, $edgename:ident, $type:ty) => {
        pagination!($name, $edgename, $type, ());
    };
    ($name:ident, $edgename:ident, $type:ty, $context:ty) => {
        #[SimpleObject]
        pub struct $edgename {
            node: $type,
            cursor: String,
        }

        impl $edgename {
            #[allow(dead_code)]
            fn new(node: $type, cursor: String) -> $edgename {
                $edgename { node, cursor }
            }
        }

        #[SimpleObject]
        pub struct $name {
            page_info: $crate::pagination::PageInfo,
            total_count: i32,
            edges: Vec<$edgename>,
        }

        impl $name {
            #[allow(dead_code)]
            pub fn new(
                page_info: $crate::pagination::PageInfo,
                edges: Vec<$edgename>,
                total_count: i32,
            ) -> $name {
                $name {
                    page_info,
                    total_count,
                    edges,
                }
            }
        }
    };
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_page_info() {
        let page_info = PageInfo::build(0, 10, 15);

        assert_eq!(true, page_info.has_next_page);
        assert_eq!(false, page_info.has_previous_page);
        assert_eq!("MA==", page_info.start_cursor);
        assert_eq!("OQ==", page_info.end_cursor);
    }
}
