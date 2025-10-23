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
      return response.data;
    }
    const { message } = response.data;
    ElMessage.error(message || '接口异常');
    return Promise.reject(new Error(message));
  },
  (error) => {
    const message = error.response?.data?.message || error.message || '网络错误';
    ElMessage.error(message);
    return Promise.reject(error);
  }
);

export default request;
