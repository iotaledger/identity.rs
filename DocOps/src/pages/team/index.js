import React from 'react';
import styles from './styles.module.css';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import Layout from '@theme/Layout';
import TEAM from './teamData';
import TeamMember from './TeamMember';

export default function Team() {
  const { siteConfig } = useDocusaurusContext();
  return (
    <Layout
      title={`Identity Team Members`}
      description="IOTA identity team">
      <main>
        <section className={styles.section}>
          <div className="container">
            <h1 className="hero__title">IOTA Identity Team Members</h1>
            <p className="hero__subtitle">
              <ul>
                <li>
                Jelle is responsible for the architecture of the IOTA Identity framework and management of the team. He directly interacts with IOTAs partners and community to promote for adoption. He joined the IOTA Foundation in Aug, 2019.
                </li>
                <li>
                  Devin, Philipp and Craig work on developing the core framework.
                </li>
                <li>
                  Abdulrahim works on improving the usability of the framework, by building improved developer tooling, documentation, examples and bindings.
                </li>
              </ul>
            </p>
            <div className="row">
              {TEAM.map((item, index) => (
                <TeamMember key={index} {...item} />
              ))}
            </div>
          </div>
        </section>
      </main>
    </Layout>
  );
}
