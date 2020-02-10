import React from 'react';
import styled from 'styled-components';
import { DocType, ISearchFilters } from '../../services/azure-search';

const StyledSearchFilter = styled.section`
  input {
    margin-right: 10px;
  }
`;

interface ISearchFilterProps {
  filters: ISearchFilters;
  setFilters: any;
}

const SearchFilter: React.FC<ISearchFilterProps> = props => {
  const { filters, setFilters } = props;

  const createCheckboxHandler = (docType: DocType) => {
    return () => {
      let { docType: docTypeClone } = filters;
      if (docTypeClone.includes(docType)) {
        const docTypeIndex = docTypeClone.indexOf(docType);
        docTypeClone = docTypeClone.splice(docTypeIndex, 1);
      } else {
        docTypeClone.push(docType);
      }
      setFilters({ docType: docTypeClone, ...filters });
    };
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
          checked={filters.docType.includes(DocType.Spc)}
          onChange={createCheckboxHandler(DocType.Spc)}
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
          checked={filters.docType.includes(DocType.Pil)}
          onChange={createCheckboxHandler(DocType.Pil)}
        />
        <label htmlFor="filter-pil">Patient Information Leaflet (PIL)</label>
      </p>
      <p>
        <input
          type="checkbox"
          id="filter-par"
          name="doc"
          value="Par"
          checked={filters.docType.includes(DocType.Par)}
          onChange={createCheckboxHandler(DocType.Par)}
        />
        <label htmlFor="filter-par">Public Assesment Reports (PAR)</label>
      </p>
    </StyledSearchFilter>
  );
};

export default SearchFilter;
