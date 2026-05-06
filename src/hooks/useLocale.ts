import { useState, useCallback } from 'react';
import { Lang } from '../types';
import zh_cn from '../i18n/zh_cn.json';
import en_us from '../i18n/en_us.json';

const locales: Record<Lang, Record<string, string>> = {
  zh_cn: zh_cn as Record<string, string>,
  en_us: en_us as Record<string, string>,
};

export function useLocale(initial: Lang = 'zh_cn') {
  const [lang, setLang] = useState<Lang>(initial);
  const dict = locales[lang];

  const t = useCallback(
    (key: string) => dict[key] ?? key,
    [dict]
  );

  const changeLang = useCallback((l: Lang) => {
    setLang(l);
  }, []);

  return { t, lang, changeLang, dict };
}
