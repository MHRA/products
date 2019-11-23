import React, { FormEvent } from 'react';
import styled from 'styled-components';
import { black, mhraBlue90, primaryColor, white } from '../../styles/colors';
import { baseSpaceSizeCss } from '../../styles/dimensions';

const StyledSearch = styled.section`
  border-radius: 5px 5px 0 0;
  border: 1px solid ${primaryColor};
  box-sizing: border-box;
  margin-bottom: ${baseSpaceSizeCss};

  & h2 {
    background-color: ${mhraBlue90};
    color: ${white};
    margin: 0;
    padding: calc(${baseSpaceSizeCss} / 2);
  }

  & form {
    padding: calc(${baseSpaceSizeCss} / 2);
    color: ${black};
    display: flex;
    flex-direction: column;
  }

  & input[type='search'] {
    width: 100%;
    margin-bottom: calc(${baseSpaceSizeCss} / 2);
  }

  & input[type='submit'] {
    max-width: 50%;
    align-self: flex-end;
  }
`;

const title = 'Search SPC and PILs';
const labelString = 'Enter a product or active substance:';

interface ISearchProps {
  onSearchChange: (e: FormEvent<HTMLInputElement>) => void;
  onSearchSubmit: (e: FormEvent<HTMLFormElement>) => void;
  search: string;
}

const Search: React.FC<ISearchProps> = props => (
  <StyledSearch>
    <h2>{title}</h2>
    <form action="" onSubmit={props.onSearchSubmit}>
      <label htmlFor="search">{labelString}</label>
      <input
        type="search"
        id="search"
        value={props.search}
        onChange={props.onSearchChange}
      />
      <input type="submit" value="Search" />
    </form>
  </StyledSearch>
);

export default Search;
