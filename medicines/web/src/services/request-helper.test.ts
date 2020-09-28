import { requestTimeout } from './request-helper';

describe(requestTimeout, () => {
  it('returns promise if promise resolves in time', () => {
    expect.assertions(1);
    const promise = new Promise((resolve, reject) => {
      setTimeout(() => resolve('Completed'), 50);
    });
    return expect(requestTimeout(100, promise)).resolves.toEqual('Completed');
  });
  it('rejects promise if timeout exceeded', () => {
    expect.assertions(1);
    const promise = new Promise((resolve, reject) => {
      setTimeout(() => resolve('Completed'), 100);
    });
    return expect(requestTimeout(50, promise)).rejects.toThrow(
      'Request timed out',
    );
  });
});
