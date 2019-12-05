import moment from 'moment';
import { useRouter } from 'next/router';
import React, { FormEvent, useEffect } from 'react';
import ReactGA from 'react-ga-gtm';
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

const Mip: React.FC = () => {
  const [search, setSearch] = React.useState('');
  const [showingResultsForTerm, setShowingResultsForTerm] = React.useState('');
  const [results, setResults] = React.useState<IDocument[]>([]);
  const router = useRouter();
  const {
    query: { search: searchTerm, page },
  } = router;

  const handleSearchChange = (e: FormEvent<HTMLInputElement>) => {
    setSearch(e.currentTarget.value);
  };

  const handleSearchSubmit = async (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();

    if (search.length > 0) {
      router.push({
        pathname: '/',
        query: { search, page: 1 },
      });
    }

    ReactGA.event({
      category: 'Search',
      action: `Searched for '${searchTerm}'`,
    });
  };

  const fetchSearchResults = async (searchTerm: string) => {
    const searchResults = await azureSearch(searchTerm);
    const results = searchResults.map((doc: IAzureSearchResult) => {
      return {
        activeSubstances: doc.substance_name,
        context: doc['@search.highlights']?.content.join(' … ') || '',
        docType: doc.doc_type?.toString().substr(0, 3) || '',
        fileSize: Math.ceil(
          doc.metadata_storage_size ? doc.metadata_storage_size : 0 / 1000,
        ).toLocaleString('en-GB'),
        created: doc.created
          ? moment(doc.created).format('DD MMMM YYYY')
          : 'Unknown',
        name: sanitizeTitle(doc.title),
        url: doc.metadata_storage_path,
      };
    });
    setResults(results);
    setSearch(searchTerm);
    setShowingResultsForTerm(searchTerm);
  };

  useEffect(() => {
    if (searchTerm && page) {
      if (typeof searchTerm === 'string') {
        fetchSearchResults(searchTerm);
      }
    }
  }, [searchTerm]);

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
      {showingResultsForTerm.length === 0 ? (
        <Main>
          <MipText />
          {/* <DrugIndex /> */}
          <div className="yellow-card-wrapper">
            <YellowCard />
          </div>
        </Main>
      ) : (
        <Main>
          <SearchResults
            drugs={results}
            showingResultsForTerm={showingResultsForTerm}
          />
          <div className="yellow-card-wrapper">
            <YellowCard />
          </div>
        </Main>
      )}
    </>
  );
};

export default Mip;
