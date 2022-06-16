const path = require('path');

module.exports = {
    plugins: [
        [
            '@docusaurus/plugin-content-docs',
            {
                id: 'identity-rs-0-6',
                path: path.resolve(__dirname, 'docs'),
                routeBasePath: 'identity.rs',
                sidebarPath: path.resolve(__dirname, 'sidebars.js'),
                editUrl: 'https://github.com/iotaledger/identity.rs/edit/support/v0.6/documentation',
                remarkPlugins: [require('remark-code-import'), require('remark-import-partial'), require('remark-remove-comments')],
                versions: {
                    current: {
                        label: '0.6',
                        badge: true
                    },
                },
            }
        ],
    ],
    staticDirectories: [path.resolve(__dirname, 'static')],
};
