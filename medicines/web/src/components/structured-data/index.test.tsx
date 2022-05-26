import renderer from 'react-test-renderer';
import React from 'react';
import {
  DrugListStructuredData,
  DrugStructuredData,
  SubstanceListStructuredData,
  SubstanceStructuredData,
} from './index';

describe(SubstanceStructuredData, () => {
  it('should render', () => {
    const component = renderer
      .create(<SubstanceStructuredData substanceName={'My cool substance'} />)
      .toJSON();
    expect(component).toMatchSnapshot();
  });
});

describe(SubstanceListStructuredData, () => {
  it('should render', () => {
    const component = renderer
      .create(
        <SubstanceListStructuredData
          substanceNames={['First cool substance', 'Second cool substance']}
        />,
      )
      .toJSON();
    expect(component).toMatchSnapshot();
  });
});

describe(DrugStructuredData, () => {
  it('should render', () => {
    const component = renderer
      .create(<DrugStructuredData drugName={'My cool drug'} />)
      .toJSON();
    expect(component).toMatchSnapshot();
  });
});

describe(DrugListStructuredData, () => {
  it('should render', () => {
    const component = renderer
      .create(
        <DrugListStructuredData
          drugNames={['First cool drug', 'Second cool drug']}
        />,
      )
      .toJSON();
    expect(component).toMatchSnapshot();
  });
});
