import { shallow } from 'enzyme';
import React from 'react';
import {
  DrugListStructuredData,
  DrugStructuredData,
  SubstanceListStructuredData,
  SubstanceStructuredData,
} from './index';

describe(SubstanceStructuredData, () => {
  it('should render', () => {
    const component = shallow(
      <SubstanceStructuredData substanceName={'My cool substance'} />,
    );
    expect(component).toMatchSnapshot();
  });
});

describe(SubstanceListStructuredData, () => {
  it('should render', () => {
    const component = shallow(
      <SubstanceListStructuredData
        substanceNames={['First cool substance', 'Second cool substance']}
      />,
    );
    expect(component).toMatchSnapshot();
  });
});

describe(DrugStructuredData, () => {
  it('should render', () => {
    const component = shallow(<DrugStructuredData drugName={'My cool drug'} />);
    expect(component).toMatchSnapshot();
  });
});

describe(DrugListStructuredData, () => {
  it('should render', () => {
    const component = shallow(
      <DrugListStructuredData
        drugNames={['First cool drug', 'Second cool drug']}
      />,
    );
    expect(component).toMatchSnapshot();
  });
});
