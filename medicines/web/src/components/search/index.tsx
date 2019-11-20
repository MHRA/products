import React from 'react';
import styled from 'styled-components';
import { black, mhraBlue90, primaryColor, white } from '../../styles/colors';
import { baseSpace } from '../../styles/dimensions';

const StyledSearch = styled.section`
  border-radius: 5px 5px 0 0;
  border: 1px solid ${primaryColor};
  box-sizing: border-box;
  margin-bottom: ${baseSpace};

  & h2 {
    background-color: ${mhraBlue90};
    color: ${white};
    margin: 0;
    padding: calc(${baseSpace} / 2);
  }

  & form {
    padding: calc(${baseSpace} / 2);
    color: ${black};
    display: flex;
    flex-direction: column;
  }

  & input[type='search'] {
    width: 100%;
    margin-bottom: calc(${baseSpace} / 2);
  }

  & input[type='submit'] {
    max-width: 50%;
    align-self: flex-end;
  }
`;

const title = 'Search SPC and PILs';
const labelString = 'Enter a product or active substance:';

const Search: React.FC = () => (
  <StyledSearch>
    <h2>{title}</h2>
    <form action="">
      <label htmlFor="search">{labelString}</label>
      <input type="search" id="search" />
      <input type="submit" value="Search" />
    </form>
  </StyledSearch>
);

export default Search;
