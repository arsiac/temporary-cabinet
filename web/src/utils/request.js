import axios from 'axios';
import { ElMessage } from 'element-plus';

const request = axios.create({
  baseURL: import.meta.env.VITE_BASE_API || '/',
  timeout: 10 * 1000,
});

// Response interceptor
request.interceptors.response.use(
  (response) => {
    if (response.status === 200) {
      if (response.request.responseType === 'blob') {
        return {
          filename: response.headers['content-disposition'].split('filename=')[1],
          data: response.data,
        };
      }
      return response.data;
    }
    const { message } = response.data;
    return Promise.reject(new Error(message));
  },
  (error) => {
    const message = error.response?.data?.message || error.message;
    if (!message) {
      ElMessage.error('Network error');
    }
    return Promise.reject(message || error);
  }
);

export default request;
