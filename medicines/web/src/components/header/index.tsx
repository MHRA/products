import React from 'react';
import styled from 'styled-components';
import { primaryColor } from '../../styles/colors';
import { baseSpace, desktopMaxWidth } from '../../styles/dimensions';
import SvgMhraLogo from '../logos/mhra-logo';

const mhra = 'Medicines Information';

const Header = styled.header`
  margin: 0 auto;
  max-width: ${desktopMaxWidth};
  padding: ${baseSpace} ${baseSpace} 0;
  border-top: 4px solid ${primaryColor};

  picture {
    max-width: 224px;
    margin-bottom: 115px;
  }

  h1 {
    margin: 0;
    border-bottom: 4px solid ${primaryColor};
    padding-bottom: 0.5rem;
    font-size: 2.25rem;
  }
`;

const header: React.FC = () => (
  <Header>
    <picture>
      <SvgMhraLogo />
    </picture>
    <h1>{mhra}</h1>
  </Header>
);

export default header;
