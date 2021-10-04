/**
 * Creating a sidebar enables you to:
 - create an ordered group of docs
 - render a sidebar for each doc of that group
 - provide next/previous navigation

 The sidebars can be generated from the filesystem, or explicitly defined here.

 Create as many sidebars as you want.
 */

module.exports = {
    // By default, Docusaurus generates a sidebar from the docs folder structure
    // tutorialSidebar: [{type: 'autogenerated', dirName: '.'}],

    // But you can create a sidebar manually

    docs: [
        {
            type: 'doc',
            id: 'introduction',
            label: 'Introduction'
        },
        {
            type: 'doc',
            id: 'decentralized_identity',
            label: 'Decentralized Identity'
        },
        {
            type: 'category',
            label: 'Getting Started',
            collapsed: false,
            items: [
                'getting_started/overview',
                'getting_started/install',
                'getting_started/create_and_publish',
            ],
        },
        {
            'Decentralized Identifiers (DID)': [
                'decentralized_identifiers/overview',
                'decentralized_identifiers/create',
                'decentralized_identifiers/update',
                'decentralized_identifiers/secure',
                'decentralized_identifiers/resolve',
                'decentralized_identifiers/resolve_history',
                'decentralized_identifiers/private_tangle',
            ],
            'Verifiable Credentials': [
                'verifiable_credentials/overview',
                'verifiable_credentials/create',
                'verifiable_credentials/revoke',
                'verifiable_credentials/merkle_key_collection',
                'verifiable_credentials/verifiable_presentations',
            ],
            'DID Communication': [
                'did_communications/overview',
                'did_communications/did_comm_messages',
                'did_communications/protocols',
            ],
            'Advanced Concepts': [
                'advanced/overview',
                'advanced/client',
                'advanced/did_messages',
                'advanced/storage_adapter',
                'advanced/signature_schemes',

            ]
        },
        {
            type: 'category',
            label: 'Programming Languages',
            collapsed: true,
            items: [
                {
                    type: 'category',
                    label: 'Rust',
                    collapsed: true,
                    items: [
                        'libraries/rust/getting_started',
                        'libraries/rust/cheat_sheet',
                        'libraries/rust/api_reference',
                        'libraries/rust/troubleshooting',
                    ],

                },
                {
                    type: 'category',
                    label: 'WASM',
                    collapsed: true,
                    items: [
                        'libraries/wasm/getting_started',
                        'libraries/wasm/cheat_sheet',
                        'libraries/wasm/api_reference',
                        'libraries/wasm/troubleshooting',
                    ],
                },
            ],
        },
        {
            type: 'category',
            label: 'Specifications',
            collapsed: true,
            items: [
                'specs/overview',
                'specs/iota_did_method_spec',
                'specs/merkle_key_collection',
            ],
        },
        'glossary',
        'contribute',
        'contact',
        'faq'
    ],
};
