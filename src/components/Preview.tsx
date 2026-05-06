import { useEffect, useRef, useState } from 'react';

interface PreviewProps {
  htmlContent: string;
  themeName: string;
  t: (key: string) => string;
  themeMode?: 'light' | 'dark';
}

export default function Preview({ htmlContent, themeName, t, themeMode = 'light' }: PreviewProps) {
  const iframeRef = useRef<HTMLIFrameElement>(null);

  useEffect(() => {
    if (iframeRef.current && htmlContent) {
      const doc = iframeRef.current.contentDocument;
      if (doc) {
        doc.open();
        doc.write(htmlContent);
        doc.close();

        // Inject styles: hide scrollbars + dark mode overlay
        const style = doc.createElement('style');
        let css = `
          html, body {
            overflow: hidden !important;
            scrollbar-width: none !important;
            -ms-overflow-style: none !important;
          }
          html::-webkit-scrollbar, body::-webkit-scrollbar {
            display: none !important;
            width: 0 !important;
            height: 0 !important;
          }
        `;
        if (themeMode === 'dark') {
          css += `
            body::after {
              content: '';
              position: fixed;
              inset: 0;
              pointer-events: none;
              background: rgba(0,0,0,0.35);
              z-index: 9999;
            }
            .quiz-container, .quiz-card, .quiz-header, .quiz-footer {
              filter: brightness(0.85) saturate(0.9);
            }
          `;
        }
        style.textContent = css;
        doc.head.appendChild(style);
      }
    }
  }, [htmlContent, themeMode]);

  if (!htmlContent) return null;

  return (
    <div className="preview-panel">
      <div className="preview-header">
        <span>{t('preview.title')}</span>
        <span className="preview-theme-name">{themeName}</span>
      </div>
      <div className="preview-frame">
        <iframe
          ref={iframeRef}
          title="Preview"
          scrolling="no"
          style={{ 
            width: '100%',
            height: '100%',
            border: 'none',
            borderRadius: '8px',
            display: 'block',
            overflow: 'hidden',
          }}
        />
      </div>
    </div>
  );
}
