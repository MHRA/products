import React from 'react';
import Link from 'next/link';
import styled from 'styled-components';
import { IBmgfReport } from '../../../model/document';
import { mhraGray10, errorRed } from '../../../styles/colors';
import {
  baseSpace,
  largePaddingSizeCss,
  mobileBreakpoint,
  tinyPaddingSizeCss,
} from '../../../styles/dimensions';
import { baseFontSize, h2FontSize } from '../../../styles/fonts';
import Pagination from '../../pagination';

const StyledReportList = styled.div`
  .title {
    font-size: ${h2FontSize};
    padding: 0;
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

  div.search-result {
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
    font-size: 1em;
    min-width: 1%;
    padding-bottom: 0.1rem;
    word-wrap: break-word;
  }

  dd.right .summary {
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

const TechnicalErrorMessage = styled.p`
  background-color: ${errorRed};
  padding: 20px;
`;

const TitleAndCountContainer = styled.div`
  margin-bottom: 30px;
`;

const Count = styled.p`
  margin: 0 0 30px;
  padding: 0;
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
  errorFetchingResults?: boolean;
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

  if (props.errorFetchingResults) {
    return (
      <StyledReportList>
        <TechnicalErrorMessage>
          Sorry - the site is experiencing technical issues right now. Please
          try again later.
        </TechnicalErrorMessage>
      </StyledReportList>
    );
  }

  if (props.isLoading) {
    return (
      <StyledReportList>
        <h2 className="title">
          {`Loading results for ${showingResultsForTerm}...`}
        </h2>
      </StyledReportList>
    );
  }

  const SearchResultsExplanation = (
    <>
      <p>
        If you’re looking for other information on medicines, you can search for
        information about medicines including patient information leaflets
        (PILs), details on how the medicine can be used (SPCs) and scientific
        reports (PARs).
      </p>
      <p>
        <Link href="/">
          <a>Go to Products website to find information on medicines</a>
        </Link>
      </p>
    </>
  );

  const NoSearchResultsExplanation = (
    <>
      <p>
        If the product information you are seeking does not appear below, please
        refer to the Summaries of Product Characteristics (SPCs) and Patient
        Information Leaflet (PILs) for recommendation about the use of this
        medicines in pregnancy.
      </p>
      <p>
        <Link href="/">
          <a>Go to Products website to find information on medicines</a>
        </Link>
      </p>
      <p>
        The information about the medicine levels in pregnancy will be updated
        when new evidence becomes available.
      </p>
    </>
  );

  return (
    <>
      <StyledReportList>
        <div>
          <TitleAndCountContainer>
            <h2 className="title">
              {searchResultsTitle(showingResultsForTerm, reports.length)}
            </h2>
            {hasReports && (
              <Count className="no-of-results">
                {searchResultsNumberingInformation({
                  page,
                  pageSize,
                  shownResultCount: reports.length,
                  totalResultCount: resultCount,
                })}
              </Count>
            )}
          </TitleAndCountContainer>
          {hasReports ? SearchResultsExplanation : NoSearchResultsExplanation}
        </div>

        <div className="row">
          <section className="column results">
            <HiddenHeader>Search results</HiddenHeader>
            <dl>
              {hasReports &&
                reports.map((report, i) => (
                  <div key={i} className="search-result">
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
                      {report.summary && (
                        <p className="summary">{report.summary}</p>
                      )}
                    </dd>
                  </div>
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
