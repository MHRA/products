import React from 'react';
import styled from 'styled-components';
import { DocType } from '../../services/azure-search';

const StyledSearchFilter = styled.section`
  input {
    margin-right: 10px;
  }
`;

interface ISearchFilterProps {
  docTypes: DocType[];
  checkDocType: (d: DocType) => void;
}

interface IDocTypeCheckboxProps extends ISearchFilterProps {
  docType: DocType;
  name: string;
}

const DocTypeCheckbox: React.FC<IDocTypeCheckboxProps> = props => {
  const { docType: d, name, checkDocType, docTypes } = props;
  const handleCheck = () => checkDocType(d);
  const id = `filter-${d.toLowerCase()}`;
  return (
    <p>
      <input
        type="checkbox"
        id={id}
        name="doc"
        value={d}
        checked={docTypes.includes(d)}
        onChange={handleCheck}
      />
      <label htmlFor={id}>
        {name} ({d.toUpperCase()})
      </label>
    </p>
  );
};

const SearchFilter: React.FC<ISearchFilterProps> = props => {
  const generateCheckbox = (d: DocType, n: string) => {
    return <DocTypeCheckbox {...props} docType={d} name={n} />;
  };
  return (
    <StyledSearchFilter>
      <h2>Filter documents by</h2>
      {generateCheckbox(DocType.Spc, 'Summary of Product Characteristics')}
      {generateCheckbox(DocType.Pil, 'Patient Information Leaflet')}
      {generateCheckbox(DocType.Par, 'Public Assesment Reports')}
    </StyledSearchFilter>
  );
};

export default SearchFilter;
