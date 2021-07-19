const lightCodeTheme = require('prism-react-renderer/themes/github');
const darkCodeTheme = require('prism-react-renderer/themes/dracula');

/** @type {import('@docusaurus/types').DocusaurusConfig} */
module.exports = {
  title: 'IOTA IDENTITY',
  tagline: 'Providing Trust between Individuals, Organizations and Things.',
  url: 'https://your-docusaurus-test-site.com',
  baseUrl: '/identity.rs/',
  onBrokenLinks: 'throw',
  onBrokenMarkdownLinks: 'warn',
  favicon: 'img/favicon.ico',
  organizationName: 'iotaledger', // Usually your GitHub org/user name.
  projectName: 'identity.rs', // Usually your repo name.
  stylesheets: [
    'https://fonts.googleapis.com/css?family=Material+Icons',
  ],
  themeConfig: {
    navbar: {
      title: 'Identity',
      logo: {
        alt: 'IOTA Identity Logo',
        src: 'img/iota_logo.svg',
      },
      items: [
        {
          type: 'doc',
          docId: 'intro',
          position: 'left',
          label: 'Documentation',
        },
        {to: '/blog', label: 'Blog/Tutorial', position: 'left'},
        {to: '/team', label: 'Team', position: 'left'},
        {
          href: 'https://github.com/iotaledger/identity.rs',
          label: 'GitHub',
          position: 'right',
        },
      ],
    },
    footer: {
      style: 'dark',
      links: [
        {
          title: 'Identity',
          items: [
            {
              label: 'Documentation',
              to: '/docs/intro',
            },
            {
              label: 'Blog/Tutorial',
              to: 'blog',
            },
            {
              label: 'Team',
              to: 'team',
            },
          ],
        },
        {
          title: 'Community',
          items: [
            {
              label: 'Discord',
              href: 'https://discord.iota.org/',
            },
            {
              label: 'Stack Exchange',
              href: 'https://iota.stackexchange.com/',
            },
            {
              label: 'Twitter',
              href: 'https://twitter.com/iota',
            },
            {
              label: 'Reddit',
              href: 'https://www.reddit.com/r/Iota/',
            },
          ],
        },
        {
          title: 'More',
          items: [
            {
              label: 'IOTA',
              href: 'https://www.iota.org/',
            },
            {
              label: 'GitHub',
              href: 'https://github.com/iotaledger/identity.rs',
            },
          ],
        },
      ],
      copyright: `Copyright © ${new Date().getFullYear()} My Project, Inc. Built with Docusaurus.`,
    },
    prism: {
      theme: lightCodeTheme,
      darkTheme: darkCodeTheme,
    },
  },
  presets: [
    [
      '@docusaurus/preset-classic',
      
      {
        docs: {
          remarkPlugins: [require('remark-import-partial')],
          sidebarPath: require.resolve('./sidebars.js'),
          // Please change this to your repo.
          editUrl:
            'https://github.com/facebook/docusaurus/edit/master/website/',
        },
        blog: {
          showReadingTime: true,
          // Please change this to your repo.
          editUrl:
            'https://github.com/facebook/docusaurus/edit/master/website/blog/',
        },
        theme: {
          customCss: require.resolve('./src/css/custom.css'),
        },
      },
    ],
  ],
};
