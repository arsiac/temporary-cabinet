import request from '@/utils/request';

/**
 * Apply for a cabinet
 *
 * @returns {object} cabinet
 * @returns {number} cabinet.code - Cabinet code
 * @returns {string} cabinet.name - Cabinet name
 * @returns {string} cabinet.description - Cabinet description
 * @returns {string} cabinet.status - Cabinet status
 * @returns {string} cabinet.hold_token - Cabinet hold token
 * @returns {string} cabinet.expire_at - Cabinet hold/occupied expiration time
 */
export function applyCabinet() {
  return request({
    url: '/api/cabinet/apply',
    method: 'post',
  });
}

/**
 * Get cabinet by code
 *
 * @param {number} cabinetCode
 * @returns {object} cabinet
 * @returns {number} cabinet.code - Cabinet code
 * @returns {string} cabinet.name - Cabinet name
 * @returns {string} cabinet.description - Cabinet description
 * @returns {string} cabinet.status - Cabinet status
 * @returns {string} cabinet.hold_token - Cabinet hold token
 * @returns {string} cabinet.expire_at - Cabinet hold/occupied expiration time
 */
export function getCabinetByCode(cabinetCode) {
  return request({
    url: `/api/cabinet/${cabinetCode}`,
    method: 'get',
  });
}

/**
 * Get cabinets status
 *
 * @returns {object} status - Cabinets Status
 * @returns {number} status.total - Total number of cabinets
 * @returns {number} status.used - Number of used cabinets
 * @returns {number} status.free - Number of free cabinets
 */
export function getCabinetsUsage() {
  return request({
    url: '/api/cabinet/usage',
    method: 'get',
  });
}

/**
 * Save cabinet items
 *
 * @param {number} cabinetCode - Cabinet code
 * @param {FormData} form - Cabinet data
 * @param {string} form.message - Message in the cabinet
 * @param {File} form.files - Files to save in the cabinet
 * @param {string} form.password - Password to encrypt the cabinet
 * @param {number} form.hours - Hours to hold the cabinet
 * @param {string} form.hold_token - Hold token
 * @returns {object} cabinet
 * @returns {number} cabinet.code - Cabinet code
 * @returns {string} cabinet.name - Cabinet name
 * @returns {string} cabinet.description - Cabinet description
 * @returns {string} cabinet.status - Cabinet status
 * @returns {string} cabinet.hold_token - Cabinet hold token
 * @returns {string} cabinet.expire_at - Cabinet hold/occupied expiration time
 */
export function saveCabinet(cabinetCode, form) {
  return request({
    url: `/api/cabinet/${cabinetCode}`,
    method: 'post',
    data: form,
  });
}

/**
 * Get cabinet items
 *
 * @param {number} cabinetCode - Cabinet Code
 * @param {object} credential - Credential
 * @param {string} credential.password - Password to decrypt the cabinet
 * @param {string} credential.public_key - Public key to encrypt the password
 * @returns {Array} items - Items in the cabinet
 */
export function getCabinetItems(cabinetCode, credential) {
  return request({
    url: `/api/cabinet/${cabinetCode}/items`,
    method: 'post',
    data: credential,
  });
}

/**
 * Get cabinet item content
 *
 * @param {number} cabinetCode - Cabinet Code
 * @param {number} itemId - Item Id
 * @param {string} mode - download mode, can be 'text' or 'file'
 * @param {object} credential - Credential
 * @param {string} credential.password - Password to decrypt the cabinet
 * @param {string} credential.public_key - Public key to encrypt the password
 * @returns text or download link
 */
export function getCabinetItemContent(cabinetCode, itemId, mode, credential) {
  const url = `/api/cabinet/${cabinetCode}/item/${itemId}/content?mode=${mode}`;
  if (mode === 'text') {
    return request({
      url,
      method: 'post',
      data: credential,
    });
  } else if (mode === 'file') {
    return request({
      url,
      method: 'post',
      data: credential,
      responseType: 'blob',
    });
  }
}
