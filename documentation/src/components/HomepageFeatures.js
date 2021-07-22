import React from 'react';
import clsx from 'clsx';
import styles from './HomepageFeatures.module.css';
import LandingpageHeader from './LandingpageHeader';

const FeatureList = [
  {
    title: 'Identity Of Things',
    Svg: require('../../static/img/undraw_docusaurus_mountain.svg').default,
    description: (
      <>
       Devices have an identity that proves their capabilities, specifications, and authenticity to allow others to feel confident in transacting with them.
      </>
    ),
  },
  {
    title: 'Self Sovereign Identity',
    Svg: require('../../static/img/undraw_docusaurus_tree.svg').default,
    description: (
      <>
        Individuals have a borderless digital identity that can be verified by anyone or anything in the world.
      </>
    ),
  },
  {
    title: 'Regulatory Compliance',
    Svg: require('../../static/img/undraw_docusaurus_react.svg').default,
    description: (
      <>
        Organizations can use digital identities to follow regulations such as GDPR in a more cost-efficient way.
      </>
    ),
  },
];

function Feature({Svg, title, description}) {
  return (
    <div className={clsx('col col--4')}>
      <div className="text--center">
        <Svg className={styles.featureSvg} alt={title} />
      </div>
      <div className="text--center padding-horiz--md">
        <h3>{title}</h3>
        <p>{description}</p>
      </div>
    </div>
  );
}

export default function HomepageFeatures() {
  return (
    <section className={styles.features}>
      <div className="container">
        <div className="row">
          {FeatureList.map((props, idx) => (
            <Feature key={idx} {...props} />
          ))}
        </div>
      </div>
    </section>
  );
}
