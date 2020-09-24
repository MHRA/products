import React, { useEffect } from 'react';
import styled from 'styled-components';

import MedicineLevelsInPregnancyHomeText from '../../components/bmgf/home-text';
import Page from '../../components/page';
import SearchWrapper from '../../components/bmgf/search-wrapper';
import { useLocalStorage } from '../../hooks';
import Events from '../../services/events';
import { mhraPharma10 } from '../../styles/colors';

const StyledHomeTextWrapper = styled.div`
  section:last-child {
    p:last-child {
      background-color: ${mhraPharma10};
      padding: 14px 16px;
    }
  }
`;

const App: React.FC = () => {
  const [storageAllowed, setStorageAllowed] = useLocalStorage(
    'allowStorage',
    false,
  );

  useEffect(() => {
    Events.viewPage('bmgf homepage');
  }, []);

  return (
    <Page
      title="Medicine levels in pregnancy"
      metaTitle="Medicine levels in pregnancy"
      storageAllowed={storageAllowed}
      setStorageAllowed={setStorageAllowed}
    >
      <SearchWrapper initialSearchValue="">
        <StyledHomeTextWrapper>
          <MedicineLevelsInPregnancyHomeText />
        </StyledHomeTextWrapper>
      </SearchWrapper>
    </Page>
  );
};

export default App;
