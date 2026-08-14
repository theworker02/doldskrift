import assert from "node:assert/strict";
import { readdir, readFile } from "node:fs/promises";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, it } from "node:test";
import { encode, decode, Encoder, Session, checksumHex } from "../dist/index.js";

interface Vector {
  version?: number;
  mode: string;
  input: string;
  encoded: string;
  checksum?: string;
  seed?: string;
  mapping_id?: string;
}

async function loadVectors(): Promise<{ name: string; data: Vector }[]> {
  const here = dirname(fileURLToPath(import.meta.url));
  const dirs = [
    join(here, "../../spec/vectors"),
    join(here, "../../../spec/vectors"),
    join(process.cwd(), "spec/vectors"),
    join(process.cwd(), "../../spec/vectors"),
  ];

  let dir: string | null = null;
  for (const d of dirs) {
    try {
      const entries = await readdir(d);
      if (entries.some((e) => e.endsWith(".json"))) {
        dir = d;
        break;
      }
    } catch {
      // try next
    }
  }
  if (!dir) {
    throw new Error(`could not find spec/vectors (tried ${dirs.join(", ")})`);
  }

  const files = (await readdir(dir)).filter((f) => f.endsWith(".json")).sort();
  const out: { name: string; data: Vector }[] = [];
  for (const file of files) {
    const raw = await readFile(join(dir, file), "utf8");
    out.push({ name: file, data: JSON.parse(raw) as Vector });
  }
  return out;
}

describe("golden vectors", async () => {
  const vectors = await loadVectors();
  assert.ok(vectors.length > 0, "expected at least one vector JSON file");

  for (const { name, data } of vectors) {
    it(`${name}: encode matches`, () => {
      const { mode, input, encoded, seed } = data;
      let actual: string;
      if (mode === "visual") {
        actual = new Encoder("visual").encode(input);
      } else if (mode === "session") {
        const session = Session.fromSeed(seed ?? "vector-seed", "v1");
        actual = session.encode(input);
        if (data.mapping_id) {
          assert.equal(session.mappingId(), data.mapping_id);
        }
      } else {
        actual = encode(input);
      }
      assert.equal(actual, encoded);
    });

    it(`${name}: decode roundtrip`, () => {
      const { mode, input, encoded, seed } = data;
      if (mode === "visual") {
        assert.equal(new Encoder("visual").encode(encoded), input);
      } else if (mode === "session") {
        const session = Session.fromSeed(seed ?? "vector-seed", "v1");
        assert.equal(session.decode(encoded), input);
      } else {
        assert.equal(decode(encoded), input);
      }
    });

    if (data.checksum !== undefined) {
      it(`${name}: input checksum`, () => {
        const hex = checksumHex(new TextEncoder().encode(data.input));
        assert.equal(hex, data.checksum);
      });
    }
  }
});
