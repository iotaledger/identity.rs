import clsx from "clsx";
import React, { useEffect } from "react";
import "./styles.css";

export default function CodeSnippet({ nodeReplitLink, rustReplitLink }) {
    const [lang, setLang] = React.useState("node");

    useEffect(() => {
        let langFromStorage = localStorage.getItem("lang");
        if (!langFromStorage) {
            setLang("node");
        } else {
            setLang(langFromStorage);
        }
    });

    return (
        <div>
            <div className={clsx("langSelector")}>
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
            </div>
            <div className={clsx("codeSnippetContainer")}>
                {lang === "node" ? (
                    <iframe
                        frameborder="0"
                        width="100%"
                        height="600px"
                        src={nodeReplitLink}
                        // src="https://repl.it/@abdulmth/Create-did?lite=true"
                    ></iframe>
                ) : (
                    <iframe
                        frameborder="0"
                        width="100%"
                        height="600px"
                        src={rustReplitLink}
                        // src="https://repl.it/@abdulmth/create-did-rust?lite=true"
                    ></iframe>
                )}
            </div>
        </div>
    );
}
