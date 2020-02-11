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

const SearchFilter: React.FC<ISearchFilterProps> = props => {
  const { docTypes, checkDocType } = props;

  const checkDocTypeGetter = (d: DocType) => {
    return () => checkDocType(d);
  };

  return (
    <StyledSearchFilter>
      <h2>Filter documents by</h2>
      <p>
        <input
          type="checkbox"
          id="filter-spc"
          name="doc"
          value="Spc"
          checked={docTypes.includes(DocType.Spc)}
          onChange={checkDocTypeGetter(DocType.Spc)}
        />
        <label htmlFor="filter-spc">
          Summary of Product Characteristics (SPC)
        </label>
      </p>
      <p>
        <input
          type="checkbox"
          id="filter-pil"
          name="doc"
          value="Pil"
          checked={docTypes.includes(DocType.Pil)}
          onChange={checkDocTypeGetter(DocType.Pil)}
        />
        <label htmlFor="filter-pil">Patient Information Leaflet (PIL)</label>
      </p>
      <p>
        <input
          type="checkbox"
          id="filter-par"
          name="doc"
          value="Par"
          checked={docTypes.includes(DocType.Par)}
          onChange={checkDocTypeGetter(DocType.Par)}
        />
        <label htmlFor="filter-par">Public Assesment Reports (PAR)</label>
      </p>
    </StyledSearchFilter>
  );
};

export default SearchFilter;
