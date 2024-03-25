/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import type { AccountTypeArgs } from "../types";
import type {
  Account,
  Context,
  Pda,
  PublicKey,
  RpcAccount,
  RpcGetAccountOptions,
  RpcGetAccountsOptions,
} from "@metaplex-foundation/umi";
import type { Serializer } from "@metaplex-foundation/umi/serializers";

import {
  assertAccountExists,
  deserializeAccount,
  gpaBuilder,
  publicKey as toPublicKey,
} from "@metaplex-foundation/umi";
import {
  mapSerializer,
  publicKey as publicKeySerializer,
  string,
  struct,
} from "@metaplex-foundation/umi/serializers";

import { AccountType, getAccountTypeSerializer } from "../types";

export type Resolver = Account<ResolverAccountData>;

export type ResolverAccountData = {
  accountType: AccountType;
  market: PublicKey;
  request: PublicKey;
};

export type ResolverAccountDataArgs = { market: PublicKey; request: PublicKey };

export function getResolverAccountDataSerializer(): Serializer<
  ResolverAccountDataArgs,
  ResolverAccountData
> {
  return mapSerializer<ResolverAccountDataArgs, any, ResolverAccountData>(
    struct<ResolverAccountData>(
      [
        ["accountType", getAccountTypeSerializer()],
        ["market", publicKeySerializer()],
        ["request", publicKeySerializer()],
      ],
      { description: "ResolverAccountData" },
    ),
    (value) => ({ ...value, accountType: AccountType.Resolver }),
  );
}

export function deserializeResolver(rawAccount: RpcAccount): Resolver {
  return deserializeAccount(rawAccount, getResolverAccountDataSerializer());
}

export async function fetchResolver(
  context: Pick<Context, "rpc">,
  publicKey: PublicKey | Pda,
  options?: RpcGetAccountOptions,
): Promise<Resolver> {
  const maybeAccount = await context.rpc.getAccount(toPublicKey(publicKey, false), options);
  assertAccountExists(maybeAccount, "Resolver");
  return deserializeResolver(maybeAccount);
}

export async function safeFetchResolver(
  context: Pick<Context, "rpc">,
  publicKey: PublicKey | Pda,
  options?: RpcGetAccountOptions,
): Promise<Resolver | null> {
  const maybeAccount = await context.rpc.getAccount(toPublicKey(publicKey, false), options);
  return maybeAccount.exists ? deserializeResolver(maybeAccount) : null;
}

export async function fetchAllResolver(
  context: Pick<Context, "rpc">,
  publicKeys: Array<PublicKey | Pda>,
  options?: RpcGetAccountsOptions,
): Promise<Array<Resolver>> {
  const maybeAccounts = await context.rpc.getAccounts(
    publicKeys.map((key) => toPublicKey(key, false)),
    options,
  );
  return maybeAccounts.map((maybeAccount) => {
    assertAccountExists(maybeAccount, "Resolver");
    return deserializeResolver(maybeAccount);
  });
}

export async function safeFetchAllResolver(
  context: Pick<Context, "rpc">,
  publicKeys: Array<PublicKey | Pda>,
  options?: RpcGetAccountsOptions,
): Promise<Array<Resolver>> {
  const maybeAccounts = await context.rpc.getAccounts(
    publicKeys.map((key) => toPublicKey(key, false)),
    options,
  );
  return maybeAccounts
    .filter((maybeAccount) => maybeAccount.exists)
    .map((maybeAccount) => deserializeResolver(maybeAccount as RpcAccount));
}

export function getResolverGpaBuilder(context: Pick<Context, "rpc" | "programs">) {
  const programId = context.programs.getPublicKey(
    "parimutuelResolver",
    "RS1njPGQsykXyyPGUiAC9dvPyoqw73vtMFPJhipibj1",
  );
  return gpaBuilder(context, programId)
    .registerFields<{
      accountType: AccountTypeArgs;
      market: PublicKey;
      request: PublicKey;
    }>({
      accountType: [0, getAccountTypeSerializer()],
      market: [1, publicKeySerializer()],
      request: [33, publicKeySerializer()],
    })
    .deserializeUsing<Resolver>((account) => deserializeResolver(account))
    .whereField("accountType", AccountType.Resolver);
}

export function getResolverSize(): number {
  return 65;
}

export function findResolverPda(
  context: Pick<Context, "eddsa" | "programs">,
  seeds: {
    /** The address of the parimutuel market to resolve. */
    market: PublicKey;
  },
): Pda {
  const programId = context.programs.getPublicKey(
    "parimutuelResolver",
    "RS1njPGQsykXyyPGUiAC9dvPyoqw73vtMFPJhipibj1",
  );
  return context.eddsa.findPda(programId, [
    string({ size: "variable" }).serialize("resolver"),
    publicKeySerializer().serialize(seeds.market),
  ]);
}

export async function fetchResolverFromSeeds(
  context: Pick<Context, "eddsa" | "programs" | "rpc">,
  seeds: Parameters<typeof findResolverPda>[1],
  options?: RpcGetAccountOptions,
): Promise<Resolver> {
  return fetchResolver(context, findResolverPda(context, seeds), options);
}

export async function safeFetchResolverFromSeeds(
  context: Pick<Context, "eddsa" | "programs" | "rpc">,
  seeds: Parameters<typeof findResolverPda>[1],
  options?: RpcGetAccountOptions,
): Promise<Resolver | null> {
  return safeFetchResolver(context, findResolverPda(context, seeds), options);
}
