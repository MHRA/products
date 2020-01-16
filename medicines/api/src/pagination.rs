use juniper::GraphQLObject;
#[derive(GraphQLObject)]
pub struct PageInfo {
    pub has_previous_page: bool,
    pub has_next_page: bool
}

#[macro_export]
macro_rules! pagination {
    ($name:ident, $edgename:ident, $type:ty) => {
        pagination!($name, $edgename, $type, ());
    };
    ($name:ident, $edgename:ident, $type:ty, $context:ty) => {
        pub struct $edgename {
            node: $type,
            cursor: String
        }

        juniper::graphql_object!($edgename: () |&self| {
            field node() -> &$type {
                &self.node
            }

            field cursor() -> &String {
                &self.cursor
            }
        });

        impl $edgename {
            #[allow(dead_code)]
             fn new(node: $type, cursor: String) -> $edgename {
                $edgename {
                    node: node,
                    cursor: cursor
                }
            }
        }

        pub struct $name {
            page_info: $crate::pagination::PageInfo,
            edges: Vec<$edgename>
        }

        juniper::graphql_object!($name: () |&self| {
            field page_info() -> &$crate::pagination::PageInfo {
                &self.page_info
            }

            field edges() -> &Vec<$edgename> {
                &self.edges
            }
        });

        impl $name {
            #[allow(dead_code)]
            pub fn new(page_info: $crate::pagination::PageInfo, edges: Vec<$edgename>) -> $name {
                $name {
                    page_info: page_info,
                    edges: edges
                }
            }
        }
    }
}