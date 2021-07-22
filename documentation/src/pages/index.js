import React from "react";
import Layout from "@theme/Layout";
import useDocusaurusContext from "@docusaurus/useDocusaurusContext";
import LandingpageHeader from "../components/LandingpageHeader";

export default function Home() {
    const { siteConfig } = useDocusaurusContext();
    return (
        <Layout
            title={`IOTA Identity Documentation`}
            description="Providing Trust between Individuals, Organizations and Things."
        >
            <LandingpageHeader />
        </Layout>
    );
}
