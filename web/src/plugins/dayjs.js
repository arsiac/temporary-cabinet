import dayjs from 'dayjs';
import utc from 'dayjs/plugin/utc';
import timezone from 'dayjs/plugin/timezone';

dayjs.extend(utc);
dayjs.extend(timezone);

export default {
  install(app) {
    app.config.globalProperties.$dayjs = dayjs;
    app.provide('dayjs', dayjs);
  },
};
