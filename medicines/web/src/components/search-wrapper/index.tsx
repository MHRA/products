import { useRouter } from 'next/router';
import React, { FormEvent, useEffect } from 'react';
import ReactGA from 'react-ga';
import styled from 'styled-components';
import { baseSpace, mobileBreakpoint } from '../../styles/dimensions';

import DrugIndex, { index } from '../drug-index';
import Search from '../search';
import YellowCard from '../yellow-card';

const StyledSearchWrapper = styled.div`
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

interface ISearchWrapperProps {
  children: React.ReactNode;
}

const formatInitialSearchTerm = (searchTerm: string | string[]) => {
  console.log(searchTerm);
  if (searchTerm) {
    console.log('returning: ' + decodeURIComponent(searchTerm.toString()));
    return decodeURIComponent(searchTerm.toString());
  }
  return '';
};

const SearchWrapper: React.FC<ISearchWrapperProps> = props => {
  const router = useRouter();
  const [search, setSearch] = React.useState('');

  useEffect(() => {
    setSearch(formatInitialSearchTerm(router.query.search));
  }, [router.query.search]);

  const handleSearchBlur = (e: FormEvent<HTMLInputElement>) => {
    setSearch(formatSearchTerm(e.currentTarget.value));
  };

  const handleSearchChange = (e: FormEvent<HTMLInputElement>) => {
    setSearch(e.currentTarget.value);
  };

  const extractProductLicenseRegExp: RegExp = new RegExp(
    '(\\b|PL)(\\s+|/|_|-)*(\\d{5})(\\s+|/|_|-)*(\\d{4})',
    'ig',
  );

  const whitespaceRegExp: RegExp = new RegExp('\\s+', 'g');

  const formatSearchTerm = (s: string): string => {
    return encodeURIComponent(
      s
        .replace(extractProductLicenseRegExp, ' PL $3/$5')
        .replace(whitespaceRegExp, ' ')
        .trim(),
    );
  };

  const handleSearchSubmit = async (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    const formattedSearchTerm = formatSearchTerm(search);

    if (search.length > 0) {
      rerouteSearchResults(formattedSearchTerm);
    }

    ReactGA.event({
      category: 'Search',
      action: `Searched for '${search}'`,
    });
  };

  const rerouteSearchResults = (searchTerm: string) => {
    const searchRoute = `/search/${searchTerm}`;
    router.push({
      pathname: searchRoute,
      query: {
        page: 1,
      },
    });
  };

  return (
    <StyledSearchWrapper>
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
      {props.children}
    </StyledSearchWrapper>
  );
};

export default SearchWrapper;
