import clsx from "clsx";
import React, { useEffect } from "react";
import "./styles.css";

export default function CodeSnippet({ nodeReplitLink, rustReplitLink }) {
    const [lang, setLang] = React.useState("node");

    useEffect(() => {
        //Set Language from storage, default to node
        let langFromStorage = localStorage.getItem("lang");
        let lang = langFromStorage ? langFromStorage : "node";

        //If Replit doesn't exist default to next option
        if(lang === "node" && !nodeReplitLink) {
            lang = "rust";
        }
        if(lang === "rust" && !rustReplitLink) {
            lang = "node";
        }
        setLang(lang);
    });

    return (
        <div>
            <div className={clsx("langSelector")}>
                { nodeReplitLink &&
                <button
                    className={clsx("button", "languageButton", "mr-sm", {
                        activeButton: lang === "node",
                        inactiveButton: lang !== "node",
                    })}
                    onClick={() => {
                        localStorage.setItem("lang", "node");
                        setLang("node");
                    }}
                >
                    Node.js
                </button>
                }
                { rustReplitLink &&
                <button
                    className={clsx("button", "languageButton", {
                        activeButton: lang == "rust",
                        inactiveButton: lang !== "rust",
                    })}
                    onClick={() => {
                        localStorage.setItem("lang", "rust");
                        setLang("rust");
                    }}
                >
                    Rust
                </button>
                }
            </div>
            <div className={clsx("codeSnippetContainer")}>
                {lang === "node" ? (
                    <iframe
                        frameborder="0"
                        width="100%"
                        height="600px"
                        src={nodeReplitLink}
                    ></iframe>
                ) : (
                    <iframe
                        frameborder="0"
                        width="100%"
                        height="600px"
                        src={rustReplitLink}
                    ></iframe>
                )}
            </div>
        </div>
    );
}