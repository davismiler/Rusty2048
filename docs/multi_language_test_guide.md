# 多语言功能测试指南

本文档提供了测试Rusty2048多语言功能的详细步骤。

## 功能概述

Rusty2048现在支持英语和中文两种语言，在所有三个版本（CLI、Web、Desktop）中都可以动态切换语言。

## 测试步骤

### 1. CLI版本测试

#### 基本功能测试
1. 启动CLI版本：
   ```bash
   cd cli && cargo run
   ```

2. 语言切换测试：
   - 按 `L` 键切换语言
   - 验证以下元素是否正确翻译：
     - 状态栏文本（分数、最佳、移动次数）
     - 控制说明
     - 游戏结束消息
     - 胜利消息

3. 语言持久化测试：
   - 切换语言后退出游戏
   - 重新启动游戏，验证语言设置是否保持

#### 预期行为
- 语言切换应该立即生效
- 所有UI文本都应该正确翻译
- 语言设置应该在重启后保持

### 2. Web版本测试

#### 基本功能测试
1. 启动Web版本：
   ```bash
   cd web && ./build.sh
   cd dist && python3 -m http.server 8000
   ```
   然后在浏览器中打开 http://localhost:8000

2. 语言切换测试：
   - 点击 "Language" 按钮
   - 验证以下元素是否正确翻译：
     - 统计标签（Score、Best、Moves）
     - 按钮文本（New Game、Undo）
     - 说明文本

3. 浏览器语言检测测试：
   - 更改浏览器语言设置
   - 刷新页面，验证是否自动检测到正确的语言

#### 预期行为
- 语言按钮应该显示当前语言名称
- 所有文本都应该正确翻译
- 浏览器语言检测应该工作正常

### 3. Desktop版本测试

#### 基本功能测试
1. 启动Desktop版本：
   ```bash
   cd desktop && cargo tauri dev
   ```

2. 语言切换测试：
   - 点击 "Language" 按钮
   - 验证以下元素是否正确翻译：
     - 统计标签（Score、Best、Moves）
     - 按钮文本（New Game、Undo）
     - 说明文本

#### 预期行为
- 语言切换应该立即生效
- 所有UI文本都应该正确翻译
- 应用重启后语言设置应该保持

## 翻译内容验证

### 英语内容
- Score: "Score"
- Best: "Best"
- Moves: "Moves"
- New Game: "New Game"
- Undo: "Undo"
- Instructions: "Use arrow keys to move tiles. Combine tiles to reach 2048!"

### 中文内容
- Score: "分数"
- Best: "最佳"
- Moves: "移动"
- New Game: "新游戏"
- Undo: "撤销"
- Instructions: "使用方向键移动瓦片。合并瓦片以达到2048！"

## 常见问题排查

### 1. 语言切换不生效
- 检查是否正确按下了语言切换键/按钮
- 验证翻译文件是否正确加载
- 检查控制台是否有错误信息

### 2. 部分文本未翻译
- 确认所有翻译键都已正确映射
- 检查是否有遗漏的翻译项
- 验证翻译函数的调用是否正确

### 3. 语言设置不持久化
- 检查语言偏好文件是否正确保存
- 验证文件权限是否正确
- 确认文件路径是否正确

## 技术实现细节

### CLI版本
- 使用 `LanguageManager` 管理语言设置
- 语言偏好保存在 `cli/language_preference.txt`
- 支持 `L` 键快速切换

### Web版本
- 使用 `I18n` 类管理翻译
- 自动检测浏览器语言
- 支持点击按钮切换语言

### Desktop版本
- 通过Tauri命令与后端通信
- 使用 `I18n` 类管理翻译
- 支持点击按钮切换语言

## 文件位置

- 翻译定义：`shared/src/i18n.rs`
- CLI语言管理：`cli/src/language.rs`
- Web翻译：`web/src/lib.rs`
- Desktop翻译：`desktop/src/main.rs`
