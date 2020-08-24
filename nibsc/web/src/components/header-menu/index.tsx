import Head from 'next/head';
import Link from 'next/link';
import React, { useEffect } from 'react';
import styled from 'styled-components';
import { Normalize } from 'styled-normalize';

import {
  nibscMainGreen,
  anchorColour,
  white,
  menuHover,
} from '../../styles/colors';
import { desktopMaxWidth, mobileBreakpoint } from '../../styles/dimensions';

const MenuContainer = styled.ul`
  min-height: 30px;
  list-style: none;
  display: flex;
  background-color: ${nibscMainGreen};
  padding: 0;
  font-weight: bold;
  font-size: 1em;
  align-items: stretch;
  justify-items: center;

  li a,
  li a:hover {
    flex: 1;
    text-decoration: none;
    color: ${white};
  }

  @media ${mobileBreakpoint} {
    flex-direction: column;
  }
`;

const StyledMenuItem = styled.li`
  flex: 1;
  text-align: center;
  border-right: 1px solid ${white};
  padding: 5px 0;
  min-height: 100%;
  display: flex;
  align-items: center;

  &:last-child {
    border-right: 0;
  }

  &:hover {
    background-color: ${menuHover};
  }

  @media ${mobileBreakpoint} {
    border-right: 0;
    border-bottom: 1px solid ${white};
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

const HeaderMenu: React.FC = () => {
  return (
    <nav>
      <MenuContainer>
        <MenuItem linkUrl="/">Home</MenuItem>
        <MenuItem linkUrl="/">Products</MenuItem>
        <MenuItem linkUrl="/">Standardisation</MenuItem>
        <MenuItem linkUrl="/">Control testing</MenuItem>
        <MenuItem linkUrl="/">Science and research</MenuItem>
        <MenuItem linkUrl="/">Expert services</MenuItem>
        <MenuItem linkUrl="/">About us</MenuItem>
      </MenuContainer>
    </nav>
  );
};

export default HeaderMenu;
