import Link from 'next/link';
import React from 'react';
import styled from 'styled-components';

import { nibscAccessibleGreen, white } from '../../styles/colors';
import { mobileBreakpoint } from '../../styles/dimensions';

const MenuContainer = styled.ul`
  list-style: none;
  padding: 0;
  font-size: 12px;
  margin: 0 0 38px 5px;
  display: inline-block;

  li a,
  li a:hover {
    color: ${white};
  }

  @media ${mobileBreakpoint} {
    text-align: center;
  }
`;

const StyledMenuItem = styled.li`
  display: inline-block;
  margin-right: 26px;

  @media ${mobileBreakpoint} {
    margin-bottom: 10px;
  }
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
