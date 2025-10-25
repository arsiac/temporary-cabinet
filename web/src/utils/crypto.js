import { sm2 } from 'sm-crypto';

/**
 * Encrypt message using SM2
 *
 * @param {string} publicKeyHex - public key in hex format
 * @param {string} message - message to encrypt
 * @returns {string} - Encrypted message in hex
 */
export function sm2Encrypt(publicKeyHex, message) {
  let encrypted = sm2.doEncrypt(message, publicKeyHex, 1);
  return '04' + encrypted;
}
