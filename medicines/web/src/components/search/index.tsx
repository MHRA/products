import React, { FormEvent } from 'react';
import styled from 'styled-components';
import {
  black,
  mhraBlue90,
  mhraGray,
  primaryColor,
  white,
} from '../../styles/colors';
import { baseSpace } from '../../styles/dimensions';

const StyledSearch = styled.section`
  border-radius: 5px 5px 0 0;
  border: 1px solid ${primaryColor};
  box-sizing: border-box;
  margin-bottom: ${baseSpace};

  h2 {
    background-color: ${mhraBlue90};
    color: ${white};
    font-size: 1.1875rem;
    margin: 0;
    padding: calc(${baseSpace} / 2);
  }

  form {
    color: ${black};
    display: flex;
    flex-direction: column;
    padding: ${baseSpace} calc(${baseSpace} / 2);
    background-color: rgba(10, 50, 150, 0.1);
  }

  label {
    font-size: 1.1875rem;
  }

  input[type='search'] {
    margin: calc(${baseSpace} / 2) 0;
    width: 100%;
    border: solid 1px ${mhraGray};
  }

  input[type='submit'] {
    align-self: flex-end;
    max-width: 50%;
    display: block;
    padding: 0 0.5rem;
    background-color: white;
    border: 1px solid black;
    border-radius: 10px;
    box-shadow: 0 4px 4px 0 rgba(0, 0, 0, 0.25);
    border: solid 1px ${mhraGray};
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
