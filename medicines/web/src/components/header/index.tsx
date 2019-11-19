import React from 'react';
import SvgAgencyDigitalLogo from '../logos/mhra';

const header: React.FC = () => (
  <header>
    <picture>
      <SvgAgencyDigitalLogo />
    </picture>
    <h1>Medicines Information: SPC & PILs</h1>
  </header>
);

export default header;
