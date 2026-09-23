import { defineConfig } from 'vitepress'

export default defineConfig({
  title: 'Cue',
  description: 'Video answers with timestamp-level proof',
  base: '/cue/',
  appearance: 'dark',
  head: [
    ['link', { rel: 'canonical', href: 'https://sylphxai.github.io/cue/' }],
    ['meta', { name: 'theme-color', content: '#120c0a' }],
    ['meta', { property: 'og:type', content: 'website' }],
    ['meta', { property: 'og:title', content: 'Cue — Video answers with timestamp-level proof' }],
    ['meta', { property: 'og:description', content: 'The default read returns container metadata, streams, chapters, and embedded subtitles. Scenes and frames are separate.' }],
    ['meta', { property: 'og:url', content: 'https://sylphxai.github.io/cue/' }],
    ['meta', { name: 'twitter:card', content: 'summary' }]
  ],
  cleanUrls: true,
  themeConfig: {
    nav: [
      { text: 'Quickstart', link: '/guide/quickstart' },
      { text: 'Tools', link: '/reference/tools' },
      { text: 'GitHub', link: 'https://github.com/SylphxAI/cue' },
      { text: 'npm', link: 'https://www.npmjs.com/package/@sylphx/cue' }
    ],
    sidebar: [
      { text: 'Product', items: [{ text: 'Vision', link: '/vision' }, { text: 'Capabilities', link: '/capabilities' }] },
      { text: 'Guide', items: [{ text: 'Quickstart', link: '/guide/quickstart' }, { text: 'Predictable defaults', link: '/reference/defaults' }] },
      { text: 'Reference', items: [{ text: 'Tools', link: '/reference/tools' }] }
    ],
    socialLinks: [{ icon: 'github', link: 'https://github.com/SylphxAI/cue' }],
    footer: { message: 'Cue · local-first agent tooling · MIT' }
  }
})
