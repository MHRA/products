import React from 'react';
import styled from 'styled-components';
import { DocType } from '../../services/azure-search';

const StyledSearchFilter = styled.section`
  input {
    margin-right: 10px;
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
    <p>
      <input
        type="checkbox"
        id={id}
        name="doc"
        value={docTypeForThisCheckbox}
        checked={currentlyEnabledDocTypes.includes(docTypeForThisCheckbox)}
        onChange={toggleDocTypeForThisCheckbox}
      />
      <label htmlFor={id}>
        {name} ({docTypeForThisCheckbox.toUpperCase()})
      </label>
    </p>
  );
};

const SearchFilter: React.FC<ISearchFilterProps> = props => {
  const generateCheckboxFor = (docType: DocType, name: string) => {
    return (
      <DocTypeCheckbox
        {...props}
        docTypeForThisCheckbox={docType}
        name={name}
      />
    );
  };
  return (
    <StyledSearchFilter>
      <h2>Filter documents by</h2>
      {generateCheckboxFor(DocType.Spc, 'Summary of Product Characteristics')}
      {generateCheckboxFor(DocType.Pil, 'Patient Information Leaflet')}
      {generateCheckboxFor(DocType.Par, 'Public Assesment Reports')}
    </StyledSearchFilter>
  );
};

export default SearchFilter;
