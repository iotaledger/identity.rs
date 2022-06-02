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
          type: 'category',
          label: 'Concepts',
          collapsed: false,
          items: [
            {
              'Decentralized Identifiers (DID)': [
                  'concepts/decentralized_identifiers/overview',
                  'concepts/decentralized_identifiers/create',
                  'concepts/decentralized_identifiers/update',
                  'concepts/decentralized_identifiers/resolve',
                  'concepts/decentralized_identifiers/private_tangle',
              ],
              'Verifiable Credentials': [
                  'concepts/verifiable_credentials/overview',
                  'concepts/verifiable_credentials/create',
                  'concepts/verifiable_credentials/revoke',
                  'concepts/verifiable_credentials/verifiable_presentations',
              ],
              'Advanced Concepts': [
                  'concepts/advanced/overview',
                  'concepts/advanced/did_messages',
                  'concepts/advanced/storage_interface',
  
              ]
            },
          ],
        },
        {
            type: 'category',
            label: 'Programming Languages',
            collapsed: true,
            items: [
                'libraries/overview',
                {
                    type: 'category',
                    label: 'Rust',
                    collapsed: true,
                    items: [
                        'libraries/rust/getting_started',
                        'libraries/rust/cheat_sheet',
                        'libraries/rust/api_reference',
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
                    ],
                },
            ],
        },
        {
          type: 'category',
          label: 'Tutorials',
          items:['tutorials/overview','tutorials/validate_university_degree']
        },
        {
          type: 'category',
          label: 'Specifications',
          collapsed: true,
          items: [
            'specs/overview',
            {
              type: 'category',
              label: 'IOTA DID',
              collapsed: true,
              items: [
                'specs/did/overview',
                'specs/did/iota_did_method_spec',
              ]
            },
            {
              type: 'category',
              label: 'IOTA DIDComm',
              collapsed: true,
              items: [
                'specs/didcomm/overview',
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
                    'specs/didcomm/protocols/revocation',
                    'specs/didcomm/protocols/revocation-options',
                    'specs/didcomm/protocols/post',
                    'specs/didcomm/protocols/termination',
                  ]
                },
                {
                  type: 'category',
                  label: 'Resources',
                  collapsed: true,
                  items: [
                    'specs/didcomm/resources/credential-info',
                    'specs/didcomm/resources/problem-reports',
                  ]
                },
                'specs/didcomm/CHANGELOG',
              ]
            },
          ],
        },
        'glossary',
        'contribute',
        'workflow',
        'contact',
        'faq'
    ],
};
