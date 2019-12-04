import moment from 'moment';
import React, { FormEvent } from 'react';
import styled from 'styled-components';
import { baseSpace, mobileBreakpoint } from '../../styles/dimensions';
import MipText from '../mip-text';
import Search from '../search';
import SearchResults, { IDocument } from '../search-results';
import YellowCard from '../yellow-card';
import { azureSearch, IAzureSearchResult } from './azure-search';

const Aside = styled.aside`
  max-width: 25%;
  padding: ${baseSpace} calc(${baseSpace} / 2) 0 ${baseSpace};

  @media ${mobileBreakpoint} {
    max-width: 100%;
    padding: 0.3125rem;

    .yellow-card-wrapper {
      display: none;
    }
  }
`;

const Main = styled.main`
  max-width: 75%;
  padding: ${baseSpace};
  padding-left: calc(${baseSpace} / 2);

  .yellow-card-wrapper {
    display: none;
  }

  @media ${mobileBreakpoint} {
    max-width: 100%;

    .yellow-card-wrapper {
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
          activeSubstances: doc.substance_name,
          context: doc['@search.highlights']?.content.join(' … ') || '',
          docType: doc.doc_type?.toString().substr(0, 3) || '',
          fileSize: Math.ceil(
            doc.metadata_storage_size ? doc.metadata_storage_size : 0 / 1000,
          ).toLocaleString('en-GB'),
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
    <>
      <Aside>
        <Search
          search={search}
          onSearchChange={handleSearchChange}
          onSearchSubmit={handleSearchSubmit}
        />
        <div className="yellow-card-wrapper">
          <YellowCard />
        </div>
      </Aside>
      {lastSearch.length === 0 ? (
        <Main>
          <MipText />
          {/* <DrugIndex /> */}
          <div className="yellow-card-wrapper">
            <YellowCard />
          </div>
        </Main>
      ) : (
        <Main>
          <SearchResults drugs={results} lastSearch={lastSearch} />
          <div className="yellow-card-wrapper">
            <YellowCard />
          </div>
        </Main>
      )}
    </>
  );
};

export default Mip;
