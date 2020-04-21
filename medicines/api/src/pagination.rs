use juniper::GraphQLObject;

#[derive(GraphQLObject)]
pub struct PageInfo {
    pub has_previous_page: bool,
    pub has_next_page: bool,
}

#[macro_export]
macro_rules! pagination {
    ($name:ident, $edgename:ident, $type:ty) => {
        pagination!($name, $edgename, $type, ());
    };
    ($name:ident, $edgename:ident, $type:ty, $context:ty) => {
        #[derive(juniper::GraphQLObject)]
        pub struct $edgename {
            node: $type,
            cursor: String,
        }

        impl $edgename {
            #[allow(dead_code)]
            fn new(node: $type, cursor: String) -> $edgename {
                $edgename {
                    node: node,
                    cursor: cursor,
                }
            }
        }

        #[derive(juniper::GraphQLObject)]
        pub struct $name {
            page_info: $crate::pagination::PageInfo,
            edges: Vec<$edgename>,
        }

        impl $name {
            #[allow(dead_code)]
            pub fn new(page_info: $crate::pagination::PageInfo, edges: Vec<$edgename>) -> $name {
                $name {
                    page_info: page_info,
                    edges: edges,
                }
            }
        }
    };
}
