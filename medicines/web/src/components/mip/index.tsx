import React from 'react';
import styled from 'styled-components';
import { baseSpace } from '../../styles/dimensions';
import Search from '../search';

const Row = styled.section`
  display: flex;
  flex-wrap: wrap;
  justify-content: space-between;
  > * {
    flex-basis: 100%;
    flex-shrink: 1;
    flex-grow: 1;
  }
`;

const Aside = styled.aside`
  max-width: 25%;
  padding: ${baseSpace} calc(${baseSpace} / 2) 0 ${baseSpace};
`;

const Main = styled.main`
  max-width: 75%;
  padding: ${baseSpace};
  padding-left: calc(${baseSpace} / 2);
`;

const Mip: React.FC = () => (
  <Row>
    <Aside>
      <Search />
    </Aside>
    <Main />
  </Row>
);

export default Mip;
