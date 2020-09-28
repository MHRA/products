import parse5 from 'parse5';

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

const removeUnwantedTableAttributes = (node: any): any => {
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

const tagShouldBeRemoved = (tagName: string): boolean => {
  return ['h1', 'o:p', 'script'].includes(tagName);
};

const replaceWsdtTags = (node: any): any => {
  if (node.tagName?.startsWith('w:sdt')) {
    node.tagName = 'div';
  }
  return node;
};

export const recurseNodes = (node: any, prefix: string): any => {
  if (tagShouldBeRemoved(node.tagName)) {
    return;
  }
  replaceWsdtTags(node);
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

export const getHtmlBody = (htmlDoc: any): any => {
  for (const node of htmlDoc.childNodes) {
    if (!node.childNodes) {
      continue;
    }
    for (const childNode of node.childNodes) {
      if (childNode.tagName === 'body') {
        return childNode;
      }
    }
  }
};

export const getCleanedHtml = (
  rawHtml: string,
  assetPrefix: string,
): string | undefined => {
  const html = parse5.parse(rawHtml);

  const htmlBody = getHtmlBody(html);

  if (!htmlBody) {
    return;
  }

  const cleanedHtml = recurseNodes(htmlBody, assetPrefix);

  if (!cleanedHtml) {
    return;
  }

  return parse5.serialize(cleanedHtml);
};
