import { defineConfig } from 'vitepress'

export default defineConfig({
  title: 'Cue',
  description: 'Video answers with timestamp-level proof',
  head: [
    ['link', { rel: 'canonical', href: 'https://sylphxai.github.io/cue/' }],
    ['meta', { property: 'og:type', content: 'website' }],
    ['meta', { property: 'og:title', content: 'Cue — Video answers with timestamp-level proof' }],
    ['meta', { property: 'og:description', content: 'Search subtitles and transcripts, inspect timelines, and recover exact frames.' }],
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
      { text: 'Guide', items: [{ text: 'Quickstart', link: '/guide/quickstart' }, { text: 'Predictable defaults', link: '/reference/defaults' }] },
      { text: 'Reference', items: [{ text: 'Tools', link: '/reference/tools' }] }
    ],
    socialLinks: [{ icon: 'github', link: 'https://github.com/SylphxAI/cue' }],
    footer: { message: 'Cue · local-first agent tooling · MIT' }
  }
})
