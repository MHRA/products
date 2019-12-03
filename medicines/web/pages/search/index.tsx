interface ISearchQuery {
  [url: string]: { [query: string]: { [query: string]: string } };
}

const Post = ({
  url: {
    query: { query },
  },
}: ISearchQuery) => {
  return <p>Post: {query}</p>;
};

export default Post;
