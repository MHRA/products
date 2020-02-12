import TagManager from 'react-gtm-module';

const pushToDataLayer = (dataLayer: any) => {
  TagManager.dataLayer({
    dataLayer,
  });
};

export default {
  search: (searchTerm: string, pageNo: number) => {
    pushToDataLayer({
      event: 'search',
      searchTerm,
      pageNo,
    });
  },
  substances: (substance: string) => {
    pushToDataLayer({ event: 'substance', substance });
  },
  drugIndex: (letter: string) => {
    pushToDataLayer({ event: 'drugIndex', letter });
  },
  homepage: () => {
    pushToDataLayer({ event: 'homepage' });
  },
};
