# 角色与开发任务
你是一位精通 Tauri (v2)、Rust、FFmpeg 以及端侧 AI 推理（ONNX Runtime / C-API）的资深系统架构师与音视频工程师。

请帮我完整实现一个名为 **"NeuroClip"** 的桌面端 AI 自动化长视频高光切片与智能解说系统。

---

## 一、 项目背景与核心功能定位

### 1. 业务目标
用户拖入一段 10~60 分钟的长视频（如电竞比赛、播客访谈、体育集锦、演讲），软件在本地离线自动化完成以下操作：
1. **多模态高光挖掘**：
   - 音频维度：通过音频能量（RMS）飙升 + 情绪音效标签（欢呼、大笑、尖叫、掌声）定位情绪爆点。
   - 视觉维度：通过轻量级镜头转场检测（TransNetV2）吸附最近的转场切点，避免粗暴切断动作或说话。
   - 输出 1~3 个 15~45 秒的高价值短视频片段（包含精准毫秒级 `start_time` 与 `end_time`）。
2. **智能解说词生成**：为高光片段编写具备“前 3 秒黄金 Hook”（如“注意看”、“Watch closely”）的快节奏解说文案，字数严格按语音语速（中文约 4.2 字/秒）与片段时长精确对齐。
3. **离线语音合成 (TTS)**：调用本地超轻量 ONNX 语音引擎，将解说文本合成为旁白音频（WAV）。
4. **工业级混剪与压制**：
   - 无损裁剪高光视频。
   - **音频闪避（Audio Ducking）**：解说出声时原片音量自动平滑压低 12dB，解说结束平滑回升。
   - 自动生成对应时间轴的 SRT/ASS 字幕并烧录（Hardsub），输出最终短视频 MP4。

### 2. 严格的体积与性能红线（总额外模型+运行时体积 < 150MB）
- **绝不使用** 动辄数 GB 的重型本地 LLM（如 7B/14B）或庞大的 Python 环境。
- **客户端外壳**：Tauri (Rust 后端 + 前端 React/Vue + Tailwind CSS)，低内存常驻。
- **AI 推理引擎**：全栈使用 **ONNX Runtime (ort crate / C-API)** 在 CPU/系统核显上毫秒级运行。
- **模型矩阵清单**：
  - 镜头转场切分：`TransNetV2.onnx` (~15 MB)
  - 声音爆点事件：`YAMNet.onnx` (~14 MB) + RMS 短时能量算法 (0 MB)
  - 离线语音合成：`Sherpa-onnx` 或 `Piper-TTS` (VITS 中/英文模型, ~45 MB)
  - 视频裁切与音画合成：系统内置/精简版 `FFmpeg` 二进制 CLI

---

## 二、 架构工作流设计 (Pipeline)

[原始长视频]
│
├── Step 1: FFmpeg 抽离单声道 16kHz WAV + 降采样低帧率视觉流
│
├── Step 2: 双通道高光定位引擎 (Fusion Engine)
│     ├── 通道 A (音频): 短时 RMS 能量滑窗 + YAMNet 检测 (Cheering/Laughter/Applause)
│     └── 通道 B (视觉): TransNetV2 识别镜头 Cut 点 (防止断头断尾)
│     └── 结果: 锁定精确高光区间 [start_time, end_time]
│
├── Step 3: 解说文案与字幕生成 (Hook 文案对齐时长)
│
├── Step 4: Sherpa-onnx 本地 TTS 合成解说音频 (voiceover.wav)
│
└── Step 5: FFmpeg 最终合成 (切片 + sidechaincompress 音频闪避 + 字幕压制) ➔ 导出短视频


---

## 三、 请为我编写并提供以下核心代码与工程方案

请不要给出泛泛的概念说明，直接提供结构清晰、带详细中文注释的生产级代码：

### 1. 【Rust 后端】FFmpeg 子进程与命令管道封装
- 实现提取 16kHz WAV 音频的函数。
- 实现工业级短视频混流指令：
  - 包含 `sidechaincompress` 滤镜（原视频为背景音，解说音频为侧链触发源，当解说出声时自动下压原音 12dB，平滑恢复）。
  - 混音（`amix`）并使用 `subtitles` 滤镜烧录解说字幕，使用 `libx264` 和 `aac` 输出短视频。

### 2. 【Rust 后端】多模态高光决策器 (HighlightPipeline)
- **音频能量计算**：实现滑窗 RMS 计算，抓取瞬时分贝显著高于均值 2.5 倍的候选区间。
- **镜头吸附对齐算法**：接收 TransNetV2 输出的转场帧索引，将候选起止时间吸附至最近的无破损镜头边缘。

### 3. 【端侧 TTS 集成】Sherpa-onnx / ONNX 语音合成实现
- 在 Rust 中调用 Sherpa-onnx（或本地 ONNX VITS 模型），输入解说文案，输出匹配采样率的 `voiceover.wav`。

### 4. 【系统提示词 (Prompt)】内置短视频解说文案生成器
- 设计一段专门内置在程序中的 System Prompt，要求模型：
  - 必须生成前 3 秒悬念 Hook。
  - 字数必须严格匹配传入的秒数（字数 ≈ duration * 4.2）。
  - 强制仅输出下游程序能解析的标准化 JSON 数据结构。

### 5. 【Tauri Command】前后端交互接口
- 编写 Tauri Command（如 `start_auto_clip(video_path: String)`），包含状态推进事件推送到前端（抽取音频 ➔ 分析高光 ➔ 合成语音 ➔ 最终导出）。

请按照上述模块分步输出完整的工程实现代码与配置细节。