import React from 'react';
import styled from 'styled-components';
import { DocType } from '../../services/azure-search';

const StyledSearchFilter = styled.section`
  .checkbox-row {
    display: flex;
    flex-direction: row;
    flex-wrap: nowrap;
    padding: 0.25em;

    .checkbox {
      flex: 0.1;
      display: flex;
      flex-direction: column;
      padding: 0.25em;

      input {
        flex: 1;
        box-shadow: 0px 0px 0px 2px rgba(0, 0, 0, 1);
      }
    }

    label {
      flex: 1;
    }
  }
`;

interface ISearchFilterProps {
  currentlyEnabledDocTypes: DocType[];
  toggleDocType: (d: DocType) => void;
}

interface IDocTypeCheckboxProps extends ISearchFilterProps {
  docTypeForThisCheckbox: DocType;
  name: string;
}

const DocTypeCheckbox: React.FC<IDocTypeCheckboxProps> = props => {
  const {
    docTypeForThisCheckbox,
    name,
    toggleDocType,
    currentlyEnabledDocTypes,
  } = props;

  const toggleDocTypeForThisCheckbox = () =>
    toggleDocType(docTypeForThisCheckbox);

  const id = `filter-${docTypeForThisCheckbox.toLowerCase()}`;

  return (
    <div className="checkbox-row">
      <div className="checkbox">
        <input
          type="checkbox"
          id={id}
          name="doc"
          value={docTypeForThisCheckbox}
          checked={currentlyEnabledDocTypes.includes(docTypeForThisCheckbox)}
          onChange={toggleDocTypeForThisCheckbox}
        />
      </div>
      <label htmlFor={id}>
        {name} ({docTypeForThisCheckbox.toUpperCase()})
      </label>
    </div>
  );
};

const SearchFilter: React.FC<ISearchFilterProps> = props => {
  const generateCheckboxFor = (docType: DocType, name: string) => (
    <DocTypeCheckbox {...props} docTypeForThisCheckbox={docType} name={name} />
  );

  return (
    <StyledSearchFilter>
      <h2>Filter documents by</h2>
      {generateCheckboxFor(DocType.Spc, 'Summary of Product Characteristics')}
      {generateCheckboxFor(DocType.Pil, 'Patient Information Leaflet')}
      {generateCheckboxFor(DocType.Par, 'Public Assessment Reports')}
    </StyledSearchFilter>
  );
};

export default SearchFilter;
