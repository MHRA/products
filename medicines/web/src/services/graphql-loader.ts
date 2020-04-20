import DataLoader from 'dataloader';
import { IProduct } from '../model/substance';

const getProductsForSubstance = async (substanceName: any) => {
  const query = `{ substance(name: "${substanceName}") { products { name, documentCount } } }`;
  const response = await fetch('http://localhost:8000/graphql', {
    method: 'POST',
    mode: 'cors',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({ query, variables: null }),
  });

  return (await response.json()).data.substance.products;
};

const products = new DataLoader<string, IProduct[]>(async substanceNames => {
  return Promise.all(substanceNames.map(getProductsForSubstance));
});

export default {
  products,
};
