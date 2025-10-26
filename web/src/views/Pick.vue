<template>
  <div class="panel">
    <div class="title">{{ t('pickup') }}</div>
    <template v-if="step === 1">
      <el-input
        v-model="code"
        type="number"
        :placeholder="t('tips:input-cabinet-code')"
        class="inp"
      />
      <password-input
        v-model="password"
        :placeholder="t('tips:input-pickup-password')"
        class="inp"
      />
      <el-button
        type="success"
        size="large"
        class="big-btn"
        :disabled="!code || !password"
        @click="openCabinet"
      >
        {{ t('open') }}
      </el-button>
    </template>
    <template v-if="step === 2">
      <div v-if="message" class="result msg-box">
        <el-input
          v-model="message"
          type="textarea"
          readonly
          autosize
          resize="none"
          class="msg-textarea"
        />
        <el-button class="copy-btn" size="small" text @click="copyMessage">
          {{ t('copy') }}
        </el-button>
      </div>
      <div v-if="cabinetItems.length" class="result file-panel">
        <div v-for="item in cabinetItems" :key="item.id" class="file-card">
          <div class="file-info">
            <el-icon size="16"><Document /></el-icon>
            <span class="name">{{ item.name }}</span>
            <span class="size">({{ formatSize(item.size) }})</span>
          </div>
          <el-button type="primary" link @click="download(item)">{{ t('download') }}</el-button>
        </div>
      </div>
      <div class="result">
        <el-button class="big-btn" type="danger" @click="clearCabinet">
          {{ t('cleanup-and-delete-cabinet') }}
        </el-button>
      </div>
    </template>
    <el-button link @click="toHome">← {{ t('back-to-home') }}</el-button>
  </div>
</template>

<script setup>
import { ref } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { useI18n } from 'vue-i18n';
import { ElMessage } from 'element-plus';
import { Document } from '@element-plus/icons-vue';
import PasswordInput from '@/components/PasswordInput.vue';
import { getCabinetItems, getCabinetItemContent, deleteCabinet } from '@/api/cabinet';
import { getPublicKey } from '@/api/crypto';
import { sm2Encrypt } from '@/utils/crypto';
import { copyToClipboard } from '@/utils';

const { t } = useI18n();
const route = useRoute();
const router = useRouter();
const step = ref(1);
const code = ref(route.query.c || '');
const password = ref('');
const cabinetItems = ref([]);
const message = ref('');

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
    ElMessage.error(e || t('error:pickup-failed'));
  }
}

function copyMessage() {
  copyToClipboard(message.value)
    .then(() => ElMessage.success(t('copied')))
    .catch(() => ElMessage.error(t('browser-not-support-copy')));
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
    ElMessage.success(t('cabinet-reclaimed'));
    router.push('/');
  } catch (e) {
    ElMessage.error(e || t('error:delete-cabinet-failed'));
  }
}

function toHome() {
  reset();
  router.push('/');
}

function formatSize(size) {
  if (size < 1024) return `${size}B`;
  if (size < 1024 * 1024) return `${(size / 1024).toFixed(2)}KB`;
  if (size < 1024 * 1024 * 1024) return `${(size / 1024 / 1024).toFixed(2)}MB`;
  return `${(size / 1024 / 1024 / 1024).toFixed(2)}GB`;
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
/* 消息一行两端对齐 */
.msg-alert :deep(.el-alert__content) {
  display: flex;
  align-items: center;
  width: 100%;
}
.msg-line {
  display: flex;
  justify-content: space-between;
  align-items: center;
  width: 100%;
}
.msg-text {
  word-break: break-all;
  flex: 1;
  margin-right: 8px;
}
.msg-box {
  position: relative;
}
.msg-textarea {
  background: #f7f8fa;
  border: none;
  box-shadow: inset 0 0 0 1px #e5e7eb;
}
.copy-btn {
  position: absolute;
  right: 8px;
  bottom: 8px;
  color: #909399;
  background: #ffffffcc;
  backdrop-filter: blur(2px);
  border-radius: 4px;
  padding: 2px 6px;
}
.copy-btn:hover {
  color: #00c389;
}
/* 文件卡片 */
.file-panel {
  margin-top: 12px;
}
.file-card {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 10px 12px;
  background: #f7f8fa;
  border-radius: 8px;
  margin-bottom: 8px;
  transition: all 0.2s;
}
.file-card:hover {
  background: #ffffff;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
}
.file-info {
  display: flex;
  align-items: center;
  gap: 6px;
}
.file-info .name {
  font-weight: 500;
}
.file-info .size {
  color: #909399;
  font-size: 12px;
}
</style>
