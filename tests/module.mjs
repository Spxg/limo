import { assert } from "./fixtures/assert.mjs";
import { message } from "./fixtures/message.mjs";

assert(message === "hello from module", `Unexpected module export: ${message}`);
