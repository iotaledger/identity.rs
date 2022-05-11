const path = require('path');

module.exports = {
    title: 'Identity',
    url: '/',
    baseUrl: '/',
    themes: ['@docusaurus/theme-classic'],
    themeConfig: {
        navbar: {
            // Workaround to disable broken logo href on test build
            logo: {
                src: 'img/identity_logo.svg',
                href: 'https://wiki.iota.org/',
            },
        },
    },
    plugins: [
        [
            '@docusaurus/plugin-content-docs',
            {
                id: 'identity-rs',
                path: path.resolve(__dirname, 'docs'),
                routeBasePath: 'identity.rs',
                sidebarPath: path.resolve(__dirname, 'sidebars.js'),
                editUrl: 'https://github.com/iotaledger/identity/edit/support/v0.5',
                remarkPlugins: [require('remark-code-import'), require('remark-import-partial'), require('remark-remove-comments')],
            }
        ],
    ],
    staticDirectories: [path.resolve(__dirname, 'static')],
};
