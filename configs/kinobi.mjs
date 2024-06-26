// @ts-check

import { execFile } from "child_process";
import path from "path";
import { fileURLToPath } from "url";

import * as k from "@metaplex-foundation/kinobi";
import { bold } from "colorette";
import { ESLint } from "eslint";

const __dirname = fileURLToPath(new URL(".", import.meta.url));

const rootDir = path.dirname(__dirname);

const idlDir = path.join(rootDir, "idls");
const clientDir = path.join(rootDir, "clients");

const jsDir = path.join(clientDir, "js", "src", "generated");
const crateDir = path.join(clientDir, "rust");
const rustDir = path.join(crateDir, "src", "generated");

const start = Date.now();

console.log("generating client code...");

const kinobi = k.createFromIdls([path.join(idlDir, "resolver.json")]);

kinobi.update(
  k.updateProgramsVisitor({
    resolver: {
      name: "parimutuelResolver",
    },
  }),
);

kinobi.update(k.defaultVisitor());

// Update accounts.
kinobi.update(
  k.updateAccountsVisitor({
    resolver: {
      seeds: [
        k.constantPdaSeedNodeFromString("resolver"),
        k.variablePdaSeedNode(
          "market",
          k.publicKeyTypeNode(),
          "The address of the parimutuel market to resolve.",
        ),
      ],
    },
  }),
);

// Set default values for instruction accounts.
kinobi.update(
  k.setInstructionAccountDefaultValuesVisitor([
    {
      account: "parimutuelProgram",
      ignoreIfOptional: true,
      defaultValue: k.publicKeyValueNode(
        "Cf9JrByfmw6CYSry39pfg2BSGHRgde2Cp5y6yZ3a2Yeo",
        "hplParimutuel",
      ),
    },
    {
      account: "resolver",
      ignoreIfOptional: true,
      defaultValue: k.pdaValueNode("resolver", [
        k.pdaSeedValueNode("market", k.accountValueNode("market")),
      ]),
    },
  ]),
);

/** @param {string} name */
const accountType = (name) => ({
  field: "accountType",
  value: k.enumValueNode("AccountType", name),
});

// Set account discriminators.
kinobi.update(
  k.setAccountDiscriminatorFromFieldVisitor({
    Resolver: accountType("Resolver"),
  }),
);

// Render Rust.
{
  console.log(`writing rust client to ${bold(path.relative(rootDir, rustDir))}...`);

  kinobi.accept(
    k.renderRustVisitor(rustDir, {
      formatCode: true,
      crateFolder: crateDir,
    }),
  );

  console.log("cleaning up generated rust client code...");

  execFile("cargo", ["fmt", `--manifest-path=${path.join(crateDir, "Cargo.toml")}`]);
}

// Render JavaScript.
{
  console.log(`writing js client to ${bold(path.relative(rootDir, jsDir))}...`);

  kinobi.accept(
    k.renderJavaScriptVisitor(jsDir, {
      formatCode: true,
    }),
  );

  console.log("cleaning up generated js client code...");

  const eslint = new ESLint({
    cache: true,
    cacheLocation: path.join(rootDir, "node_modules", ".cache", "eslint-kinobi"),
    cacheStrategy: "content",
    fix: true,
  });
  const lintResults = await eslint.lintFiles(jsDir);

  await ESLint.outputFixes(lintResults);

  const eslintFormatter = await eslint.loadFormatter();
  const lintOutput = await eslintFormatter.format(lintResults);

  if (lintOutput) {
    console.error(lintOutput);
  }
}

console.log(`done in ${bold(`${Date.now() - start}ms`)}`);
