import clsx from "clsx";
import React, {useEffect} from "react";
import "./styles.css";
import CodeBlock from "@theme/CodeBlock";

export default function CodeSnippet({nodeReplitLink, nodeContent, rustContent, nodeGithubLink, rustGithubLink}) {
  const [lang, setLang] = React.useState("node");

  const ARROW_OUT_OF_BOX_ICON = (
    <svg
      xmlns="http://www.w3.org/2000/svg"
      width="12"
      height="12"
      fill="currentColor"
      class="bi bi-box-arrow-up-right"
      viewBox="0 0 16 16"
    >
      <path
        fill-rule="evenodd"
        d="M8.636 3.5a.5.5 0 0 0-.5-.5H1.5A1.5 1.5 0 0 0 0 4.5v10A1.5 1.5 0 0 0 1.5 16h10a1.5 1.5 0 0 0 1.5-1.5V7.864a.5.5 0 0 0-1 0V14.5a.5.5 0 0 1-.5.5h-10a.5.5 0 0 1-.5-.5v-10a.5.5 0 0 1 .5-.5h6.636a.5.5 0 0 0 .5-.5z"
      />
      <path
        fill-rule="evenodd"
        d="M16 .5a.5.5 0 0 0-.5-.5h-5a.5.5 0 0 0 0 1h3.793L6.146 9.146a.5.5 0 1 0 .708.708L15 1.707V5.5a.5.5 0 0 0 1 0v-5z"
      />
    </svg>
  );

  useEffect(() => {
    //Set Language from storage, default to node
    let langFromStorage = localStorage.getItem("lang");
    let lang = langFromStorage ? langFromStorage : "node";

    //If Node doesn't exist, default to next option
    if (lang === "node" && !nodeReplitLink && !nodeContent) {
      lang = "rust";
    }
    if (lang === "rust" && !rustContent) {
      lang = "node";
    }
    setLang(lang);
  });

  return (
    <div>

      {/***** Tabs *****/}
      <div className={clsx("langSelector")}>
        {rustContent && (
          <button
              className={clsx("button", "languageButton", {
                  activeButton: lang == "rust",
                  inactiveButton: lang !== "rust"
              })}
              onClick={() => {
                  localStorage.setItem("lang", "rust");
                  setLang("rust");
              }}
          >Rust</button>
        )}
        {(nodeReplitLink || nodeContent) && (
          <button
            className={clsx("button", "languageButton", "mr-sm", {
              activeButton: lang === "node",
              inactiveButton: lang !== "node"
            })}
            onClick={() => {
              localStorage.setItem("lang", "node");
              setLang("node");
            }}
          >Node.js</button>
        )}
      </div>

      {/***** Code Snippet *****/}
      <div className={clsx("codeSnippetContainer")}>
        {
            (() => {
                if(lang === "node" && nodeReplitLink) {
                    return (
                        <>
                            <iframe frameborder="0" width="100%" height="700px" src={nodeReplitLink}></iframe>
                        </>
                    )
                } else if (lang === "node" && nodeContent) {
                    return (
                        <div className={clsx("nodeContainer")}>
                            <CodeBlock className={clsx("noMarginBottom")} language="typescript">
                                {nodeContent}
                            </CodeBlock>
                        </div>
                    )
                } else {
                    return (
                        <div className={clsx("rustContainer")}>
                            <CodeBlock className={clsx("noMarginBottom")} language="rust">
                                {rustContent}
                            </CodeBlock>
                        </div>
                    )
                }
            })()
        }
      </div>

      {/*****  Github Link *****/}
      <div className={clsx("githubLink")}>
        {nodeGithubLink && lang === "node" && (
          <a href={nodeGithubLink} target="_blank">
            GitHub&nbsp;
            {ARROW_OUT_OF_BOX_ICON}
          </a>
        )}
        {rustGithubLink && lang === "rust" && (
          <a href={rustGithubLink} target="_blank">
            GitHub&nbsp;
            {ARROW_OUT_OF_BOX_ICON}
          </a>
        )}
      </div>
    </div>
  );
}
