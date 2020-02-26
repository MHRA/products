import { useState } from 'react';

// useSessionStorage and useLocalStorage adapted from
// https://usehooks.com/useLocalStorage/

export const useSessionStorage = (key: string, initialValue: any): any => {
  const [storedValue, setStoredValue] = useState(() => {
    try {
      const item = window.sessionStorage.getItem(key);
      return item ? JSON.parse(item) : initialValue;
    } catch (error) {
      // tslint:disable-next-line: no-console
      console.warn(
        `An unhandled error ocurred while retrieving data from session storage.
This is probably because we can't access the browser window object server-side.
Returning initial value.`,
      );
      return initialValue;
    }
  });

  const setValue = (value: any, force = false) => {
    if (!force && !window.localStorage.getItem('allowStorage')) {
      throw Error(
        "We don't have the user's permission to set a value in session storage.",
      );
    }
    try {
      const valueToStore =
        value instanceof Function ? value(storedValue) : value;
      setStoredValue(valueToStore);
      window.sessionStorage.setItem(key, JSON.stringify(valueToStore));
    } catch (error) {
      // tslint:disable-next-line: no-console
      console.warn(
        `An unhandled error ocurred while setting data in session storage.
This is probably because we can't access the browser window object server-side.`,
      );
    }
  };
  return [storedValue, setValue];
};

export const useLocalStorage = (key: string, initialValue: any): any => {
  const [storedValue, setStoredValue] = useState(() => {
    try {
      const item = window.localStorage.getItem(key);
      return item ? JSON.parse(item) : initialValue;
    } catch (error) {
      // tslint:disable-next-line: no-console
      console.warn(
        `An unhandled error ocurred while retrieving data from local storage.
This is probably because we can't access the browser window object server-side.
Returning initial value.`,
      );
      return initialValue;
    }
  });

  const setValue = (value: any, force = false) => {
    if (!force && !window.localStorage.getItem('allowStorage')) {
      throw Error(
        "We don't have the user's permission to set a value in local storage.",
      );
    }
    try {
      const valueToStore =
        value instanceof Function ? value(storedValue) : value;
      setStoredValue(valueToStore);
      window.localStorage.setItem(key, JSON.stringify(valueToStore));
    } catch (error) {
      // tslint:disable-next-line: no-console
      console.warn(
        `An unhandled error ocurred while setting data in local storage.
This is probably because we can't access the browser window object server-side.`,
      );
    }
  };
  return [storedValue, setValue];
};
