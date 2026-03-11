<!-- Run "wasm-pack build --target bundler" in Rust project -->
<!-- "npm install /path/to/your/wasm_parser/pkg" -->
"use client"; // This tells Next.js to run this in the browser

import { useEffect, useState } from "react";

export default function Home() {
  const [htmlContent, setHtmlContent] = useState<string>("Loading Rust...");

  useEffect(() => {
    // We dynamically import the Wasm package we installed
    import("wasm_parse").then((wasm) => {
      // We are calling your actual Rust code here!
      const result = wasm.parse_markdown_to_html("hello from webassembly!");
      setHtmlContent(result);
    });
  }, []);

  return (
    <main style={{ padding: "2rem", fontFamily: "sans-serif" }}>
      <h2>Next.js + Rust Wasm Sandbox</h2>
      <hr />
      {/* This will render the <h1> that Rust generated! */}
      <div dangerouslySetInnerHTML={{ __html: htmlContent }} />
    </main>
  );
}
