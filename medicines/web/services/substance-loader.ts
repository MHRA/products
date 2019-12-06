import { facetSearch } from './azure-search';

import DataLoader from 'dataloader';
import { IFacet } from '../components/drug-index';

const substanceLoader = new DataLoader<string, IFacet[]>(async keys => {
  return Promise.all(keys.map(facetSearch)).then(r =>
    r.map(([k, f]) => f.facets.filter(x => x.value.startsWith(k))),
  );
});

export default substanceLoader;
