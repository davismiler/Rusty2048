// 游戏存储管理器
export class GameStorage {
    constructor() {
        this.STORAGE_KEY = 'rusty2048_game_state';
        this.SETTINGS_KEY = 'rusty2048_settings';
    }

    // 保存游戏状态
    saveGameState(gameState) {
        try {
            this.updateSaveStatus('saving');
            
            // 确保board是普通数组
            const boardArray = Array.isArray(gameState.board) ? gameState.board : Array.from(gameState.board);
            
            const stateToSave = {
                board: boardArray,
                score: gameState.score,
                moves: gameState.moves,
                state: gameState.state,
                timestamp: Date.now(),
                version: '1.0.0' // 用于版本兼容性
            };
            
            localStorage.setItem(this.STORAGE_KEY, JSON.stringify(stateToSave));
            
            this.updateSaveStatus('saved');
            return true;
        } catch (error) {
            console.error('保存游戏状态失败:', error);
            this.updateSaveStatus('error');
            return false;
        }
    }

    // 加载游戏状态
    loadGameState() {
        try {
            const savedState = localStorage.getItem(this.STORAGE_KEY);
            if (!savedState) {
                return null;
            }

            const parsedState = JSON.parse(savedState);
            
            // 检查版本兼容性
            if (!parsedState.version || parsedState.version !== '1.0.0') {
                this.clearGameState();
                return null;
            }

            // 检查数据完整性
            if (!parsedState.board || !parsedState.score || 
                typeof parsedState.moves !== 'number' || !parsedState.state) {
                this.clearGameState();
                return null;
            }

            // 确保board是数组格式
            if (!Array.isArray(parsedState.board)) {
                this.clearGameState();
                return null;
            }

            return parsedState;
        } catch (error) {
            console.error('加载游戏状态失败:', error);
            this.clearGameState();
            return null;
        }
    }

    // 清除游戏状态
    clearGameState() {
        try {
            localStorage.removeItem(this.STORAGE_KEY);
        } catch (error) {
            console.error('清除游戏状态失败:', error);
        }
    }

    // 保存设置
    saveSettings(settings) {
        try {
            const settingsToSave = {
                language: settings.language,
                theme: settings.theme,
                timestamp: Date.now(),
                version: '1.0.0'
            };
            
            localStorage.setItem(this.SETTINGS_KEY, JSON.stringify(settingsToSave));
            return true;
        } catch (error) {
            console.error('保存设置失败:', error);
            return false;
        }
    }

    // 加载设置
    loadSettings() {
        try {
            const savedSettings = localStorage.getItem(this.SETTINGS_KEY);
            if (!savedSettings) {
                return null;
            }

            const parsedSettings = JSON.parse(savedSettings);
            
            // 检查版本兼容性
            if (!parsedSettings.version || parsedSettings.version !== '1.0.0') {
                this.clearSettings();
                return null;
            }

            return parsedSettings;
        } catch (error) {
            console.error('加载设置失败:', error);
            this.clearSettings();
            return null;
        }
    }

    // 清除设置
    clearSettings() {
        try {
            localStorage.removeItem(this.SETTINGS_KEY);
        } catch (error) {
            console.error('清除设置失败:', error);
        }
    }

    // 检查是否有保存的游戏
    hasSavedGame() {
        return localStorage.getItem(this.STORAGE_KEY) !== null;
    }

    // 获取保存的游戏信息
    getSavedGameInfo() {
        try {
            const savedState = localStorage.getItem(this.STORAGE_KEY);
            if (!savedState) {
                return null;
            }

            const parsedState = JSON.parse(savedState);
            return {
                score: parsedState.score?.current || 0,
                bestScore: parsedState.score?.best || 0,
                moves: parsedState.moves || 0,
                state: parsedState.state || 'playing',
                timestamp: parsedState.timestamp || 0,
                savedDate: new Date(parsedState.timestamp || 0).toLocaleString()
            };
        } catch (error) {
            console.error('获取保存的游戏信息失败:', error);
            return null;
        }
    }

    // 自动保存（用于定期保存）
    autoSave(gameState) {
        // 只在游戏进行中时保存
        if (gameState.state === 'playing' || gameState.state === 'won') {
            this.saveGameState(gameState);
        }
    }

    // 更新保存状态指示器
    updateSaveStatus(status) {
        const saveStatus = document.getElementById('saveStatus');
        const saveText = document.getElementById('saveText');
        
        if (!saveStatus || !saveText) return;
        
        // 移除所有状态类
        saveStatus.classList.remove('saving', 'saved', 'error');
        
        switch (status) {
            case 'saving':
                saveStatus.classList.add('saving');
                saveText.textContent = '正在保存...';
                break;
            case 'saved':
                saveStatus.classList.add('saved');
                saveText.textContent = '游戏进度已自动保存';
                // 3秒后隐藏保存状态
                setTimeout(() => {
                    if (saveStatus.classList.contains('saved')) {
                        saveStatus.style.opacity = '0.3';
                    }
                }, 3000);
                break;
            case 'error':
                saveStatus.classList.add('error');
                saveText.textContent = '保存失败，请检查存储空间';
                // 5秒后恢复默认状态
                setTimeout(() => {
                    if (saveStatus.classList.contains('error')) {
                        saveStatus.classList.remove('error');
                        saveStatus.style.opacity = '0.8';
                        saveText.textContent = '游戏进度已自动保存';
                    }
                }, 5000);
                break;
        }
    }
}
