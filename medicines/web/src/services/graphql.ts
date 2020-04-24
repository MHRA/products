interface IGraphQLResponse<T> {
  data: T;
  // TODO: add proper type for errors field
}

const graphqlUrl = process.env.GRAPHQL_URL as string;

export async function graphqlRequest<T, V extends {}>({
  query,
  variables,
}: {
  query: string;
  variables: V;
}): Promise<IGraphQLResponse<T>> {
  const response = await fetch(graphqlUrl, {
    method: 'POST',
    mode: 'cors',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({
      query,
      variables,
    }),
  });

  if (!response.ok) {
    throw new Error(`Error response code from GraphQL API: ${response.status}`);
  }

  return response.json();
}
