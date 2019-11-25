import React from 'react';
import styled from 'styled-components';
import { black, mhraBlue10, mhraBlue80, white } from '../../styles/colors';
import {
  baseSpace,
  largePaddingSizeCss,
  tinyPaddingSizeCss,
} from '../../styles/dimensions';
import { baseFontSize, h2FontSize } from '../../styles/fonts';

const StyledDrugList = styled.section`
  .title {
    font-size: ${h2FontSize};
    padding: 0;
    margin: 0;
  }

  .no-of-results {
    padding-top: 0;
    padding-bottom: 30px;
    margin: 0;
  }

  .ema-message {
    padding: 0;
    margin: 0;
  }

  ul {
    list-style: none;
    padding: 0;
  }

  li {
    display: flex;
    background-color: ${mhraBlue10};
    padding: ${baseSpace};
    margin: ${baseSpace} 0;
  }

  li p {
    margin: 0;
    padding: 0;
  }

  li a {
    color: ${black};
    text-decoration: none;
  }

  li .left {
    flex: 0;
  }

  li .left .icon {
    background-color: ${mhraBlue80};
    color: ${white};
    font-size: ${h2FontSize};
    font-weight: bold;
    padding: 10px 0;
    text-align: center;
    width: 50px;
  }

  li .right {
    flex: 1;
    padding: 0 ${baseFontSize};
  }

  li .right .drug-name {
    font-size: ${h2FontSize};
    font-weight: bold;
    padding-bottom: ${tinyPaddingSizeCss};
  }

  li .right .metadata {
    font-size: ${baseFontSize};
  }

  li .right .context {
    font-size: ${h2FontSize};
    padding-top: ${largePaddingSizeCss};
  }

  em {
    font-weight: bold;
    font-style: normal;
  }
`;

export interface IDocument {
  activeSubstance: string;
  context: string;
  docType: string;
  fileSize: string;
  lastUpdated: string;
  name: string;
  url: string;
}

const emaWebsiteLink = () => (
  <a href="https://www.ema.europa.eu/en" target="_new">
    European Medicines Agency
  </a>
);

const searchResultsTitle = (lastSearch: string, noOfResults: number) => {
  return noOfResults === 0
    ? `There are no search results for ${lastSearch}`
    : `Showing results for ${lastSearch}`;
};

const SearchResults = (props: { drugs: IDocument[]; lastSearch: string }) => (
  <StyledDrugList>
    <div>
      <h1 className="title">
        {searchResultsTitle(props.lastSearch, props.drugs.length)}
      </h1>
      {props.drugs.length > 0 && (
        <p className="no-of-results">{props.drugs.length} results</p>
      )}
      <p className="ema-message">
        If the product information you are seeking does not appear below, it is
        possible that the product holds a central European license and its
        information may be available at the {emaWebsiteLink()} website.
      </p>
    </div>
    <ul>
      {props.drugs.length > 0 &&
        props.drugs.map((drug, i) => (
          <li key={i}>
            <div className="left">
              <p className="icon">{drug.docType.toUpperCase()}</p>
            </div>
            <div className="right">
              <a href={drug.url}>
                <p className="drug-name">
                  {drug.name} ({drug.fileSize} KB)
                </p>
                <p className="metadata">
                  Last updated: {drug.lastUpdated}
                  <br />
                  Active substance: {drug.activeSubstance}
                </p>
                <p
                  className="context"
                  dangerouslySetInnerHTML={{ __html: drug.context }}
                />
              </a>
            </div>
          </li>
        ))}
    </ul>
  </StyledDrugList>
);

export default SearchResults;
