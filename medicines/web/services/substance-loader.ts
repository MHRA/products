import { facetSearch } from './azure-search';

import DataLoader from 'dataloader';
import { ISubstance } from '../model/substance';

const substanceLoader = new DataLoader<string, ISubstance[]>(async keys => {
  return Promise.all(keys.map(facetSearch)).then(r =>
    r.map(([k, f]) => {
      const ss: { [id: string]: ISubstance } = {};
      f.facets
        .filter(x => x.value.startsWith(k))
        .forEach(f => {
          const xs = f.value.split(', ').slice(1);
          if (xs.length > 0) {
            const s = xs[0];
            if (ss[s] === undefined) {
              ss[s] = { name: s, count: f.count, products: [] };
            } else {
              ss[s].products?.push({ name: xs[1], count: f.count });
            }
          }
        });
      return Object.values(ss);
    }),
  );
});

export default substanceLoader;
