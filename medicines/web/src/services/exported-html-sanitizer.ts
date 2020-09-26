import parse5 from 'parse5';
import htmlparser2Adapter from 'parse5-htmlparser2-tree-adapter';

const updateImageTag = (imageNode: any, prefix: string): any => {
  for (let i = 0; i < imageNode.attrs.length; i++) {
    if (imageNode.attrs[i].name === 'src') {
      const imageName = imageNode.attrs[i].value.split('/').pop();
      imageNode.attrs[i].value = encodeURI(`${prefix}${imageName}`);
    } else if (imageNode.attrs[i].name === 'v:shapes') {
      imageNode.attrs.splice(i, 1);
      i--;
    }
  }
  return imageNode;
};

const updateAnchorNameToId = (node: any): any => {
  for (const attribute of node.attrs) {
    if (attribute.name === 'name') {
      attribute.name = 'id';
      return node;
    }
  }
  return node;
};

const removeUnwantedTableAttributes = (node: any) => {
  if (!node.attrs) {
    return node;
  }

  const unwantedAttributes = [
    'style',
    'v:shapes',
    'cellspacing',
    'cellpadding',
    'border',
    'width',
    'valign',
    'class',
  ];

  for (let i = 0; i < node.attrs.length; i++) {
    if (unwantedAttributes.includes(node.attrs[i].name)) {
      node.attrs.splice(i, 1);
      i--;
    }
  }

  return node;
};

const removeUnwantedAttributes = (node: any): any => {
  if (!node.attrs) {
    return node;
  }

  const unwantedAttributes = ['style', 'align', 'class'];
  for (let i = 0; i < node.attrs.length; i++) {
    if (unwantedAttributes.includes(node.attrs[i].name)) {
      node.attrs.splice(i, 1);
      i--;
    }
  }
  return node;
};

const tagShouldBeRemoved = (tagName: string) => {
  return ['h1', 'o:p', 'w:sdt'].includes(tagName);
};

export const recurseNodes = (node: any, prefix: string): any => {
  if (tagShouldBeRemoved(node.tagName)) {
    return;
  }
  if (node.tagName === 'img') {
    node = updateImageTag(node, prefix);
  } else if (node.tagName === 'td' || node.tagName === 'table') {
    node = removeUnwantedTableAttributes(node);
  } else if (node.tagName === 'a') {
    node = updateAnchorNameToId(node);
  } else {
    node = removeUnwantedAttributes(node);
  }

  for (let i = 0; i < node.childNodes?.length ?? 0; i++) {
    const returnedNode = recurseNodes(node.childNodes[i], prefix);
    if (returnedNode) {
      node.childNodes[i] = returnedNode;
    } else {
      node.childNodes.splice(i, 1);
      i--;
    }
  }
  return node;
};

export const cleanUpHtml = (htmlBody: any, assetPrefix: string): any => {
  return recurseNodes(htmlBody, assetPrefix);
};

export const getHtmlBody = (htmlDoc: any): any => {
  const html = htmlDoc.childNodes[0];
  for (const node of html.childNodes) {
    if (node.tagName === 'body') {
      return node;
    }
  }
};

export const getCleanedHtml = (rawHtml: string): string => {
  const htmlparser2Adapter = require('parse5-htmlparser2-tree-adapter');
  let html = parse5.parse(rawHtml, {
    scriptingEnabled: false,
    treeAdapter: htmlparser2Adapter,
  });
  console.log(html);
  for (let node of htmlparser2Adapter.getChildNodes(html)[0].children) {
    if (node.tagName === 'head') {
      htmlparser2Adapter.detachNode(node);
    }
  }
  for (let node of htmlparser2Adapter.getChildNodes(html)[0].children) {
    console.log(node);
  }
  return 'ha';
};
