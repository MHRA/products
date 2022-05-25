import renderer from 'react-test-renderer';
import React from 'react';
import { IFacet } from '../../../model/facet';
import DrugIndex, { IndexType } from './index';

describe(DrugIndex, () => {
  it('should render horizontal items', () => {
    const component = renderer
      .create(
        <DrugIndex
          title={'Coffee'}
          indexType={IndexType.Horizontal}
          items={[
            { name: 'A' },
            { name: 'B' },
            { name: 'C' },
            { name: 'D' },
            { name: 'E' },
            { name: 'F' },
            { name: 'G' },
            { name: 'H' },
            { name: 'I' },
            { name: 'J' },
            { name: 'K' },
            { name: 'L' },
            { name: 'M' },
            { name: 'N' },
            { name: 'O' },
            { name: 'P' },
            { name: 'Q' },
            { name: 'R' },
            { name: 'S' },
            { name: 'T' },
            { name: 'U' },
            { name: 'V' },
            { name: 'W' },
            { name: 'X' },
            { name: 'Y' },
            { name: 'Z' },
            { name: '0' },
            { name: '1' },
            { name: '2' },
            { name: '3' },
            { name: '4' },
            { name: '5' },
            { name: '6' },
            { name: '7' },
            { name: '8' },
            { name: '9' },
          ]}
        />,
      )
      .toJSON();
    expect(component).toMatchSnapshot();
  });
  it('should render substances', () => {
    const substance: IFacet = {
      name: 'Ibuprofen',
      count: 1,
    };
    const substances = [substance];
    const component = renderer
      .create(
        <DrugIndex
          title={'Coffee'}
          items={substances}
          indexType={IndexType.SubstancesIndex}
        />,
      )
      .toJSON();
    expect(component).toMatchSnapshot();
  });
});
