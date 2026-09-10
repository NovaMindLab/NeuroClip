# NeuroClip - AI 桌面端长视频高光切片与智能解说系统

<div align="center">
  <img src="./public/neuroclip_logo.jpg" alt="NeuroClip Logo" width="220" style="border-radius: 28px; box-shadow: 0 10px 30px rgba(0, 240, 255, 0.25);" />
  <h3>桌面端 AI 自动化长视频高光切片、旁白合成与硬字幕工业级混剪系统</h3>
  <p><strong>Tauri v2 • Rust • FFmpeg • ONNX Runtime • React • Tailwind CSS</strong></p>
</div>

---

## 一、 项目背景与核心功能定位

用户拖入一段 10~60 分钟的长视频（如电竞比赛、播客访谈、体育集锦、演讲），软件在本地离线自动化完成以下操作：

1. **多模态高光挖掘**：
   - **音频维度**：通过音频能量（RMS）滑窗飙升（> 2.5x 基线） + 情绪音效标签（欢呼、大笑、尖叫、掌声）定位情绪爆点。
   - **视觉维度**：通过轻量级镜头转场检测（TransNetV2）吸附最近的转场切点，避免粗暴切断动作或说话。
   - **高光输出**：输出 1~3 个 15~45 秒的高价值短视频片段（包含精准毫秒级 `start_time` 与 `end_time`）。
2. **智能解说词生成**：
   - 为高光片段编写具备“**前 3 秒黄金 Hook**”（如“注意看”、“谁能想到”）的快节奏解说文案。
   - 字数严格按语音语速（中文约 **4.2 字/秒**）与片段时长精确对齐（误差控制在 ±3 字内）。
3. **离线语音合成 (TTS)**：
   - 调用本地超轻量 ONNX 语音引擎（Sherpa-onnx / Piper VITS），将解说文本合成为 16kHz WAV 旁白音频。
4. **工业级混剪与压制**：
   - 无损精准裁剪高光视频。
   - **音频闪避（Audio Ducking）**：使用 `sidechaincompress` 滤镜，解说出声时原片音量自动平滑压低 12dB，解说结束平滑回升。
   - 自动生成对应时间轴的 SRT 字幕并使用 `subtitles` 滤镜烧录（Hardsub），输出最终短视频 MP4。

---

## 二、 严格的体积与性能红线（总额外模型+运行时体积 < 150MB）

- **绝不使用** 动辄数 GB 的重型本地 LLM（如 7B/14B）或庞大的 Python 环境。
- **客户端外壳**：Tauri v2 (Rust 后端 + 前端 React + Tailwind CSS)，超低内存常驻。
- **AI 推理矩阵**：
  - 镜头转场切分：`TransNetV2.onnx` (~15 MB)
  - 声音爆点事件：`YAMNet.onnx` (~14 MB) + RMS 短时能量算法 (0 MB)
  - 离线语音合成：`Sherpa-onnx` / `Piper-TTS` (VITS 中/英文模型, ~45 MB)
  - 视频裁切与音画合成：系统内置/精简版 `FFmpeg` CLI 管道

---

## 三、 五阶段架构工作流 (Pipeline)

```
[原始长视频 (10~60min)]
       │
       ├── Step 1: FFmpeg 抽离单声道 16kHz WAV + 降采样低帧率视觉流
       │
       ├── Step 2: 双通道高光定位引擎 (Fusion Engine)
       │     ├── 通道 A (音频): 短时 RMS 能量滑窗 + YAMNet 检测 (Cheering/Laughter/Applause/Scream)
       │     └── 通道 B (视觉): TransNetV2 识别镜头 Cut 点 (镜头吸附对齐算法, 约束 15s~45s)
       │     └── 结果: 锁定精准高光区间 [start_time, end_time]
       │
       ├── Step 3: 解说文案与字幕生成 (前3秒黄金Hook, 严格按 4.2字/秒 对齐)
       │
       ├── Step 4: Sherpa-onnx 本地 TTS 合成解说音频 (voiceover.wav)
       │
       └── Step 5: FFmpeg 最终合成 (切片 + sidechaincompress 音频闪避 -12dB + 字幕压制) ➔ 导出短视频 MP4
```

---

## 四、 核心工程代码模块

### 1. Rust 后端架构 (`src-tauri/`)

- [src/ffmpeg/extractor.rs](file:///Users/hou/hqx/hubproject/NeuroClip/src-tauri/src/ffmpeg/extractor.rs): 抽离 16kHz 单声道 WAV 与视觉流。
- [src/ffmpeg/pipeline.rs](file:///Users/hou/hqx/hubproject/NeuroClip/src-tauri/src/ffmpeg/pipeline.rs): 工业级混剪混流指令，包含 `sidechaincompress=threshold=0.125:ratio=4:attack=20:release=250` 与 `subtitles` 硬字幕烧录。
- [src/highlight/rms.rs](file:///Users/hou/hqx/hubproject/NeuroClip/src-tauri/src/highlight/rms.rs): 滑窗 RMS 均方根计算，自动捕获高于均值 2.5 倍爆点区间。
- [src/highlight/sound_events.rs](file:///Users/hou/hqx/hubproject/NeuroClip/src-tauri/src/highlight/sound_events.rs): YAMNet 情绪识别标签（欢呼、大笑、尖叫、掌声）。
- [src/highlight/transnet.rs](file:///Users/hou/hqx/hubproject/NeuroClip/src-tauri/src/highlight/transnet.rs): TransNetV2 镜头吸附算法，精准贴合切点防止粗暴截断。
- [src/highlight/fusion.rs](file:///Users/hou/hqx/hubproject/NeuroClip/src-tauri/src/highlight/fusion.rs): 双通道多模态高光决策引擎，输出 Top 1~3 黄金片段。
- [src/tts/mod.rs](file:///Users/hou/hqx/hubproject/NeuroClip/src-tauri/src/tts/mod.rs): Sherpa-onnx / 本地 ONNX 语音合成与 16kHz WAV 输出。
- [src/script/prompt.rs](file:///Users/hou/hqx/hubproject/NeuroClip/src-tauri/src/script/prompt.rs): 内置短视频解说 Prompt、黄金 Hook 与 4.2 字/秒配速生成器。
- [src/commands/clip.rs](file:///Users/hou/hqx/hubproject/NeuroClip/src-tauri/src/commands/clip.rs): Tauri v2 前后端状态流式推送接口 (`start_auto_clip`)。

### 2. 前端桌面 UI (`src/`)

- [src/components/NeuroClipLogo.tsx](file:///Users/hou/hqx/hubproject/NeuroClip/src/components/NeuroClipLogo.tsx): 专属神经突触与胶片剪辑流光 Logo。
- [src/components/TimelineWaveform.tsx](file:///Users/hou/hqx/hubproject/NeuroClip/src/components/TimelineWaveform.tsx): 声学波形图、RMS 突变、YAMNet 情绪标签与转场切点渲染。
- [src/components/HighlightList.tsx](file:///Users/hou/hqx/hubproject/NeuroClip/src/components/HighlightList.tsx): 候选高光列表与转场吸附指示器。
- [src/components/CommentaryStudio.tsx](file:///Users/hou/hqx/hubproject/NeuroClip/src/components/CommentaryStudio.tsx): 黄金 Hook 卡片、4.2 字/秒配速仪表盘与字幕时间轴。
- [src/components/ClipPlayerModal.tsx](file:///Users/hou/hqx/hubproject/NeuroClip/src/components/ClipPlayerModal.tsx): -12dB 侧链音频闪避与烧录硬字幕成片预览。

---

## 五、 测试与构建

### 1. 运行 Rust 核心单元测试
```bash
cd src-tauri
cargo test
```
*已包含 6 项完整单元测试（TransNet 吸附、FFmpeg 滤镜指令拼接、4.2 字/秒 Hook 生成、RMS 能量爆点捕获、TTS 合成与多模态融合流动）。*

### 2. 构建前端
```bash
npm run build
```

### 3. 启动桌面端应用
```bash
npm run tauri dev
```

---

## 六、 GitHub 云端执行与版本发布

无需在本地配置繁重的音视频环境，NeuroClip 支持在 **GitHub 端完成全部执行与发布**：

### 1. 🎬 在 GitHub 端直接执行高光切片流水线 (无需本地 GPU/FFmpeg)
1. 访问 GitHub 仓库 Actions：[Run NeuroClip Pipeline](https://github.com/NovaMindLab/NeuroClip/actions/workflows/run-pipeline.yml)
2. 点击右上角 **"Run workflow"**：
   - 可输入目标视频的网络下载 URL（或留空，GitHub 将自动生成 60s 测试视频）。
   - 输入解说词主题提示（例如：`电竞高能决胜名场面`）。
   - 选择切片数量（1~3 个）。
3. 运行完毕后，GitHub Actions 将直接在 **Job Summary** 中展示分析评分表与前 3 秒 Hook，并在下方提供打包的 `neuroclip-highlight-clips` 产物供一键下载（包含高光短视频 MP4、旁白 WAV、硬字幕 SRT 与 JSON）。

### 2. 📦 在 GitHub 端一键打包并发布 Release
1. 访问 GitHub 仓库 Actions：[Release](https://github.com/NovaMindLab/NeuroClip/actions/workflows/release.yml)
2. 点击右上角 **"Run workflow"**：
   - 输入版本号（如 `v0.1.1`）。
   - 点击确认执行。
3. GitHub Actions 将自动在云端为 **macOS (Apple Silicon / Intel)、Windows、Ubuntu Linux** 全平台编译打包，自动打上 Git Tag 并直接发布到 GitHub Releases！

