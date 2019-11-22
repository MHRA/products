import React, { FormEvent } from 'react';
import styled from 'styled-components';
import {
  baseSpace,
  desktopMaxWidth,
  mobileBreakpoint,
} from '../../styles/dimensions';
import DrugIndex from '../drug-index';
import DrugList, { IDocument } from '../drug-list';
import MipText from '../mip-text';
import Pdf from '../pdf';
import Search from '../search';
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
  const [results, setResults] = React.useState<IDocument[]>([]);

  const handleSearchChange = (e: FormEvent<HTMLInputElement>) => {
    setSearch(e.currentTarget.value);
  };

  const handleSearchSubmit = async (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();

    setResults(
      (await azureSearch(search)).map((doc: IAzureSearchResult) => ({
        activeSubstance: 'Ibuprofen',
        context: doc['@search.highlights'].content.join(' … '),
        docType: doc.doc_type.toString().substr(0, 3),
        fileSize: Math.ceil(doc.metadata_storage_size / 1000).toLocaleString(
          'en-GB',
        ),
        lastUpdated: '12th April, 2019',
        name: 'Nurofen',
        url: atob(doc.metadata_storage_path),
      })),
    );
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
      <Main>
        <MipText />
        <DrugIndex />
        <DrugList drugs={results} />
      </Main>
    </Row>
  );
};

export default Mip;
