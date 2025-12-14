import { useState } from 'react';
import { createPortal } from 'react-dom';
import { useThemeStore, themes, type ThemeName } from '../stores/themeStore';
import styles from './ThemeSwitcher.module.css';

export default function ThemeSwitcher() {
  const { currentTheme, currentMode, setTheme, setMode } = useThemeStore();
  const [isOpen, setIsOpen] = useState(false);

  const handleThemeSelect = (theme: ThemeName) => {
    setTheme(theme);
    setIsOpen(false);
  };

  return (
    <div className={styles.container}>
      {/* Theme Toggle Button */}
      <button
        className={styles.toggleButton}
        onClick={() => setIsOpen(!isOpen)}
        aria-label="Change theme"
      >
        <span className={styles.currentThemeEmoji}>
          {themes[currentTheme].emoji}
        </span>
        <span className={styles.buttonText}>Theme</span>
      </button>

      {/* Theme Picker Modal */}
      {isOpen && createPortal(
        <>
          <div className={styles.overlay} onClick={() => setIsOpen(false)} />
          <div className={styles.modal}>
            <div className={styles.modalContent}>
              <div className={styles.modalHeader}>
                <h3 className={styles.modalTitle}>Choose Theme</h3>
                <button
                  className={styles.closeButton}
                  onClick={() => setIsOpen(false)}
                  aria-label="Close"
                >
                  ×
                </button>
              </div>

              {/* Mode Toggle */}
              <div className={styles.modeToggle}>
                <button
                  className={`${styles.modeButton} ${
                    currentMode === 'light' ? styles.modeButtonActive : ''
                  }`}
                  onClick={() => setMode('light')}
                >
                  ☀️ Light
                </button>
                <button
                  className={`${styles.modeButton} ${
                    currentMode === 'dark' ? styles.modeButtonActive : ''
                  }`}
                  onClick={() => setMode('dark')}
                >
                  🌙 Dark
                </button>
              </div>

              {/* Theme Grid */}
              <div className={styles.themeGrid}>
                {(Object.keys(themes) as ThemeName[]).map((themeName) => {
                  const theme = themes[themeName];
                  const colors = theme[currentMode];
                  const isActive = currentTheme === themeName;

                  return (
                    <button
                      key={themeName}
                      className={`${styles.themeCard} ${
                        isActive ? styles.themeCardActive : ''
                      }`}
                      onClick={() => handleThemeSelect(themeName)}
                    >
                      <div className={styles.themePreview}>
                        <div
                          className={styles.previewBar}
                          style={{ backgroundColor: colors.primary }}
                        />
                        <div
                          className={styles.previewBar}
                          style={{ backgroundColor: colors.secondary }}
                        />
                        <div
                          className={styles.previewBar}
                          style={{ backgroundColor: colors.accent }}
                        />
                      </div>
                      <div className={styles.themeInfo}>
                        <span className={styles.themeEmoji}>{theme.emoji}</span>
                        <span className={styles.themeName}>{theme.displayName}</span>
                      </div>
                      {isActive && (
                        <div className={styles.activeIndicator}>✓</div>
                      )}
                    </button>
                  );
                })}
              </div>
            </div>
          </div>
        </>,
        document.body
      )}
    </div>
  );
}
