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
  const [checkedFilters, setCheckedFilters] = React.useState(props.currentlyEnabledDocTypes)
  const generateCheckboxFor = (docType: DocType, name: string) => (
    <DocTypeCheckbox toggleDocType={ toggleDocType} currentlyEnabledDocTypes={checkedFilters} docTypeForThisCheckbox={docType} name={name} />
  );

const toggleDocType = docTypeToToggle => {
  const enabledDocTypes = Array.from(checkedFilters);
  if (enabledDocTypes.includes(docTypeToToggle)) {
    const docTypeIndex = enabledDocTypes.indexOf(docTypeToToggle);
    enabledDocTypes.splice(docTypeIndex, 1);
  } else {
    enabledDocTypes.push(docTypeToToggle);
  }
  setCheckedFilters(enabledDocTypes);
  console.log(enabledDocTypes)
}

const submit = () => {
  console.log("submitted");
}

  return (
    <StyledSearchFilter>
      <h2>Filter documents by</h2>
      {generateCheckboxFor(DocType.Spc, 'Summary of Product Characteristics')}
      {generateCheckboxFor(DocType.Pil, 'Patient Information Leaflet')}
      {generateCheckboxFor(DocType.Par, 'Public Assessment Reports')}
      <button onClick={submit}>Submit</button>
    </StyledSearchFilter>
  );
};

export default SearchFilter;
