import moment from 'moment';
import { useRouter } from 'next/router';
import React, { FormEvent, useEffect } from 'react';
import ReactGA from 'react-ga';
import styled from 'styled-components';
import { IProduct } from '../../model/substance';
import {
  docSearch,
  DocType,
  ISearchFilters,
  ISearchResult,
} from '../../services/azure-search';
import Events from '../../services/events';
import substanceLoader from '../../services/substance-loader';
import { baseSpace, mobileBreakpoint } from '../../styles/dimensions';
import DrugIndex, { index } from '../drug-index';
import MipText from '../mip-text';
import Search from '../search';
import SearchResults, { IDocument } from '../search-results';
import YellowCard from '../yellow-card';

const StyledMip = styled.div`
  width: 100%;
  padding: 1.25rem 0.625rem 0 1.25rem;
  .search {
    background-color: rgba(10, 50, 150, 0.1);
    margin-bottom: 20px;
    padding: ${baseSpace} calc(${baseSpace} / 2);
  }

  @media ${mobileBreakpoint} {
    padding: 1.25rem;

    .search {
      padding: 1.25rem;
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
  const [hasIntro, setHasIntro] = React.useState(true);
  const [resultCount, setResultCount] = React.useState(0);
  const pageSize = 10;
  const [results, setResults] = React.useState<IDocument[]>([]);
  const [search, setSearch] = React.useState('');
  const [showingResultsForTerm, setShowingResultsForTerm] = React.useState('');
  const [products, setProducts] = React.useState<IProduct[] | null>(null);
  const [disclaimerAgree, setDisclaimerAgree] = React.useState(false);

  const router = useRouter();

  const {
    query: { search: searchTerm, page, substance, disclaimer, doc },
  } = router;

  const filters: ISearchFilters = {};
  if (typeof doc === 'string') {
    filters.docType = DocType[doc as keyof typeof DocType];
  }
  if (typeof substance === 'string') {
    filters.substanceName = substance;
  }

  const handleSearchBlur = (e: FormEvent<HTMLInputElement>) => {
    setSearch(formatSearchTerm(e.currentTarget.value));
  };

  const handleSearchChange = (e: FormEvent<HTMLInputElement>) => {
    setSearch(e.currentTarget.value);
  };

  const fetchSearchResults = async (searchTerm: string, page: number) => {
    const searchResults = await docSearch({
      query: searchTerm,
      page,
      pageSize,
      filters,
    });
    const results = searchResults.results.map((doc: ISearchResult) => {
      return {
        activeSubstances: doc.substance_name,
        product: doc.product_name,
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
    setProducts([]);
  };

  const extractProductLicenseRegExp: RegExp = new RegExp(
    '(\\b|PL)(\\s+|/|_|-)*(\\d{5})(\\s+|/|_|-)*(\\d{4})',
    'ig',
  );
  const whitespaceRegExp: RegExp = new RegExp('\\s+', 'g');

  const formatSearchTerm = (s: string): string => {
    return s
      .replace(extractProductLicenseRegExp, ' PL $3/$5')
      .replace(whitespaceRegExp, ' ')
      .trim();
  };

  const handleSearchSubmit = async (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    setSearch(formatSearchTerm(search));

    if (search.length > 0) {
      rerouteSearchResults(1);
    }

    ReactGA.event({
      category: 'Search',
      action: `Searched for '${search}'`,
    });
  };

  const rerouteSearchResults = (pageNo: number) => {
    router.push({
      pathname: router.route,
      query: { search, page: pageNo },
    });
  };

  const loadSearchResults = async (
    searchTerm: string | string[],
    page: string | string[],
  ) => {
    if (typeof searchTerm === 'string') {
      let parsedPage = Number(page);
      if (!parsedPage || parsedPage < 1) {
        parsedPage = 1;
      }
      setHasIntro(false);
      setSearch(formatSearchTerm(searchTerm));
      setPageNumber(parsedPage);
      if (disclaimer === 'agree') setDisclaimerAgree(true);
      await fetchSearchResults(searchTerm, parsedPage);
      Events.searchForProductsMatchingKeywords(search, parsedPage);
    }
  };

  const loadSubstancePage = async (substanceName: string | string[]) => {
    if (typeof substanceName === 'string') {
      (async () => {
        setHasIntro(false);
        setResults([]);
        setSearch('');
        setShowingResultsForTerm('');
        const letter = substanceName.charAt(0);
        const substanceIndex = await substanceLoader.load(letter);
        const substances = substanceIndex.find(s => s.name === substanceName);
        if (substances) {
          setProducts(substances.products);
          Events.viewProductsForSubstance(substanceName);
        } else {
          setProducts(substanceIndex);
          Events.viewSubstancesStartingWith(letter);
        }
        if (disclaimer === 'agree') setDisclaimerAgree(true);
      })();
    }
  };

  const loadHomepage = () => {
    setHasIntro(true);
    setResults([]);
    setSearch('');
    setShowingResultsForTerm('');
    setProducts(null);
    setDisclaimerAgree(false);
    Events.viewPage('homepage');
  };

  useEffect(() => {
    if (searchTerm && page) {
      loadSearchResults(searchTerm, page);
    } else if (substance) {
      loadSubstancePage(substance);
    } else {
      loadHomepage();
    }
    window.scrollTo(0, 0);
  }, [page, searchTerm, substance, disclaimer]);

  return (
    <StyledMip>
      <section className="search">
        <Search
          search={search}
          onSearchChange={handleSearchChange}
          onSearchBlur={handleSearchBlur}
          onSearchSubmit={handleSearchSubmit}
        />
        <DrugIndex
          title="or find by active substance:"
          items={index}
          horizontal
        />
      </section>
      <YellowCard />
      {showingResultsForTerm.length === 0 ? (
        <>
          {hasIntro && <MipText />}
          {products == null ? (
            <></>
          ) : products.length > 0 ? (
            <DrugIndex title={`${substance || '...'}`} items={products} />
          ) : (
            <p>Nothing found for "{substance}"</p>
          )}
        </>
      ) : (
        <SearchResults
          drugs={results}
          showingResultsForTerm={formatSearchTerm(showingResultsForTerm)}
          resultCount={resultCount}
          page={pageNumber}
          pageSize={pageSize}
          searchTerm={search}
          disclaimerAgree={disclaimerAgree}
        />
      )}
    </StyledMip>
  );
};

export default Mip;
