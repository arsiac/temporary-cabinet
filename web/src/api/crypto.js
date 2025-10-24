import request from '@/utils/request';

/**
 * Get public key
 *
 * @returns {string} publicKey - Public key
 */
export function getPublicKey() {
  return request({
    url: '/api/crypto/pk',
    method: 'get',
  });
}
