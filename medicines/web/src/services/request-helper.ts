export const requestTimeout = (
  timeoutMs: number,
  promise: Promise<any>,
): Promise<any> => {
  const timeout = new Promise((resolve, reject) => {
    setTimeout(() => {
      reject(new Error('Request timed out'));
    }, timeoutMs);
  });
  return Promise.race([promise, timeout]);
};
