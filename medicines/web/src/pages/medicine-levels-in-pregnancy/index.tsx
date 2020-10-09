import React, { useEffect } from 'react';
import styled from 'styled-components';

import MedicineLevelsInPregnancyHomeText from '../../components/bmgf/home-text';
import { BmgfPage } from '../../components/page';
import SearchWrapper from '../../components/bmgf/search-wrapper';
import { useLocalStorage } from '../../hooks';
import Events from '../../services/events';

const StyledHomeTextWrapper = styled.div`
  section:last-child {
    p:last-child {
      padding: 14px 0;
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
    <BmgfPage
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
    </BmgfPage>
  );
};

export default App;
