export class EventManager {
    constructor(gameInstance, canvasManager, animationManager) {
        this.game = gameInstance;
        this.canvasManager = canvasManager;
        this.animationManager = animationManager;

        // Mouse drag related state
        this.isDragging = false;
        this.startX = null;
        this.startY = null;
        this.dragThreshold = 30;

        // Touch related state
        this.touchStartX = null;
        this.touchStartY = null;
    }

    setupEventListeners() {
        this.setupKeyboardControls();
        this.setupMouseControls();
        this.setupButtonControls();
        this.setupTouchControls();
        this.setupResizeHandler();
    }

    setupKeyboardControls() {
        document.addEventListener('keydown', async (e) => {
            let direction = null;
            switch (e.key) {
                case 'ArrowUp':
                case 'w':
                case 'W':
                    direction = 'up';
                    break;
                case 'ArrowDown':
                case 's':
                case 'S':
                    direction = 'down';
                    break;
                case 'ArrowLeft':
                case 'a':
                case 'A':
                    direction = 'left';
                    break;
                case 'ArrowRight':
                case 'd':
                case 'D':
                    direction = 'right';
                    break;
            }

            if (direction) {
                e.preventDefault();
                // Quick check game state without blocking
                this.game.get_state().then(state => {
                    if (state === 'playing') {
                        // Async move processing without blocking keyboard events
                        this.game.handleMove(direction);
                    }
                });
            }
        });
    }

    setupMouseControls() {
        const canvasElement = this.canvasManager.getCanvas();
        const dragHint = document.querySelector('.drag-hint');

        if (!canvasElement) return;

        canvasElement.addEventListener('mousedown', (e) => {
            // Use Promise to get state asynchronously without blocking
            this.game.get_state().then(state => {
                if (state !== 'playing') return;

                this.isDragging = true;
                this.startX = e.clientX;
                this.startY = e.clientY;

                canvasElement.style.cursor = 'grabbing';
                canvasElement.style.userSelect = 'none';

                if (dragHint) {
                    dragHint.style.display = 'block';
                    dragHint.style.opacity = '1';
                }
            });
        });

        document.addEventListener('mousemove', (e) => {
            if (!this.isDragging) return;

            const deltaX = e.clientX - this.startX;
            const deltaY = e.clientY - this.startY;
            const distance = Math.sqrt(deltaX * deltaX + deltaY * deltaY);

            if (distance > this.dragThreshold) {
                let direction = null;

                if (Math.abs(deltaX) > Math.abs(deltaY)) {
                    direction = deltaX > 0 ? 'right' : 'left';
                } else {
                    direction = deltaY > 0 ? 'down' : 'up';
                }

                if (direction) {
                    // Quick check game state without blocking
                    this.game.get_state().then(state => {
                        if (state === 'playing') {
                            // Async move processing without blocking mouse events
                            this.game.handleMove(direction);
                        }
                    });
                }

                this.resetDragState(canvasElement, dragHint);
            }
        });

        document.addEventListener('mouseup', () => {
            if (this.isDragging) {
                this.resetDragState(canvasElement, dragHint);
            }
        });

        document.addEventListener('mouseleave', () => {
            if (this.isDragging) {
                this.resetDragState(canvasElement, dragHint);
            }
        });

        canvasElement.style.cursor = 'grab';
    }

    resetDragState(canvasElement, dragHint) {
        this.isDragging = false;
        canvasElement.style.cursor = 'grab';
        canvasElement.style.userSelect = 'auto';

        if (dragHint) {
            this.animationManager.fadeOutElement(dragHint);
        }
    }

    setupButtonControls() {
        // 主界面按钮
        document.getElementById('newGame').addEventListener('click', async () => {
            await this.game.handleNewGame();
        });

        document.getElementById('undo').addEventListener('click', async () => {
            await this.game.undo();
            await this.game.updateDisplay();
        });



        // 侧边菜单按钮
        document.getElementById('newGameMenu').addEventListener('click', async () => {
            await this.game.handleNewGame();
            this.closeMenu();
        });

        document.getElementById('undoMenu').addEventListener('click', async () => {
            await this.game.undo();
            await this.game.updateDisplay();
            this.closeMenu();
        });

        document.getElementById('languageToggleMenu').addEventListener('click', async () => {
            await this.game.toggleLanguage();
            this.closeMenu();
        });

        // 主题按钮（包括主界面和侧边菜单）
        document.querySelectorAll('.theme-btn').forEach(btn => {
            btn.addEventListener('click', async () => {
                const themeName = btn.getAttribute('data-theme');
                await this.game.applyTheme(themeName);

                // 更新所有主题按钮的状态
                document.querySelectorAll('.theme-btn').forEach(b => {
                    b.classList.remove('active');
                });
                btn.classList.add('active');
            });
        });

        // 侧边菜单控制
        this.setupMenuControls();
    }

    setupMenuControls() {
        const menuToggle = document.getElementById('menuToggle');
        const menuClose = document.getElementById('menuClose');
        const sideMenu = document.getElementById('sideMenu');
        const menuOverlay = document.getElementById('menuOverlay');

        // 打开菜单
        menuToggle.addEventListener('click', () => {
            this.openMenu();
        });

        // 关闭菜单
        menuClose.addEventListener('click', () => {
            this.closeMenu();
        });

        // 点击遮罩关闭菜单
        menuOverlay.addEventListener('click', () => {
            this.closeMenu();
        });

        // ESC键关闭菜单
        document.addEventListener('keydown', (e) => {
            if (e.key === 'Escape' && sideMenu.classList.contains('open')) {
                this.closeMenu();
            }
        });
    }

    openMenu() {
        const sideMenu = document.getElementById('sideMenu');
        const menuOverlay = document.getElementById('menuOverlay');

        sideMenu.classList.add('open');
        menuOverlay.classList.add('active');
        document.body.style.overflow = 'hidden';
    }

    closeMenu() {
        const sideMenu = document.getElementById('sideMenu');
        const menuOverlay = document.getElementById('menuOverlay');

        sideMenu.classList.remove('open');
        menuOverlay.classList.remove('active');
        document.body.style.overflow = '';
    }

    setupTouchControls() {
        const canvasElement = this.canvasManager.getCanvas();

        if (!canvasElement) return;

        canvasElement.addEventListener('touchstart', (e) => {
            // 阻止默认行为，防止页面滚动
            e.preventDefault();
            this.touchStartX = e.touches[0].clientX;
            this.touchStartY = e.touches[0].clientY;
        }, { passive: false });

        canvasElement.addEventListener('touchend', (e) => {
            // 阻止默认行为
            e.preventDefault();

            if (!this.touchStartX || !this.touchStartY) return;

            const endX = e.changedTouches[0].clientX;
            const endY = e.changedTouches[0].clientY;

            const diffX = this.touchStartX - endX;
            const diffY = this.touchStartY - endY;

            // 添加最小滑动距离阈值
            const minSwipeDistance = 30;
            if (Math.abs(diffX) < minSwipeDistance && Math.abs(diffY) < minSwipeDistance) {
                this.touchStartX = this.touchStartY = null;
                return;
            }

            let direction = null;
            if (Math.abs(diffX) > Math.abs(diffY)) {
                // Horizontal swipe
                if (diffX > 0) {
                    direction = 'left';
                } else {
                    direction = 'right';
                }
            } else {
                // Vertical swipe
                if (diffY > 0) {
                    direction = 'up';
                } else {
                    direction = 'down';
                }
            }

            if (direction) {
                // Quick check game state without blocking
                this.game.get_state().then(state => {
                    if (state === 'playing') {
                        // Async move processing without blocking touch events
                        this.game.handleMove(direction);
                    }
                });
            }

            this.touchStartX = this.touchStartY = null;
        }, { passive: false });

        // 防止触摸事件冒泡到document
        canvasElement.addEventListener('touchmove', (e) => {
            e.preventDefault();
        }, { passive: false });
    }

    setupResizeHandler() {
        window.addEventListener('resize', () => {
            this.canvasManager.handleResize();
            if (this.game.previousBoard) {
                const tiles = this.game.uiManager.boardToTiles(this.game.previousBoard);
                this.canvasManager.drawBoardRaw(tiles);
            }
        });
    }
}
