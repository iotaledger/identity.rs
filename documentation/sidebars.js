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
            type:'doc',
            id:'decentralized_identity',
            label:'Decentralized Identity'
        },
        {
            type: 'category',
            label: 'Getting Started',
            collapsed: false,
            items: [
                'getting-started/overview',
                {
                    'Decentralized Identifiers (DID)': [
                        'getting-started/decentralized_identifiers/overview',
                        'getting-started/decentralized_identifiers/create',
                        'getting-started/decentralized_identifiers/resolve',
                        'getting-started/decentralized_identifiers/update',
                        'getting-started/decentralized_identifiers/manage',
                        'getting-started/decentralized_identifiers/merkle_key_collection',
                    ],
                },
                {
                    'Verifiable Credentials': [
                        'getting-started/verifiable_credentials/overview',
                        'getting-started/verifiable_credentials/create',
                        'getting-started/verifiable_credentials/revoke',
                        'getting-started/verifiable_credentials/verifiable_presentations',
                    ]
                },
                {
                    'DID Communication': [
                        'getting-started/did_communications/overview',
                        'getting-started/did_communications/did_comm_messages',
                        'getting-started/did_communications/protocols',
                    ],
                    'Advanced Concepts': [
                        'getting-started/advanced/overview',
                        'getting-started/advanced/client',
                        'getting-started/advanced/did_messages',
                        'getting-started/advanced/storage_adapter',
                        'getting-started/advanced/signature_schemes',

                    ]
                },
            ],
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
                        'libraries/rust/overview',
                        'libraries/rust/getting_started',
                        'libraries/rust/examples',
                        'libraries/rust/api_reference',
                        'libraries/rust/troubleshooting',
                    ],

                },
                {
                    type: 'category',
                    label: 'WASM',
                    collapsed: true,
                    items: [
                        'libraries/wasm/overview',
                        'libraries/wasm/getting_started',
                        'libraries/wasm/examples',
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
            {
              type: 'category',
              label: 'IOTA DID',
              collapsed: true,
              items: [
                'specs/did/overview',
                'specs/did/iota_did_method_spec',
                'specs/did/merkle_key_collection',
              ]
            },
            {
              type: 'category',
              label: 'IOTA DIDComm',
              collapsed: true,
              items: [
                'specs/didcomm/overview',
                'specs/didcomm/CHANGELOG',
                {
                  type: 'category',
                  label: 'Resources',
                  collapsed: true,
                  items: [
                    'specs/didcomm/resources/credential-types',
                    'specs/didcomm/resources/problem-reports',
                  ]
                },
                {
                  type: 'category',
                  label: 'Protocols',
                  collapsed: true,
                  items: [
                    'specs/didcomm/protocols/connection',
                    'specs/didcomm/protocols/authentication',
                    'specs/didcomm/protocols/presentation',
                    'specs/didcomm/protocols/issuance',
                    'specs/didcomm/protocols/signing',
                    'specs/didcomm/protocols/revocation-options',
                    'specs/didcomm/protocols/revocation',
                    'specs/didcomm/protocols/post',
                    'specs/didcomm/protocols/termination',
                  ]
                },
              ]
            },
          ],
        },
        'glossary',
        'contribute',
        'contact',
        'faq'
    ],
};
