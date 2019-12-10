import { useState } from 'react';

export const useSessionStorage = (key: string, initialValue: any): any => {
  const [storedValue, setStoredValue] = useState(() => {
    try {
      const item = window.sessionStorage.getItem(key);
      return item ? JSON.parse(item) : initialValue;
    } catch (error) {
      // tslint:disable-next-line: no-console
      console.error(error);
      return initialValue;
    }
  });

  const setValue = (value: any) => {
    try {
      const valueToStore =
        value instanceof Function ? value(storedValue) : value;
      setStoredValue(valueToStore);
      window.sessionStorage.setItem(key, JSON.stringify(valueToStore));
    } catch (error) {
      // tslint:disable-next-line: no-console
      console.error(error);
    }
  };
  return [storedValue, setValue];
};
