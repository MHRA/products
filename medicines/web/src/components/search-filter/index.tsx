import React, { useEffect, useRef, useState } from 'react';
import styled from 'styled-components';
import { RerouteType } from '../../model/rerouteType';
import {
  DocType,
  SearchType,
  TerritoryType,
} from '../../services/azure-search';
import { Button, Checkbox } from '../form-elements';

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
      padding: 0.25em 0;
    }

    label {
      display: flex;
      align-items: center;
    }
  }
`;

const ButtonContainer = styled.div`
  padding-top: 15px;
`;

const Fieldset = styled.fieldset`
  border: 0;
  padding: 0;
`;

const Legend = styled.legend`
  display: block;
  font-size: 18px;
  font-weight: bold;
  margin-top: 19px;
`;

const SubLegend = styled.legend`
  display: block;
  font-weight: bold;
  margin: 19px 0;
`;

const AccessibleHeading = styled.h3`
  visibility: hidden;
  width: 0;
  height: 0;
  margin: 0;
`;

interface ISearchFilterProps {
  currentlyEnabledDocTypes: DocType[];
  currentlyEnabledTerritoryTypes: TerritoryType[];
  updatePageFilters: (d: DocType[], t: TerritoryType[]) => void;
  rerouteType: RerouteType;
}

interface ICheckboxProps {
  value: string;
  label: string;
  checked: boolean;
  query: string;
  toggleFilter: (s: string) => void;
}

const FilterCheckbox: React.FC<ICheckboxProps> = (props) => {
  const { value, label, checked, toggleFilter, query } = props;

  const toggleDocTypeForThisCheckbox = () => toggleFilter(value);

  const id = `filter-${value.toLowerCase()}`;

  return (
    <div className="checkbox-row">
      <div className="checkbox">
        <Checkbox
          id={id}
          name={query}
          value={value}
          checked={checked}
          onChange={toggleDocTypeForThisCheckbox}
        />
      </div>
      <label htmlFor={id}>
        {label} ({value.toUpperCase()})
      </label>
    </div>
  );
};

const SearchFilter: React.FC<ISearchFilterProps> = (props) => {
  const [checkedDocFilters, setCheckedDocFilters] = useState(
    props.currentlyEnabledDocTypes,
  );
  const [checkedTerritoryFilters, setCheckedTerritoryFilters] = useState(
    props.currentlyEnabledTerritoryTypes,
  );
  const submitButton = useRef(null);
  const filterHeader = useRef(null);

  const generateDocTypeCheckboxFor = (docType: DocType, name: string) => {
    const toggleDoc = (docType) =>
      toggleFilter(docType, checkedDocFilters, setCheckedDocFilters);
    return (
      <FilterCheckbox
        toggleFilter={toggleDoc}
        value={docType}
        label={name}
        query={SearchType.Doc}
        checked={checkedDocFilters.includes(docType)}
      />
    );
  };

  const generateTerritoryTypeCheckboxFor = (
    territoryType: TerritoryType,
    name: string,
  ) => {
    const toggleTerritory = (territoryType) =>
      toggleFilter(
        territoryType,
        checkedTerritoryFilters,
        setCheckedTerritoryFilters,
      );
    return (
      <FilterCheckbox
        toggleFilter={toggleTerritory}
        value={territoryType}
        label={name}
        query={SearchType.Territory}
        checked={checkedTerritoryFilters.includes(territoryType)}
      />
    );
  };

  const toggleFilter = (filterToToggle, checkedFilters, setCheckedFilters) => {
    const enabledFilters = Array.from(checkedFilters);
    if (enabledFilters.includes(filterToToggle)) {
      const filterIndex = enabledFilters.indexOf(filterToToggle);
      enabledFilters.splice(filterIndex, 1);
    } else {
      enabledFilters.push(filterToToggle);
    }
    setCheckedFilters(enabledFilters);
  };

  const submit = (e) => {
    e.preventDefault();
    props.updatePageFilters(checkedDocFilters, checkedTerritoryFilters);
  };

  useEffect(() => {
    if (
      props.rerouteType != null &&
      RerouteType[props.rerouteType.toString()] === RerouteType.CheckboxSelected
    ) {
      filterHeader.current?.scrollIntoView();
      submitButton.current?.focus();
    }
  }, [props.rerouteType]);

  return (
    <StyledSearchFilter>
      <AccessibleHeading>Documents filter</AccessibleHeading>
      <Fieldset>
        <Legend ref={filterHeader}>Filter documents by</Legend>
        <SubLegend ref={filterHeader}>Type of document</SubLegend>
        {generateDocTypeCheckboxFor(
          DocType.Spc,
          'Summary of Product Characteristics',
        )}
        {generateDocTypeCheckboxFor(DocType.Pil, 'Patient Information Leaflet')}
        {generateDocTypeCheckboxFor(DocType.Par, 'Public Assessment Reports')}
        <SubLegend ref={filterHeader}>Applicable to territory</SubLegend>
        {generateTerritoryTypeCheckboxFor(TerritoryType.UK, 'United Kingdom')}
        {generateTerritoryTypeCheckboxFor(TerritoryType.NI, 'Northern Ireland')}
        {generateTerritoryTypeCheckboxFor(TerritoryType.GB, 'Great Britain')}
        <ButtonContainer>
          <Button
            type="submit"
            onClick={submit}
            value="Submit"
            ref={submitButton}
          />
        </ButtonContainer>
      </Fieldset>
    </StyledSearchFilter>
  );
};

export default SearchFilter;
