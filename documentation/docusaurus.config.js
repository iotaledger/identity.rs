const path = require('path');

module.exports = {
    plugins: [
        [
            '@docusaurus/plugin-content-docs',
            {
                id: 'identity-rs',
                path: path.resolve(__dirname, 'docs'),
                routeBasePath: 'identity.rs',
                sidebarPath: path.resolve(__dirname, 'sidebars.js'),
                editUrl: 'https://github.com/iotaledger/identity.rs/edit/support/v0.5/documentation',
                remarkPlugins: [require('remark-code-import'), require('remark-import-partial'), require('remark-remove-comments')],
            }
        ],
    ],
    staticDirectories: [path.resolve(__dirname, 'static')],
};
