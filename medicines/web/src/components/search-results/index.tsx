import React, { MouseEvent } from 'react';
import styled from 'styled-components';
import { useLocalStorage, useSessionStorage } from '../../hooks';
import { mhraBlue80, mhraGray10, white } from '../../styles/colors';
import {
  baseSpace,
  largePaddingSizeCss,
  mobileBreakpoint,
  tinyPaddingSizeCss,
} from '../../styles/dimensions';
import { baseFontSize, h2FontSize } from '../../styles/fonts';
import Disclaimer from '../disclaimer';
import SearchFilter from '../search-filter';
import Pagination from './pagination';

import { DocType } from '../../services/azure-search';
import { IDocument } from '../../services/results-converter';

const StyledDrugList = styled.div`
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

interface ISearchResultsProps {
  drugs: IDocument[];
  page: number;
  pageSize: number;
  resultCount: number;
  searchTerm: string;
  showingResultsForTerm: string;
  disclaimerAgree: boolean;
  docTypes: DocType[];
  handleDocTypeCheckbox: (d: DocType) => void;
  handlePageChange: (num: number) => void;
  isLoading: boolean;
}

const SearchResults = (props: ISearchResultsProps) => {
  const [storageAllowed, setStorageAllowed] = useLocalStorage(
    'allowStorage',
    false,
  );

  // Keep track of whether we should show the disclaimer warning.
  // If storage is not allowed, use React state.
  let [showDisclaimerWarning, setShowDisclaimerWarning] = React.useState(true);
  if (storageAllowed) {
    // If storage is allowed, use session storage.
    [showDisclaimerWarning, setShowDisclaimerWarning] = useSessionStorage(
      'showDisclaimer',
      true,
    );
  }

  const {
    disclaimerAgree,
    drugs,
    page,
    pageSize,
    resultCount,
    searchTerm,
    showingResultsForTerm,
    docTypes,
    handleDocTypeCheckbox,
  } = props;

  const hasDrugs = drugs.length > 0;

  const handleOnDisclaimerAgree = (
    event: MouseEvent<HTMLButtonElement>,
  ): void => {
    event.preventDefault();
    setShowDisclaimerWarning(false);
  };

  return props.isLoading ? (
    <StyledDrugList>
      <h1 className="title">
        {`Loading results for ${showingResultsForTerm}...`}
      </h1>
    </StyledDrugList>
  ) : (
    <>
      <StyledDrugList>
        <div>
          <h1 className="title">
            {searchResultsTitle(showingResultsForTerm, drugs.length)}
          </h1>
          {hasDrugs && (
            <p className="no-of-results">
              {searchResultsNumberingInformation({
                page,
                pageSize,
                shownResultCount: drugs.length,
                totalResultCount: resultCount,
              })}
            </p>
          )}
          <p className="ema-message">
            If the product information you are seeking does not appear below, it
            is possible that the product holds a European licence and its
            information may be available at the {emaWebsiteLink()} website.
          </p>
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
        {showDisclaimerWarning && hasDrugs && !disclaimerAgree ? (
          <Disclaimer
            onDisclaimerAgree={handleOnDisclaimerAgree}
            searchTerm={searchTerm}
          />
        ) : (
          <div className="row">
            <div className="column filter">
              <SearchFilter
                currentlyEnabledDocTypes={docTypes}
                toggleDocType={handleDocTypeCheckbox}
              />
            </div>
            <div className="column results">
              <dl>
                {hasDrugs &&
                  drugs.map((drug, i) => (
                    <article key={i}>
                      <dt className="left">
                        <p className="icon">{drug.docType.toUpperCase()}</p>
                      </dt>
                      <dd className="right">
                        <a
                          href={drug.url}
                          className={'doc-type-' + drug.docType.toLowerCase()}
                        >
                          {drug.product != null ? (
                            <>
                              <p className="title">{drug.product}</p>
                              <p className="subtitle">{drug.name}</p>
                            </>
                          ) : (
                            <p className="title">{drug.name}</p>
                          )}
                        </a>
                        <p className="metadata">
                          File size: {drug.fileSize} KB
                        </p>
                        {drug.activeSubstances != null &&
                          drug.activeSubstances.length > 0 && (
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
                      </dd>
                    </article>
                  ))}
              </dl>
            </div>
          </div>
        )}
      </StyledDrugList>

      {resultCount > pageSize && (!showDisclaimerWarning || disclaimerAgree) ? (
        <Pagination
          currentPage={page}
          pageSize={pageSize}
          resultCount={resultCount}
          searchTerm={searchTerm}
          enabledDocTypes={docTypes}
          handlePageChange={props.handlePageChange}
        />
      ) : (
        ''
      )}
    </>
  );
};

export default SearchResults;
