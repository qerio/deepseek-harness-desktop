# DeepSeek Harness Desktop

DeepSeek Harness 的 Tauri 2 多端外壳。上游源码通过 `vendor/deepseek-harness` Git submodule 完整嵌入，不修改上游文件。

## 环境

- Node.js 22.19+ 或 24+
- pnpm 11.7+
- Rust stable
- macOS/iOS：Xcode；Android：Android Studio、SDK、NDK、JDK

## 初始化

```bash
git submodule update --init --recursive
npm install
npm run harness:install
```

## 桌面端

```bash
npm run dev
npm run build
```

桌面开发模式启动子模块中的 `deepseek-harness` Web 服务并打开 Tauri WebView，退出应用时会同步清理后端进程。当前工程保留上游完整源码，并要求运行目录能找到子模块且目标机器能执行 `node`；若要制作完全独立的安装包，后续应按目标系统加入 Node sidecar 与生产依赖资源（每个平台分别构建），外层 Rust 启动逻辑无需改动上游。

## Android / iOS

移动系统不能运行 deepseek-harness 所依赖的 Node、shell 和本机进程能力，因此移动端作为同一 Web UI 的远程客户端。构建时必须配置可访问的 HTTPS 服务地址：

```bash
DSH_REMOTE_URL=https://harness.example.com npm run android:init
DSH_REMOTE_URL=https://harness.example.com npm run android:dev

DSH_REMOTE_URL=https://harness.example.com npm run ios:init
DSH_REMOTE_URL=https://harness.example.com npm run ios:dev
```

远端服务应通过认证和 TLS 保护。不要把 deepseek-harness 的回环开发服务直接暴露到公网。

## 上游更新

```bash
git -C vendor/deepseek-harness fetch origin
git -C vendor/deepseek-harness checkout <commit-or-tag>
git add vendor/deepseek-harness
```
