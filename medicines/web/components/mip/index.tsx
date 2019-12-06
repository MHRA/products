import moment from 'moment';
import { useRouter } from 'next/router';
import React, { FormEvent, useEffect } from 'react';
import styled from 'styled-components';
import { IProduct } from '../../model/substance';
import {
  docSearch,
  facetSearch,
  ISearchResult,
} from '../../services/azure-search';
import substanceLoader from '../../services/substance-loader';
import { baseSpace, mobileBreakpoint } from '../../styles/dimensions';
import DrugIndex, { IFacet, index } from '../drug-index';
import MipText from '../mip-text';
import Search from '../search';
import SearchResults, { IDocument } from '../search-results';
import YellowCard from '../yellow-card';

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

  h2 {
    margin-top: 3rem;
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
  const [pageNumber, setPageNumber] = React.useState(1);
  const [resultCount, setResultCount] = React.useState(0);
  const pageSize = 10;
  const [results, setResults] = React.useState<IDocument[]>([]);
  const [facetResults, setFacetResults] = React.useState<IFacet[]>([]);
  const [search, setSearch] = React.useState('');
  const [showingResultsForTerm, setShowingResultsForTerm] = React.useState('');
  const [products, setSubstances] = React.useState<IProduct[]>([]);

  const router = useRouter();

  const {
    query: { search: searchTerm, page, substance },
  } = router;

  const handleSearchChange = (e: FormEvent<HTMLInputElement>) => {
    setSearch(e.currentTarget.value);
  };

  const fetchFacetResults = async (searchTerm: string) => {
    const searchResults = await facetSearch(searchTerm);
    const filtered = searchResults[1].facets.filter(x =>
      x.value.startsWith(searchTerm),
    );
    setFacetResults(filtered);
  };

  const fetchSearchResults = async (searchTerm: string, page: number) => {
    const searchResults = await docSearch(searchTerm, page, pageSize);
    const results = searchResults.results.map((doc: ISearchResult) => {
      return {
        activeSubstances: doc.substance_name,
        context: doc['@search.highlights']?.content.join(' … ') || '',
        docType: doc.doc_type?.toString().substr(0, 3) || '',
        fileSize: Math.ceil(
          (doc.metadata_storage_size ? doc.metadata_storage_size : 0) / 1000,
        ).toLocaleString('en-GB'),
        created: doc.created
          ? moment(doc.created).format('D MMMM YYYY')
          : 'Unknown',
        name: sanitizeTitle(doc.title),
        url: doc.metadata_storage_path,
      };
    });
    setResults(results);
    setResultCount(searchResults.resultCount);
    setShowingResultsForTerm(searchTerm);
    setSubstances([]);
  };

  const handleSearchSubmit = async (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();

    if (search.length > 0) {
      rerouteSearchResults(1);
    }
  };

  const rerouteSearchResults = (pageNo: number) => {
    router.push({
      pathname: router.route,
      query: { search, page: pageNo },
    });
  };

  useEffect(() => {
    if (searchTerm && page) {
      if (typeof searchTerm === 'string') {
        setSearch(searchTerm);
        let parsedPage = Number(page);
        if (!parsedPage || parsedPage < 1) {
          parsedPage = 1;
        }
        setPageNumber(parsedPage);
        fetchSearchResults(searchTerm, parsedPage);
      }
    } else if (substance) {
      if (typeof substance === 'string') {
        (async () => {
          const ss = await substanceLoader.load(substance.charAt(0));
          const products = ss.find(s => s.name === substance);
          if (products) {
            setSubstances(products.products);
          } else {
            setSubstances(ss);
          }
          setResults([]);
          setSearch('');
          setShowingResultsForTerm('');
        })();
      }
    }
    window.scrollTo(0, 0);
  }, [page, searchTerm, substance]);

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
      <Main>
        {showingResultsForTerm.length === 0 ? (
          <>
            <MipText />
            <h2>List of active substances</h2>
            <DrugIndex items={index} horizontal />
            {products.length > 0 && <DrugIndex items={products} />}
          </>
        ) : (
          <SearchResults
            drugs={results}
            showingResultsForTerm={showingResultsForTerm}
            resultCount={resultCount}
            page={pageNumber}
            pageSize={pageSize}
            searchTerm={search}
          />
        )}
      </Main>
    </>
  );
};

export default Mip;
