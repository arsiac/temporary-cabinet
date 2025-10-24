<template>
  <div class="panel">
    <!-- 屏0 -->
    <template v-if="step === 0">
      <div class="title">30 秒临时存包柜</div>
      <div class="chip" :class="{ danger: stats.free === 0 }">
        <el-icon v-if="stats.free === 0"><Warning /></el-icon>
        剩余柜子 <b>{{ stats.free }}/{{ stats.total }}</b>
      </div>
      <el-button
        type="success"
        size="large"
        class="big-btn"
        :disabled="stats.free === 0"
        @click="apply"
      >
        申请柜子
      </el-button>
      <p v-if="stats.free === 0" class="tips">柜子已用完，请等待他人取出后再试</p>
      <el-button link @click="$router.push('/pick')" class="link">我要取件 →</el-button>
    </template>

    <!-- 屏1 存入内容 -->
    <template v-if="step === 1">
      <div class="cabinet-bar">
        <span class="title"
          >柜子 <span class="code">{{ cabinet.code }}</span></span
        >
      </div>

      <div class="step-dot">1 / 4</div>

      <div class="label">存一些消息</div>

      <el-input v-model="text" type="textarea" :rows="4" placeholder="贴一段文字…" class="txt" />
      <div class="label">或者放一些文件</div>

      <!-- 多文件上传 -->
      <el-upload drag multiple :auto-upload="false" :on-change="uploadChange">
        <el-icon class="el-icon--upload"><upload-filled /></el-icon>
        <div class="el-upload__text">拖拽或点击上传文件</div>
        <template #tip>
          <div class="el-upload__tip">单位文件大小不超过 2MB, 文件总大小不超过 10MB</div>
        </template>
      </el-upload>

      <el-button
        type="success"
        size="large"
        class="big-btn"
        :disabled="!text && files.length === 0"
        @click="toLock"
      >
        下一步
      </el-button>
      <el-button link @click="step = 0">← 返回</el-button>
    </template>

    <!-- 屏2 锁柜 -->
    <template v-if="step === 2">
      <div class="step-dot">2 / 4</div>
      <div class="title">锁柜</div>
      <el-input
        v-model="pwd"
        type="password"
        placeholder="输入 4-20 位密码"
        minlength="4"
        maxlength="20"
        show-password
        class="inp"
      />
      <el-input
        v-model.number="hours"
        type="number"
        :min="1"
        :max="168"
        placeholder="保存小时数"
        class="inp"
      >
        <template #append>小时</template>
      </el-input>
      <el-button
        type="success"
        size="large"
        class="big-btn"
        :disabled="!pwd || pwd.length < 4 || !hours"
        @click="lockCabinet"
      >
        锁柜并取走编号
      </el-button>
      <el-button link @click="step = 1">← 返回</el-button>
    </template>

    <!-- 屏3 完成 -->
    <template v-if="step === 3">
      <div class="step-dot green">3 / 4</div>
      <el-result icon="success" title="柜子已锁好">
        <template #sub-title>
          <div style="text-align: left">
            <p>柜子编号：{{ cabinet.code }}</p>
            <p>取件密码：●●●●</p>
            <p>到期时间：{{ dayjs(cabinet.expire_at).format('YYYY-MM-DD HH:mm:ss') }}</p>
          </div>
        </template>
        <template #extra>
          <el-button type="success" @click="share">把编号发给我</el-button>
          <el-button @click="reset">再存一个</el-button>
        </template>
      </el-result>
    </template>
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted } from 'vue';
import { ElMessage } from 'element-plus';
import { getCabinetsUsage, applyCabinet, saveCabinet, getCabinetByCode } from '@/api/cabinet';
import { getPublicKey } from '@/api/crypto';
import { sm2Encrypt } from '@/utils/crypto';
import { inject } from 'vue';
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
    ElMessage.error(e || '申请失败，暂无空柜');
  }
}

function toLock() {
  step.value = 2;
}

function uploadChange(_uploadFile, uploadFiles) {
  files.value = uploadFiles;
}

async function lockCabinet() {
  const pk = await getPublicKey();
  let encryptedPassword = sm2Encrypt(pk, pwd.value);

  try {
    const form = new FormData();
    form.set('hold_token', cabinet.value.hold_token);
    form.set('hours', hours.value);
    form.set('pk', pk);
    form.append('password', `04${encryptedPassword}`);
    form.append('message', text.value);
    files.value.forEach((f) => form.append('files', f.raw));
    await saveCabinet(cabinet.value.code, form);
    cabinet.value = await getCabinetByCode(cabinet.value.code);
    step.value = 3;
  } catch (e) {
    ElMessage.error(e || '锁柜失败，请重试');
  }
}

function share() {
  const url = `${location.origin}/pick?c=${cabinet.value.code}`;
  const text = `柜子编号 ${cabinet.value.code}，链接 ${url}`;
  if (navigator.share) navigator.share({ title: '临时柜子', text });
  else navigator.clipboard.writeText(text);
  ElMessage.success('已复制');
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
