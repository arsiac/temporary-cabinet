<template>
  <div class="panel">
    <div class="title">取件</div>
    <template v-if="step === 1">
      <el-input v-model="code" placeholder="输入柜子号码" class="inp" />
      <password-input v-model="password" placeholder="输入取件密码" class="inp" />
      <el-button
        type="success"
        size="large"
        class="big-btn"
        :disabled="!code || !password"
        @click="openCabinet"
      >
        打开
      </el-button>
    </template>
    <template v-if="step === 2">
      <!-- 1. 文本消息 -->
      <el-alert v-if="message" :title="message" type="info" :closable="false" class="result">
        <template #default>
          <div class="msg-line">
            <el-button size="small" text @click="copyMessage">复制</el-button>
          </div>
        </template>
      </el-alert>

      <!-- 2. 文件列表 -->
      <div v-if="cabinetItems.length" class="result">
        <div class="file" v-for="item in cabinetItems" :key="item.id">
          <span class="file-item">
            <el-icon><Document /></el-icon>
            <span class="name">{{ item.name }}</span>
          </span>
          <span class="file-item">
            <!-- <span class="size">({{ formatSize(f.size) }})</span> -->
            <el-button type="primary" link size="small" @click="download(item)"> 下载 </el-button>
          </span>
        </div>
      </div>

      <!-- 3. 销毁 -->
      <div class="result">
        <el-button type="danger" class="big-btn" @click="clearCabinet"> 清空并回收柜子 </el-button>
      </div>
    </template>
    <el-button link @click="toHome">← 返回首页</el-button>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { ElMessage } from 'element-plus';
import { Document } from '@element-plus/icons-vue';
import PasswordInput from '@/components/PasswordInput.vue';
import { getCabinetItems, getCabinetItemContent, deleteCabinet } from '@/api/cabinet';
import { getPublicKey } from '@/api/crypto';
import { sm2Encrypt } from '@/utils/crypto';

const route = useRoute();
const router = useRouter();
const step = ref(1);
const code = ref(route.query.c || '');
const password = ref('');
const cabinetItems = ref([]);
const message = ref('');

onMounted(() => {
  if (code.value) ElMessage.info('已填入编号，请输入密码');
});

async function openCabinet() {
  let pk = await getPublicKey();
  let encryptedPassword = sm2Encrypt(pk, password.value);
  try {
    let credential = {
      password: encryptedPassword,
      public_key: pk,
    };
    const items = await getCabinetItems(code.value, credential);
    cabinetItems.value = [];
    for (const item of items) {
      if (item.category === 'Text') {
        pk = await getPublicKey();
        encryptedPassword = sm2Encrypt(pk, password.value);
        credential = {
          password: encryptedPassword,
          public_key: pk,
        };
        message.value = await getCabinetItemContent(item.cabinet_code, item.id, 'text', credential);
      } else {
        cabinetItems.value.push(item);
      }
    }
    cabinetItems.value = items.filter((item) => item.category === 'File');
    step.value = 2;
  } catch (e) {
    ElMessage.error(e || '取件失败');
  }
}

function copyMessage() {
  navigator.clipboard.writeText(message.value);
  ElMessage.success('已复制');
}

async function download(item) {
  const pk = await getPublicKey();
  let encryptedPassword = sm2Encrypt(pk, password.value);
  let credential = {
    password: encryptedPassword,
    public_key: pk,
  };
  const { filename, data } = await getCabinetItemContent(
    item.cabinet_code,
    item.id,
    'file',
    credential
  );
  const blob = new Blob([data]);
  const a = document.createElement('a');
  a.href = URL.createObjectURL(blob);
  a.download = filename;
  a.click();
  URL.revokeObjectURL(a.href);
}

function reset() {
  code.value = '';
  password.value = '';
  cabinetItems.value = [];
  message.value = '';
}

async function clearCabinet() {
  const pk = await getPublicKey();
  let encryptedPassword = sm2Encrypt(pk, password.value);
  let credential = {
    password: encryptedPassword,
    public_key: pk,
  };
  try {
    await deleteCabinet(code.value, credential);
    reset();
    ElMessage.success('柜子已回收');
    router.push('/');
  } catch (e) {
    ElMessage.error(e || '回收失败');
  }
}

function toHome() {
  reset();
  ElMessage.success('已返回首页');
  router.push('/');
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
  justify-content: space-between;
}

.file .file-item {
  display: flex;
  align-items: center;
  gap: 6px;
}
</style>
