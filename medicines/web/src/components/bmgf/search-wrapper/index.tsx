import { useRouter } from 'next/router';
import React, { FormEvent, useEffect } from 'react';
import styled from 'styled-components';
import { accessibleBackgroundBlue } from '../../../styles/colors';
import { baseSpace, mobileBreakpoint } from '../../../styles/dimensions';

import DrugIndex, { index, IndexType } from '../drug-index';
import Search from '../../search';

const StyledSearchWrapper = styled.div`
  width: 100%;
  padding: 1.25rem 0.625rem 0 1.25rem;
  .search {
    background-color: ${accessibleBackgroundBlue};
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

const AccessibleSearchBoxHeading = styled.h2`
  visibility: hidden;
  width: 0;
  height: 0;
  margin: 0;
`;
interface ISearchWrapperProps {
  initialSearchValue: string;
  children: React.ReactNode;
}

const whitespaceRegExp: RegExp = new RegExp('\\s+', 'g');

const formatSearchTerm = (s: string): string => {
  return s.replace(whitespaceRegExp, ' ').toLowerCase();
};

const SearchWrapper: React.FC<ISearchWrapperProps> = (props) => {
  const [search, setSearch] = React.useState('');
  const router = useRouter();

  useEffect(() => {
    setSearch(props.initialSearchValue);
  }, [props.initialSearchValue]);

  const handleSearchBlur = (e: FormEvent<HTMLInputElement>) => {
    setSearch(e.currentTarget.value);
  };

  const handleSearchChange = (e: FormEvent<HTMLInputElement>) => {
    setSearch(e.currentTarget.value);
  };

  const handleSearchSubmit = async (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    const formattedSearchTerm = formatSearchTerm(search);

    if (search.length > 0) {
      rerouteSearchResults(formattedSearchTerm);
    }
  };

  const rerouteSearchResults = (searchTerm: string) => {
    const searchRoute = `/medicine-levels-in-pregnancy/search`;
    router.push({
      pathname: searchRoute,
      query: {
        search: searchTerm,
        page: 1,
      },
    });
  };

  return (
    <StyledSearchWrapper>
      <section className="search">
        <AccessibleSearchBoxHeading>Search box</AccessibleSearchBoxHeading>
        <Search
          search={search}
          onSearchChange={handleSearchChange}
          onSearchBlur={handleSearchBlur}
          onSearchSubmit={handleSearchSubmit}
        />
        <DrugIndex
          title="or find by active substance:"
          items={index}
          indexType={IndexType.Horizontal}
        />
      </section>
      {props.children}
    </StyledSearchWrapper>
  );
};

export default SearchWrapper;
