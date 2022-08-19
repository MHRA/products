import { useRouter } from 'next/router';
import React, { FormEvent, useEffect } from 'react';
import styled from 'styled-components';
import { accessibleBackgroundBlue } from '../../styles/colors';
import { baseSpace, mobileBreakpoint } from '../../styles/dimensions';
import { productLicenseRegExp } from '../../services/search-query-normalizer';
import DrugIndex, { index, IndexType } from '../drug-index';
import Search from '../search';
import YellowCard from '../yellow-card';

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
const Notice = styled.div`
  width: 100%;
  padding: 0.5rem 0rem 0rem 0rem;
  `;

  const Notice2 = styled.div`
  width: 100%;
  padding: 0.5rem 0rem 1.5rem 0rem;
  `;

interface ISearchWrapperProps {
  initialSearchValue: string | string[];
  children: React.ReactNode;
}

const formatInitialSearchTerm = (searchTerm: string | string[]) => {
  if (searchTerm) {
    return searchTerm.toString();
  }
  return '';
};

const whitespaceRegExp: RegExp = new RegExp('\\s+', 'g');

const formatSearchTerm = (s: string): string => {
  return s
    .replace(productLicenseRegExp, (match, p1, p2, p3, p4, p5) => {
      return `${p1.toUpperCase()} ${p3}/${p5}`;
    })
    .replace(whitespaceRegExp, ' ')
    .trim();
};

const SearchWrapper: React.FC<ISearchWrapperProps> = (props) => {
  const [search, setSearch] = React.useState('');
  const router = useRouter();

  useEffect(() => {
    setSearch(formatInitialSearchTerm(props.initialSearchValue));
  }, [props.initialSearchValue]);

  const handleSearchBlur = (e: FormEvent<HTMLInputElement>) => {
    setSearch(formatSearchTerm(e.currentTarget.value));
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
    const searchRoute = `/search`;
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
      <Notice>     <b>*STATUS UPDATE*</b>  
      &nbsp; We apologise for any inconvenience as we are currently experiencing technical difficulties with adding, updating or deleting all SPCs and PILs. When this message is no longer displayed the issue will have been resolved. 
 </Notice>
 <Notice2>
 If Product Information documents are urgently required, please send a request to: MHRACustomerServices@mhra.gov.uk
 </Notice2>
 
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
      <YellowCard />
      {props.children}
    </StyledSearchWrapper>
  );
};

export default SearchWrapper;
