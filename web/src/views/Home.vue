<template>
  <div class="panel">
    <!-- Home -->
    <template v-if="step === 0">
      <div class="title">{{ t('temporary-cabinet') }}</div>
      <div class="chip" :class="{ danger: stats.free === 0 }">
        <el-icon v-if="stats.free === 0"><Warning /></el-icon>
        {{ t('cabinets-remain') }} <b>{{ stats.free }}/{{ stats.total }}</b>
      </div>
      <el-button
        type="success"
        size="large"
        class="big-btn"
        :disabled="stats.free === 0"
        @click="apply"
      >
        {{ t('apply-cabinet') }}
      </el-button>
      <p v-if="stats.free === 0" class="tips">{{ t('no-avaliable-cabinet-tips') }}</p>
      <el-button link @click="$router.push('/pick')" class="link"
        >{{ t('pickup-items') }} →</el-button
      >
    </template>

    <!-- Step 1: Store -->
    <template v-if="step === 1">
      <div class="cabinet-bar">
        <span class="title"
          >{{ t('cabinet') }} <span class="code">{{ cabinet.code }}</span></span
        >
      </div>
      <div class="step-dot">1 / 4</div>
      <div class="label">{{ t('label:store-some-message') }}</div>
      <el-input
        v-model="text"
        type="textarea"
        :rows="4"
        :placeholder="t('tips:input-some-text')"
        class="txt"
        maxlength="500"
        show-word-limit
      />
      <div class="label">{{ t('label:store-some-files') }}</div>
      <el-upload drag multiple :auto-upload="false" :on-change="onUploadChange">
        <el-icon class="el-icon--upload"><upload-filled /></el-icon>
        <div class="el-upload__text">{{ t('upload-file-placeholder') }}</div>
        <template #tip>
          <div class="el-upload__tip">{{ t('upload-file-tips') }}</div>
        </template>
      </el-upload>
      <el-button
        type="success"
        size="large"
        class="big-btn"
        :disabled="!text && files.length === 0"
        @click="toLock"
      >
        {{ t('next-step') }}
      </el-button>
      <el-button link @click="step = 0">← {{ t('last-step') }}</el-button>
    </template>

    <!-- Step 2: Lock -->
    <template v-if="step === 2">
      <div class="step-dot">2 / 4</div>
      <div class="title">{{ t('lock-cabinet') }}</div>
      <password-input v-model="pwd" :placeholder="t('tips:input-password')" class="inp" />
      <el-input
        v-model.number="hours"
        type="number"
        :min="1"
        :max="168"
        :placeholder="t('tips:keep-hours')"
        class="inp"
      >
        <template #append>{{ t('hours') }}</template>
      </el-input>
      <el-button
        type="success"
        size="large"
        class="big-btn"
        :disabled="!pwd || pwd.length < 4 || !hours"
        @click="lockCabinet"
      >
        {{ t('button:lock-cabinet') }}
      </el-button>
      <el-button
        link
        @click="
          () => {
            files = [];
            step = 1;
          }
        "
        >← {{ t('last-step') }}</el-button
      >
    </template>

    <!-- Step 3: Finish -->
    <template v-if="step === 3">
      <div class="step-dot green">3 / 4</div>
      <el-result icon="success" :title="t('cabinet-locked')">
        <template #sub-title>
          <div style="text-align: left">
            <p>{{ t('cabinet-code') }}: {{ cabinet.code }}</p>
            <p>{{ t('password') }}: ●●●●●●</p>
            <p>
              {{ t('expire-time') }}: {{ dayjs(cabinet.expire_at).format('YYYY-MM-DD HH:mm:ss') }}
            </p>
          </div>
        </template>
        <template #extra>
          <el-button type="success" @click="share">{{ t('copy-pickup-link') }}</el-button>
          <el-button @click="reset">{{ t('apply-another-cabinet') }}</el-button>
        </template>
      </el-result>
    </template>
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted } from 'vue';
import { ElMessage } from 'element-plus';
import { Warning, UploadFilled } from '@element-plus/icons-vue';
import PasswordInput from '@/components/PasswordInput.vue';
import { getCabinetsUsage, applyCabinet, saveCabinet } from '@/api/cabinet';
import { getPublicKey } from '@/api/crypto';
import { sm2Encrypt } from '@/utils/crypto';
import { inject } from 'vue';
import { useI18n } from 'vue-i18n';

const { t } = useI18n();
const dayjs = inject('dayjs');
const step = ref(0);
const stats = ref({ total: 0, used: 0, free: 0 });
const text = ref('');
const files = ref([]);
const pwd = ref('');
const hours = ref(1);
const cabinet = ref({});

onMounted(() => fetchUsage());

let timer = setInterval(fetchUsage, 30_000);
onUnmounted(() => clearInterval(timer));

async function fetchUsage() {
  stats.value = await getCabinetsUsage();
}

async function apply() {
  try {
    const c = await applyCabinet();
    cabinet.value = c;
    step.value = 1;
  } catch (e) {
    ElMessage.error(e || t('error:cabinet:apply-failed'));
  }
}

function toLock() {
  for (let file in files.value) {
    if (!checkFileSize(file)) {
      return;
    }
  }
  if (!checkFilesSize(files.value)) {
    return;
  }
  step.value = 2;
}

function checkFileSize(uploadFile) {
  if (uploadFile.size > 2 * 1024 * 1024) {
    const fileSize = uploadFile.size / 1024 / 1024;
    ElMessage.error(
      t('error:cabinet:file-exceeds', { filename: uploadFile.name, size: fileSize.toFixed(2) })
    );
    return false;
  }
  return true;
}

function checkFilesSize(uploadFiles) {
  let totalSize = uploadFiles.reduce((acc, f) => acc + f.size, 0);
  if (totalSize > 10 * 1024 * 1024) {
    const totalFileSize = totalSize / 1024 / 1024;
    ElMessage.error(t('error:cabinet:total-file-exceeds', { size: totalFileSize.toFixed(2) }));
    return false;
  }
  return true;
}

function onUploadChange(uploadFile, uploadFiles) {
  if (checkFileSize(uploadFile)) {
    checkFilesSize(uploadFiles);
  }
  files.value = uploadFiles;
}

async function lockCabinet() {
  const pk = await getPublicKey();
  let encryptedPassword = sm2Encrypt(pk, pwd.value);
  try {
    const form = new FormData();
    form.set('hold_token', cabinet.value.hold_token);
    form.set('hours', hours.value);
    form.set('public_key', pk);
    form.append('password', encryptedPassword);
    form.append('message', text.value);
    files.value.forEach((f) => form.append('files', f.raw));
    cabinet.value = await saveCabinet(cabinet.value.code, form);
    step.value = 3;
  } catch (e) {
    ElMessage.error(e || t('error:cabinet:lock-failed'));
  }
}

function share() {
  const url = `${location.origin}/pick?c=${cabinet.value.code}`;
  const text = t('cabinet:link-info', { code: cabinet.value.code, link: url });
  if (navigator.clipboard) {
    try {
      navigator.clipboard.writeText(text);
      ElMessage.success(t('copied'));
    } catch (e) {
      ElMessage.error(t('error:pickup-failed') + ' ' + e.message);
    }
  } else {
    ElMessage.error(t('browser-not-support-copy'));
  }
}

function reset() {
  text.value = '';
  files.value = [];
  pwd.value = '';
  hours.value = 1;
  step.value = 0;
  fetchUsage();
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
  margin-bottom: 12px;
}
.chip {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  background: #f3f4f6;
  color: #6b7280;
  padding: 4px 8px;
  border-radius: 8px;
  font-size: 14px;
  margin: 0 auto 24px;
}
.chip.danger {
  background: #fee2e2;
  color: #ef4444;
}
.big-btn {
  width: 100%;
  margin: 12px 0;
}
.tips {
  color: #ef4444;
  font-size: 12px;
  text-align: center;
  margin-top: -8px;
  margin-bottom: 8px;
}
.link {
  margin-top: 12px;
  font-size: 13px;
}
.step-dot {
  text-align: center;
  font-size: 13px;
  color: #999;
  margin-bottom: 8px;
}
.step-dot.green {
  color: #00c389;
}
.drop,
.txt {
  margin-bottom: 16px;
}
.inp {
  margin-bottom: 12px;
}

/* 1. 柜子编号 - 卡片式高亮 */
.cabinet-bar {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  font-size: 15px;
  margin: 0 auto 20px;
  width: 100%;
}
.cabinet-bar .title {
  color: #000; /* 黑色 */
  font-weight: normal;
  margin-bottom: 0;
}
.cabinet-bar .code {
  margin-left: 6px;
  font-size: 20px;
  font-weight: bold;
  font-family: Menlo, monospace;
  color: var(--el-color-success);
}

/* 2. 步骤点 */
.step-dot {
  text-align: center;
  font-size: 13px;
  color: #909399;
  margin-bottom: 8px;
}

/* 3. label 主次区分 */
.label {
  font-size: 15px;
  color: #303133;
  margin: 16px 0 8px;
  font-weight: 500;
  position: relative;
  padding-left: 12px;
}
.label::before {
  /* 左侧色条 */
  content: '';
  position: absolute;
  left: 0;
  top: 50%;
  transform: translateY(-50%);
  width: 4px;
  height: 16px;
  background: #00c389;
  border-radius: 2px;
}
.label:nth-of-type(2) {
  /* 第二个 label 降一级 */
  font-size: 14px;
  color: #606266;
}
.label:nth-of-type(2)::before {
  background: #dcdfe6;
}

/* 4. 上传区域微调 */
.el-upload__text {
  font-size: 14px;
  color: #606266;
}
.el-upload__tip {
  font-size: 12px;
  color: #909399;
  margin-top: 4px;
}

/* 5. 按钮保持原样 */
.big-btn {
  width: 100%;
  margin: 20px 0 8px;
}
</style>
