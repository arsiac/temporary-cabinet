/**
 * Copy text to clipboard
 *
 * @param {string} text - Text to be copied to clipboard
 * @returns Promise
 */
export function copyToClipboard(text) {
  if (navigator.clipboard) {
    return navigator.clipboard.writeText(text);
  } else if (document.queryCommandSupported && document.queryCommandSupported('copy')) {
    const input = document.createElement('input');
    input.value = text;
    document.body.appendChild(input);
    input.select();
    const result = document.execCommand('copy');
    document.body.removeChild(input);
    return result ? Promise.resolve() : Promise.reject();
  }
  return Promise.reject();
}
