import moment from 'moment';
import React, { FormEvent } from 'react';
import styled from 'styled-components';
import {
  baseSpace,
  desktopMaxWidth,
  mobileBreakpoint,
} from '../../styles/dimensions';
import DrugIndex from '../drug-index';
import MipText from '../mip-text';
import Search from '../search';
import SearchResults, { IDocument } from '../search-results';
import YellowCard from '../yellow-card';
import { azureSearch, IAzureSearchResult } from './azure-search';

const Row = styled.section`
  display: flex;
  flex-wrap: wrap;
  justify-content: space-between;
  margin: 0 auto;
  max-width: ${desktopMaxWidth};
  > * {
    flex-basis: 100%;
    flex-shrink: 1;
    flex-grow: 1;
  }
`;

const Aside = styled.aside`
  max-width: 25%;
  padding: ${baseSpace} calc(${baseSpace} / 2) 0 ${baseSpace};

  @media ${mobileBreakpoint} {
    max-width: 100%;

    .pdf-yellow-card-wrapper {
      display: none;
    }
  }
`;

const Main = styled.main`
  max-width: 75%;
  padding: ${baseSpace};
  padding-left: calc(${baseSpace} / 2);

  .pdf-yellow-card-wrapper {
    display: none;
  }

  @media ${mobileBreakpoint} {
    max-width: 100%;

    .pdf-yellow-card-wrapper {
      display: block;
    }
  }
`;

const Mip: React.FC = () => {
  const [search, setSearch] = React.useState('');
  const [lastSearch, setLastSearch] = React.useState('');
  const [results, setResults] = React.useState<IDocument[]>([]);

  const handleSearchChange = (e: FormEvent<HTMLInputElement>) => {
    setSearch(e.currentTarget.value);
  };

  const sanitizeTitle = (title: string | null): string => {
    let name: string;
    if (!title) return 'Unknown';

    try {
      name = decodeURIComponent(title);
    } catch {
      name = title;
    }
    return name;
  };

  const handleSearchSubmit = async (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    if (search.length > 0) {
      const searchResults = await azureSearch(search);
      const results = searchResults.map((doc: IAzureSearchResult) => {
        return {
          activeSubstance: 'Ibuprofen',
          context: doc['@search.highlights'].content.join(' … '),
          docType: doc.doc_type.toString().substr(0, 3),
          fileSize: Math.ceil(doc.metadata_storage_size / 1000).toLocaleString(
            'en-GB',
          ),
          lastUpdated: doc.created
            ? moment(doc.created).format('Do MMM YYYY')
            : 'Unknown',
          name: sanitizeTitle(doc.title),
          url: doc.metadata_storage_path,
        };
      });
      setResults(results);
    }

    setLastSearch(search);
  };

  return (
    <Row>
      <Aside>
        <Search
          search={search}
          onSearchChange={handleSearchChange}
          onSearchSubmit={handleSearchSubmit}
        />
        <YellowCard />
      </Aside>
      {lastSearch.length === 0 ? (
        <Main>
          <MipText />
          <DrugIndex />
        </Main>
      ) : (
        <Main>
          <SearchResults drugs={results} lastSearch={lastSearch} />
        </Main>
      )}
    </Row>
  );
};

export default Mip;
