import React from 'react';
import styled from 'styled-components';
import { IBmgfReport } from '../../../model/document';
import { mhraBlue80, mhraGray10, white } from '../../../styles/colors';
import {
  baseSpace,
  largePaddingSizeCss,
  mobileBreakpoint,
  tinyPaddingSizeCss,
} from '../../../styles/dimensions';
import { baseFontSize, h2FontSize } from '../../../styles/fonts';
import SearchFilter from '../../search-filter';
import Pagination from './pagination';

const StyledReportList = styled.div`
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

  dl,
  dt,
  dd {
    list-style: none;
    padding: 0;
  }

  article {
    display: flex;
    background-color: ${mhraGray10};
    padding: ${baseSpace};
    margin: ${baseSpace} 0;
    min-width: 1%;
    word-wrap: break-word;
  }

  dl p {
    margin: 0;
    padding: 0;
  }

  dd h3 {
    margin: 0;
  }

  dd a {
    text-decoration: none;
  }

  dt.left {
    flex: 1;
  }

  dt.left .icon {
    background-color: ${mhraBlue80};
    color: ${white};
    font-size: ${h2FontSize};
    font-weight: bold;
    padding: 10px 0;
    text-align: center;
    width: 70px;
  }

  dd.right {
    flex: 12;
    margin-left: 0;
    min-width: 1%;
    padding: 0 ${baseFontSize};
    word-wrap: break-word;
  }

  dd.right .title {
    font-size: ${h2FontSize};
    font-weight: bold;
    min-width: 1%;
    padding-bottom: ${tinyPaddingSizeCss};
    word-wrap: break-word;
  }

  dd.right .subtitle {
    font-size: ${baseFontSize};
    min-width: 1%;
    padding-bottom: 0.2rem;
    word-wrap: break-word;
  }

  dd.right .metadata {
    font-size: ${baseFontSize};
    min-width: 1%;
    padding-bottom: 0.1rem;
    word-wrap: break-word;
  }

  dd.right .context {
    font-size: ${h2FontSize};
    min-width: 1%;
    padding-top: ${largePaddingSizeCss};
    word-wrap: break-word;
  }

  em {
    font-weight: bold;
    font-style: normal;
  }

  .row {
    display: flex;
    flex-direction: row;

    .column {
      flex: 3;
      &.filter {
        flex: 1;
      }
    }
  }

  @media ${mobileBreakpoint} {
    .row {
      flex-direction: column;
    }
    .column.filter {
      width: 100%;
    }
  }
`;

const HiddenHeader = styled.h3`
  visibility: hidden;
  margin: 0;
  height: 0;
`;

const searchResultsTitle = (
  showingResultsForTerm: string,
  noOfResults: number,
) => {
  return noOfResults === 0
    ? `There are no search results for ${showingResultsForTerm}`
    : `Showing results for ${showingResultsForTerm}`;
};

interface ISearchNumberingInformation {
  page: number;
  pageSize: number;
  totalResultCount: number;
  shownResultCount: number;
}

const searchResultsNumberingInformation = (
  numbering: ISearchNumberingInformation,
) => {
  const zero = (numbering.page - 1) * numbering.pageSize;
  const one = zero + 1;
  const last = zero + numbering.shownResultCount;

  return `${one} to ${last} of ${numbering.totalResultCount}`;
};

const normalizeDescription = (description: string): string => {
  if (!description || typeof description !== 'string') {
    return description;
  }

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
    (substance as string).toLowerCase().charAt(0).toUpperCase() +
    substance.slice(1)
  );
}

interface ISearchResultsProps {
  reports: IBmgfReport[];
  page: number;
  pageSize: number;
  resultCount: number;
  searchTerm: string;
  showingResultsForTerm: string;

  handlePageChange: (num: number) => void;
  isLoading: boolean;
}

const SearchResults = (props: ISearchResultsProps) => {
  const {
    reports,
    page,
    pageSize,
    resultCount,
    searchTerm,
    showingResultsForTerm,
  } = props;

  const hasReports = reports.length > 0;

  return props.isLoading ? (
    <StyledReportList>
      <h2 className="title">
        {`Loading results for ${showingResultsForTerm}...`}
      </h2>
    </StyledReportList>
  ) : (
    <>
      <StyledReportList>
        <div>
          <h2 className="title">
            {searchResultsTitle(showingResultsForTerm, reports.length)}
          </h2>
          {hasReports && (
            <p className="no-of-results">
              {searchResultsNumberingInformation({
                page,
                pageSize,
                shownResultCount: reports.length,
                totalResultCount: resultCount,
              })}
            </p>
          )}
          <p>
            Before a medicine can be sold in the UK, a number of licences are
            essential. Products with a UK marketing authorisation have a licence
            number in the format ‘PL 12345/0001’. The first 2 characters are
            always the letters ‘PL’, and this can be found on the packaging of
            the product.
          </p>
          <p>
            You can identify the product in the list below using the PL number.
          </p>
          <p>
            The information about a medicine will be updated when new evidence
            becomes available. This may mean that there are differences between
            the information in the pack and the information here. The most
            up-to-date information will be available on this site.
          </p>
        </div>

        <div className="row">
          <section className="column results">
            <HiddenHeader>Search results</HiddenHeader>
            <dl>
              {hasReports &&
                reports.map((report, i) => (
                  <article key={i}>
                    <dt className="left">
                      <p className="icon">PKIP</p>
                    </dt>
                    <dd className="right">
                      <a href={report.url}>
                        <p className="title">{report.title}</p>
                      </a>

                      <p className="metadata">
                        Active substances:{' '}
                        {report.activeSubstances
                          .map((substance) => toSentenceCase(substance))
                          .join(', ')}
                      </p>

                      <p
                        className="context"
                        dangerouslySetInnerHTML={{
                          __html: normalizeDescription(report.context),
                        }}
                      />
                    </dd>
                  </article>
                ))}
            </dl>
          </section>
        </div>
      </StyledReportList>

      {resultCount > pageSize ? (
        <Pagination
          currentPage={page}
          pageSize={pageSize}
          resultCount={resultCount}
          searchTerm={searchTerm}
          handlePageChange={props.handlePageChange}
        />
      ) : (
        ''
      )}
    </>
  );
};

export default SearchResults;
