import { CopyBlock, dracula } from "react-code-blocks";

type AllowedLanguages = "c" | "rust";

function Code({
  language,
  content,
}: {
  language: AllowedLanguages;
  content: string;
}) {
  return (
    <CopyBlock
      text={content}
      language={language}
      theme={dracula}
      codeBlock={true}
    />
  );
}

export default Code;
