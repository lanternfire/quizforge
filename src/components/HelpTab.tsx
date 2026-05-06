import { useState } from 'react';
import zhDict from '../i18n/zh_cn.json';
import enDict from '../i18n/en_us.json';

interface HelpTabProps {
  t: (key: string) => string;
  lang: string;
}

const zhPromptText = (zhDict as Record<string, string>)['help.ai_prompt_zh'] || '';
const enPromptText = (enDict as Record<string, string>)['help.ai_prompt_en'] || '';

export default function HelpTab({ t, lang }: HelpTabProps) {
  const [copied, setCopied] = useState<'zh' | 'en' | null>(null);

  const handleCopy = async (text: string, type: 'zh' | 'en') => {
    try {
      await navigator.clipboard.writeText(text);
      setCopied(type);
      setTimeout(() => setCopied(null), 2000);
    } catch {}
  };

  const steps = [
    { num: 1, title: t('help.step1'), desc: t('help.step1.desc') },
    { num: 2, title: t('help.step2'), desc: t('help.step2.desc') },
    { num: 3, title: t('help.step3'), desc: t('help.step3.desc') },
    { num: 4, title: t('help.step4'), desc: t('help.step4.desc') },
  ];

  return (
    <div className="tab-content">
      <div className="tab-content-inner">
        <div className="help-tab">
          <h2>{t('help.title')}</h2>
          <p className="subtitle">{t('help.workflow.desc')}</p>

          <div className="help-steps">
            {steps.map(s => (
              <div key={s.num} className="help-step">
                <div className="step-num">{s.num}</div>
                <div className="step-text">
                  <h4>{s.title}</h4>
                  <p>{s.desc}</p>
                </div>
              </div>
            ))}
          </div>

          <div className="help-prompts">
            <h3>{t('help.ai_prompts')}</h3>
            <p className="prompt-header-desc">{t('help.ai_prompt_header')}</p>

            <div className="prompt-card">
              <div className="prompt-card-header">
                <span>{t('help.prompt_zh_label')}</span>
                <button
                  className="btn btn-ghost"
                  onClick={() => handleCopy(zhPromptText, 'zh')}
                >
                  {copied === 'zh' ? t('help.copied') : t('help.copy')}
                </button>
              </div>
              <textarea
                className="prompt-textarea"
                readOnly
                value={zhPromptText}
                onClick={(e) => (e.target as HTMLTextAreaElement).select()}
              />
            </div>

            <div className="prompt-card">
              <div className="prompt-card-header">
                <span>{t('help.prompt_en_label')}</span>
                <button
                  className="btn btn-ghost"
                  onClick={() => handleCopy(enPromptText, 'en')}
                >
                  {copied === 'en' ? t('help.copied') : t('help.copy')}
                </button>
              </div>
              <textarea
                className="prompt-textarea"
                readOnly
                value={enPromptText}
                onClick={(e) => (e.target as HTMLTextAreaElement).select()}
              />
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
