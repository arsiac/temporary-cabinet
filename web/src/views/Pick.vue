<template>
  <div class="panel">
    <div class="title">取件</div>

    <el-input v-model="id" placeholder="输入柜子编号，如 8F2C" class="inp" />
    <el-input v-model="pwd" type="password" placeholder="输入取件密码" class="inp" />
    <el-button
      type="success"
      size="large"
      class="big-btn"
      :disabled="!id || !pwd"
      @click="openCabinet"
    >
      打开
    </el-button>

    <div v-if="cabinet.text || cabinet.file" class="result">
      <el-divider />
      <el-input v-if="cabinet.text" v-model="cabinet.text" type="textarea" :rows="4" readonly />
      <div v-if="cabinet.file" class="file">
        <el-icon><Document /></el-icon> {{ cabinet.file }}
      </div>
      <el-button type="danger" class="big-btn" @click="clearCabinet">清空并回收</el-button>
    </div>

    <el-button link @click="$router.push('/')">← 返回首页</el-button>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue';
import { useRoute } from 'vue-router';
import { getCabinetByCode } from '@/api/cabinet';
import { ElMessage } from 'element-plus';

const route = useRoute();
const code = ref(route.query.c || '');
const pwd = ref('');
const cabinet = ref({});

onMounted(() => {
  if (code.value) ElMessage.info('已填入编号，请输入密码');
});

async function openCabinet() {
  try {
    const c = await getCabinetByCode(code.value);
    if (!c.used) return ElMessage.error('柜子不存在');
    if (c.password !== pwd.value) return ElMessage.error('密码错误');
    if (Date.now() > c.expireAt) {
      ElMessage.error('柜子已过期');
      return clearCabinet();
    }
    cabinet.value = c;
  } catch {
    ElMessage.error('柜子不存在');
  }
}

async function clearCabinet() {
  ElMessage.success('柜子已回收');
  code.value = '';
  pwd.value = '';
  cabinet.value = {};
}
</script>

<style scoped>
.panel {
  width: 100%;
  max-width: 440px;
  background: white;
  border-radius: 12px;
  padding: 32px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.08);
}
.title {
  font-size: 24px;
  text-align: center;
  margin-bottom: 16px;
}
.inp {
  margin-bottom: 12px;
}
.big-btn {
  width: 100%;
  margin: 12px 0;
}
.result {
  margin-top: 12px;
}
.file {
  display: flex;
  align-items: center;
  gap: 6px;
  margin: 12px 0;
}
</style>
