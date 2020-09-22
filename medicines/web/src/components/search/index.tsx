import React, { FormEvent } from 'react';
import styled from 'styled-components';
import { black, mhraBlue90, white } from '../../styles/colors';
import { baseSpace, mobileBreakpoint } from '../../styles/dimensions';
import { Button } from '../form-elements';

const StyledSearch = styled.section`
  box-sizing: border-box;
  margin-bottom: calc(${baseSpace} / 2);

  h2 {
    background-color: ${mhraBlue90};
    border-radius: 5px 5px 0 0;
    color: ${white};
    font-size: 1.1875rem;
    margin: 0;
    padding: calc(${baseSpace} / 2);
  }

  form {
    color: ${black};
    display: flex;
    flex-direction: column;
  }

  .searchbar {
    margin: calc(${baseSpace} / 2) 0;
    display: flex;
    > input {
      padding: 0.5rem;
    }
  }

  input[type='search'] {
    width: 100%;
    border: solid 1px ${black};
    margin-right: 0.5rem;
    min-width: 0;
  }

  @media ${mobileBreakpoint} {
    margin-bottom: 0;

    .search {
      padding: 1.25rem;
    }

    form {
      padding: ${baseSpace} 0;
    }
  }
`;

const AccessibleSearchInputHeading = styled.h3`
  visibility: hidden;
  width: 0;
  height: 0;
  margin: 0;
`;

const labelString = 'Enter a product, active substance, or PL number:';

interface ISearchProps {
  onSearchBlur: (e: FormEvent<HTMLInputElement>) => void;
  onSearchChange: (e: FormEvent<HTMLInputElement>) => void;
  onSearchSubmit: (e: FormEvent<HTMLFormElement>) => void;
  search: string;
}

const Search: React.FC<ISearchProps> = (props) => (
  <StyledSearch>
    <AccessibleSearchInputHeading>Search by text</AccessibleSearchInputHeading>
    <form
      onSubmit={props.onSearchSubmit}
      role="search"
      aria-label="Search for medicines"
    >
      <label htmlFor="search">{labelString}</label>
      <div className="searchbar">
        <input
          type="search"
          id="search"
          value={props.search}
          onChange={props.onSearchChange}
          onBlur={props.onSearchBlur}
        />
        <Button type="submit" value="Search" />
      </div>
    </form>
  </StyledSearch>
);

export default Search;
