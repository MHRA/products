import React from 'react';
import styled from 'styled-components';
import {
  baseSpace,
  desktopMaxWidth,
  mobileBreakpoint,
} from '../../styles/dimensions';
import DrugIndex from '../drug-index';
import DrugList, { IDrug } from '../drug-list';
import MipText from '../mip-text';
import Pdf from '../pdf';
import Search from '../search';
import YelllowCard from '../yellow-card';

const Row = styled.section`
  display: flex;
  flex-wrap: wrap;
  justify-content: space-between;
  margin: 0 auto;
  max-width: ${desktopMaxWidth};
  > * {
    flex-basis: 100%;
    flex-shrink: 1;
    flex-grow: 1;
  }
`;

const Aside = styled.aside`
  max-width: 25%;
  padding: ${baseSpace} calc(${baseSpace} / 2) 0 ${baseSpace};

  @media ${mobileBreakpoint} {
    max-width: 100%;

    .pdf-yellow-card-wrapper {
      display: none;
    }
  }
`;

const Main = styled.main`
  max-width: 75%;
  padding: ${baseSpace};
  padding-left: calc(${baseSpace} / 2);

  .pdf-yellow-card-wrapper {
    display: none;
  }

  @media ${mobileBreakpoint} {
    max-width: 100%;

    .pdf-yellow-card-wrapper {
      display: block;
    }
  }
`;

const drugs: IDrug[] = [
  {
    name: 'ABACAVIR',
    url: 'none',
  },
  {
    name: 'ABACAVIR',
    url: 'none',
  },
  {
    name: 'ABACAVIR',
    url: 'none',
  },
  {
    name: 'ABACAVIR',
    url: 'none',
  },
];

const Mip: React.FC = () => (
  <Row>
    <Aside>
      <Search />
      <div className="pdf-yellow-card-wrapper">
        <Pdf />
        <YelllowCard />
      </div>
    </Aside>
    <Main>
      <MipText />
      <DrugIndex />
      <DrugList drugs={drugs} />
      <div className="pdf-yellow-card-wrapper">
        <Pdf />
        <YelllowCard />
      </div>
    </Main>
  </Row>
);

export default Mip;
