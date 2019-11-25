import React, { SyntheticEvent } from 'react';
import styled from 'styled-components';

const StyledSuggestions = styled.section`
  #search-suggestions {
  }

  li {
  }
`;

export interface ISuggestion {
  value: string;
}

const SearchSuggestions = (props: {
  suggestions: ISuggestion[];
  onSelectSuggestion: (suggestion: string) => void;
}) => (
  <StyledSuggestions>
    <ul id="search-suggestions">
      {props.suggestions.map((suggestion: ISuggestion) => (
        <li
          key={suggestion.value}
          className="suggestion"
          /* tslint:disable-next-line:jsx-no-lambda */
          onClick={(e: SyntheticEvent) =>
            props.onSelectSuggestion(suggestion.value)
          }
        >
          {suggestion.value}
        </li>
      ))}
    </ul>
  </StyledSuggestions>
);

export default SearchSuggestions;
