export const requestTimeout = (
  timeoutMs: number,
  promise: Promise<any>,
): Promise<any> => {
  return new Promise((resolve, reject) => {
    setTimeout(() => {
      reject(new Error('Request timed out'));
    }, timeoutMs);
    promise.then(resolve, reject);
  });
};
