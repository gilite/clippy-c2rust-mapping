import { Tab, TabList, TabPanel, TabPanels, Tabs } from "@chakra-ui/react";
import { useState } from "react";
import CRustCard from "./CRustCard";

interface Snippet {
  content: string;
}

interface FileOutput {
  name: string;
  c: Snippet;
  rust: Snippet;
}

const c1 =
  'float a = 1.f / 81;\nfloat b = 0;\nfor (int i = 0; i < 729; ++ i)\n    b += a;\nprintf("%.7g\n", b); // prints 9.000023';

const fakeFileOutput: FileOutput[] = [
  { name: "Tab 1", c: { content: c1 }, rust: { content: "<hello>" } },
  { name: "Tab 2", c: { content: "<world>" }, rust: { content: "<world>" } },
  { name: "Tab 3", c: { content: "<whoop>" }, rust: { content: "<whoop>" } },
];

function MappingsList() {
  const [fileOutput, setFileOutput] = useState<FileOutput[]>(fakeFileOutput);

  return (
    <Tabs orientation="vertical" colorScheme="teal" isLazy>
      <TabList minWidth={36} aria-orientation="vertical">
        {fileOutput.map((file) => (
          <Tab>{file.name}</Tab>
        ))}
      </TabList>
      <TabPanels>
        {fileOutput.map((file) => (
          <TabPanel>
            <CRustCard
              cContent={file.c.content}
              rustContent={file.rust.content}
            />
          </TabPanel>
        ))}
      </TabPanels>
    </Tabs>
  );
}

export default MappingsList;
