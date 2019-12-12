import React, { FormEvent } from 'react';
import styled from 'styled-components';
import {
  black,
  mhra70,
  mhraBlue90,
  mhraGray,
  mhraWhite,
  primaryColor,
  white,
} from '../../styles/colors';
import { baseSpace, mobileBreakpoint } from '../../styles/dimensions';

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
    border: solid 1px ${mhraGray};
    margin-right: 0.5rem;
    min-width: 0;
  }

  input[type='submit'] {
    display: block;
    cursor: pointer;
    color: ${mhraWhite};
    background-color: ${primaryColor};
    align-self: flex-end;
    max-width: 50%;
    border-radius: 6px;
    text-decoration: none;
    -webkit-appearance: none;
    border: solid 1px ${mhra70};

    &:hover {
      background-color: ${mhra70};
    }
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

const labelString = 'Enter a product or active substance:';

interface ISearchProps {
  onSearchChange: (e: FormEvent<HTMLInputElement>) => void;
  onSearchSubmit: (e: FormEvent<HTMLFormElement>) => void;
  search: string;
}

const Search: React.FC<ISearchProps> = props => (
  <StyledSearch>
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
        />
        <input type="submit" value="Search" />
      </div>
    </form>
  </StyledSearch>
);

export default Search;
