import moment from 'moment';
import React, { FormEvent } from 'react';
import styled from 'styled-components';
import {
  baseSpaceSizeCss,
  desktopMaxWidthCss,
  mobileBreakpointCss,
} from '../../styles/dimensions';
import DrugIndex from '../drug-index';
import MipText from '../mip-text';
import Pdf from '../pdf';
import Search from '../search';
import SearchResults, { IDocument } from '../search-results';
import YellowCard from '../yellow-card';
import { azureSearch, IAzureSearchResult } from './azure-search';

const Row = styled.section`
  display: flex;
  flex-wrap: wrap;
  justify-content: space-between;
  margin: 0 auto;
  max-width: ${desktopMaxWidthCss};
  > * {
    flex-basis: 100%;
    flex-shrink: 1;
    flex-grow: 1;
  }
`;

const Aside = styled.aside`
  max-width: 25%;
  padding: ${baseSpaceSizeCss} calc(${baseSpaceSizeCss} / 2) 0
    ${baseSpaceSizeCss};

  @media ${mobileBreakpointCss} {
    max-width: 100%;

    .pdf-yellow-card-wrapper {
      display: none;
    }
  }
`;

const Main = styled.main`
  max-width: 75%;
  padding: ${baseSpaceSizeCss};
  padding-left: calc(${baseSpaceSizeCss} / 2);

  .pdf-yellow-card-wrapper {
    display: none;
  }

  @media ${mobileBreakpointCss} {
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

  const handleSearchSubmit = async (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();

    if (search.length > 0) {
      setResults(
        (await azureSearch(search)).map((doc: IAzureSearchResult) => ({
          activeSubstance: 'Ibuprofen',
          context: doc['@search.highlights'].content.join(' … '),
          docType: doc.doc_type.toString().substr(0, 3),
          fileSize: Math.ceil(doc.metadata_storage_size / 1000).toLocaleString(
            'en-GB',
          ),
          lastUpdated: doc.created
            ? moment(doc.created).format('Do MMM YYYY')
            : 'Unknown',
          name: decodeURIComponent(doc.title || 'Unknown'),
          url: doc.metadata_storage_path,
        })),
      );
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
        <Pdf />
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
