// The Rust root tested from TypeScript: `nsh test` runs this file in a
// headless runner it builds with the crate, and `open(".")` starts the root
// (`nsh.json` "root").
import { expect, open, test } from "nsh/test";

test("shows the heading, tabs and the active placeholder", async () => {
  const app = await open(".");
  expect(app.texts()).toContain("Ruder Cam Beta");
  expect(app.texts()).toContain("Home placeholder");
  await app.press("Fahrtenbuch");
  expect(app.texts()).toContain("Fahrtenbuch placeholder");
  await app.press("Settings");
  expect(app.texts()).toContain("Settings placeholder");
});
