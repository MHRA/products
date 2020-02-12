import TagManager from 'react-gtm-module';

const pushToDataLayer = (dataLayer: any) => {
  TagManager.dataLayer({
    dataLayer,
  });
};

export default {
  search: (search: string, pageNo: number) => {
    pushToDataLayer({
      event: 'search',
      searchTerm: search,
      pageNo,
    });
  },
  substances: (substance: string) => {
    pushToDataLayer({ event: 'substance', substanceName: substance });
  },
};
