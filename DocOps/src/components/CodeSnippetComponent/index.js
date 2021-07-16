import clsx from "clsx";
import React from "react";
import "./styles.css";

export default function CodeSnippet({nodeReplitLink, RustReplitLink}) {
    const [lang, setLang] = React.useState("node");
    return (
        <div>
            <div className={clsx("langSelector")}>
            <button
                className={clsx("button", "languageButton", "mr-sm", {
                    activeButton: lang === "node",
                    inactiveButton: lang !== "node",
                })}
                onClick={() => setLang("node")}
            >
                Node.js
            </button>
            <button
                className={clsx("button", "languageButton", {
                    activeButton: lang == "rust",
                    inactiveButton: lang !== "rust",
                })}
                onClick={() => setLang("rust")}
            >
                Rust
            </button>
            </div>
            <div className={clsx("codeSnippetContainer")}>
                {lang === "node" ? (
                    <iframe
                        frameborder="0"
                        width="100%"
                        height="800px"
                        src={nodeReplitLink}
                        // src="https://repl.it/@abdulmth/Create-did?lite=true"
                    ></iframe>
                ) : (
                    <iframe
                        frameborder="0"
                        width="100%"
                        height="800px"
                        src={RustReplitLink}
                        // src="https://repl.it/@abdulmth/create-did-rust?lite=true"
                    ></iframe>
                )}
            </div>
        </div>
    );
}
