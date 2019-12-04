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
    min-width: 1%;
    word-wrap: break-word;
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
    width: 70px;
  }

  li .right {
    flex: 1;
    padding: 0 ${baseFontSize};
    min-width: 1%;
    word-wrap: break-word;
  }

  li .right .drug-name {
    font-size: ${h2FontSize};
    font-weight: bold;
    padding-bottom: ${tinyPaddingSizeCss};
    min-width: 1%;
    word-wrap: break-word;
  }

  li .right .metadata {
    font-size: ${baseFontSize};
    min-width: 1%;
    word-wrap: break-word;
  }

  li .right .context {
    font-size: ${h2FontSize};
    padding-top: ${largePaddingSizeCss};
    min-width: 1%;
    word-wrap: break-word;
  }

  em {
    font-weight: bold;
    font-style: normal;
  }
`;

export interface IDocument {
  activeSubstances: string[];
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

const searchResultsTitle = (
  showingResultsForTerm: string,
  noOfResults: number,
) => {
  return noOfResults === 0
    ? `There are no search results for ${showingResultsForTerm}`
    : `Showing results for ${showingResultsForTerm}`;
};

const normalizeDescription = (description: string): string => {
  const normalized = description
    .substr(0, 300) // Cut to 300 characters.
    .replace(/[^\w<>/\s…]/gi, '') // Remove non-word characters other than ellipsis & tags.
    .replace(/\s+/, ''); // Replace multi-spaces with one.

  if (/^\S/.test(description.substr(300))) {
    return normalized.replace(/\s+\S*$/, '') + '…'; // Add ellipsis.
  }

  return normalized;
};

function toSentenceCase(substance: string): string {
  return (
    (substance as string)
      .toLowerCase()
      .charAt(0)
      .toUpperCase() + substance.slice(1)
  );
}

const SearchResults = (props: {
  drugs: IDocument[];
  showingResultsForTerm: string;
}) => (
  <StyledDrugList>
    <div>
      <h1 className="title">
        {searchResultsTitle(props.showingResultsForTerm, props.drugs.length)}
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
                <p className="metadata">Last updated: {drug.lastUpdated}</p>
                {drug.docType !== 'Par' && (
                  <p className="metadata">
                    Active substances:{' '}
                    {drug.activeSubstances
                      .map(substance => toSentenceCase(substance))
                      .join(', ')}
                  </p>
                )}
                <p
                  className="context"
                  dangerouslySetInnerHTML={{
                    __html: normalizeDescription(drug.context),
                  }}
                />
              </a>
            </div>
          </li>
        ))}
    </ul>
  </StyledDrugList>
);

export default SearchResults;
