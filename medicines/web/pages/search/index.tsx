interface ISearchQuery {
  [url: string]: {
    [query: string]: {
      [key: string]: string;
    };
  };
}

const Post = ({
  url: {
    query: { query, page },
  },
}: ISearchQuery) => {
  return (
    <p>
      Post: {query} | page: {page}
    </p>
  );
};

export default Post;
