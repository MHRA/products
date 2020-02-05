import path from 'path';
export default {
  process(
    // tslint:disable-next-line: variable-name
    _src: string | null | undefined,
    filename: string,
    // tslint:disable-next-line: variable-name
    _config: string | null | undefined,
    // tslint:disable-next-line: variable-name
    _options: string | null | undefined,
  ) {
    return 'module.exports = ' + JSON.stringify(path.basename(filename)) + ';';
  },
};
