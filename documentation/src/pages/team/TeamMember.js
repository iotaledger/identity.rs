import React from 'react';
import styles from './styles.module.css';
import SocialLink from './SocialLink';

const TeamMember = function(props) {
  let { name, title, desc, image_url, social_links } = props

  if(!social_links) {
    social_links = []
  }

  return (
    <div className="col col--4">
      <div className={styles.teamMemberCard}>
        <div className={styles.cardHeader}>
          <img className={styles.image} src={image_url} alt={`Image of ${name}`} />
          <h1 className={styles.name}>{name}</h1>
          <p className={styles.title}>{title}</p>
          <p className={styles.title}>{desc}</p>
        </div>
        <div className={styles.cardFooter}>
          {social_links.map((item, index) => (
            <SocialLink key={index} {...item} />
          ))}
        </div>
      </div>
    </div>
  )
}

export default TeamMember;

