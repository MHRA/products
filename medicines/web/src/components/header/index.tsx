import React from 'react';
import styled from 'styled-components';
import { primaryColor } from '../../styles/colors';
import { baseSpace, desktopMaxWidth } from '../../styles/dimensions';
import SvgMhraLogo from '../logos/mhra-logo';

const mhra = 'Medicines Information: SPC & PILs';

const Header = styled.header`
  border-bottom: 4px solid ${primaryColor};
  display: flex;
  justify-content: space-between;
  margin: 0 auto;
  max-width: ${desktopMaxWidth};
  padding: ${baseSpace};
`;

const H1 = styled.h1`
  align-self: flex-end;
  margin: 0;
`;

const Picture = styled.picture`
  max-width: 300px;
`;

const header: React.FC = () => (
  <Header>
    <Picture>
      <SvgMhraLogo />
    </Picture>
    <H1>{mhra}</H1>
  </Header>
);

export default header;
