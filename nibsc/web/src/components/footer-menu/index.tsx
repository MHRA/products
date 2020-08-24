import Link from 'next/link';
import React from 'react';
import styled from 'styled-components';

import {
  nibscMainGreen,
  anchorColour,
  white,
  menuHover,
} from '../../styles/colors';
import { mobileBreakpoint } from '../../styles/dimensions';

const MenuContainer = styled.ul`
  min-height: 30px;
  list-style: none;
  display: flex;
  padding: 0;
  font-weight: bold;
  font-size: 12px;
  align-items: stretch;
  justify-items: center;
  margin-bottom: 38px;
  flex-wrap: wrap;

  li a,
  li a:hover {
    text-decoration: none;
    color: ${nibscMainGreen};
  }
`;

const StyledMenuItem = styled.li`
  flex: 1;
  display: flex;
  align-items: center;
  padding-right: 15px;
  margin-bottom: 10px;
`;

interface MenuItemProps {
  linkUrl: string;
}

const MenuItem: React.FC<MenuItemProps> = (props) => {
  return (
    <StyledMenuItem>
      <Link href={props.linkUrl}>
        <a>{props.children}</a>
      </Link>
    </StyledMenuItem>
  );
};

const FooterMenu: React.FC = () => {
  return (
    <nav>
      <MenuContainer>
        <MenuItem linkUrl="/">Careers</MenuItem>
        <MenuItem linkUrl="/">Terms and conditions</MenuItem>
        <MenuItem linkUrl="/">Accessibility</MenuItem>
        <MenuItem linkUrl="/">Privacy notice</MenuItem>
        <MenuItem linkUrl="/">Cookie information</MenuItem>
        <MenuItem linkUrl="/">Sitemap</MenuItem>
      </MenuContainer>
    </nav>
  );
};

export default FooterMenu;
