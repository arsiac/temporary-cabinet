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
      <div class="step-dot">1 / 4</div>
      <div class="title">把东西丢进来</div>

      <el-upload drag :limit="1" :before-upload="beforeUpload" class="drop">
        <el-icon class="el-icon--upload"><upload-filled /></el-icon>
        <div>点击或拖拽文件（≤10MB）</div>
      </el-upload>

      <el-input
        v-model="text"
        type="textarea"
        :rows="4"
        placeholder="或者贴一段文字…"
        class="txt"
      />

      <el-button
        type="success"
        size="large"
        class="big-btn"
        :disabled="!text && !file"
        @click="toLock"
        >下一步</el-button
      >
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
        <template #subTitle>
          柜子编号：<b class="code">{{ cabinet.id }}</b
          ><br />
          取件密码：●●●●<br />
          到期时间：{{ expire }}
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
import { getCabinetsUsage, applyCabinet, saveCabinet } from '@/api/cabinet';

const step = ref(0);
const stats = ref({ total: 0, used: 0, free: 0 });
const text = ref('');
const file = ref([]);
const pwd = ref('');
const hours = ref(1);
const cabinet = ref({});
const expire = ref('');

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

function beforeUpload(f) {
  if (f.size > 2 * 1024 * 1024) {
    ElMessage.warning('文件过大');
    return false;
  }
  file.value.push(f);
  return false;
}

function toLock() {
  step.value = 2;
}

async function lockCabinet() {
  const form = new FormData();
  form.set('hold_token', cabinet.value.hold_token);
  form.set('hours', hours.value);
  form.append('password', pwd.value);
  form.append('message', text.value);
  file.value.forEach((f) => form.append('files', f));
  await saveCabinet(cabinet.value.code, form);
  step.value = 3;
  fetchUsage();
}

function share() {
  const url = `${location.origin}/pick?c=${cabinet.value.id}`;
  const text = `柜子编号 ${cabinet.value.id}，链接 ${url}`;
  if (navigator.share) navigator.share({ title: '临时柜子', text });
  else navigator.clipboard.writeText(text);
  ElMessage.success('已复制');
}

function reset() {
  text.value = '';
  file.value = null;
  pwd.value = '';
  hours.value = 24;
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
.code {
  font-size: 32px;
  letter-spacing: 4px;
}
</style>
