/* tslint:disable */
/**
 * This file was automatically generated by json-schema-to-typescript.
 * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
 * and run json-schema-to-typescript to regenerate this file.
 */

export type QueryMsgResponse = {
  pair_info: {
    amount_0: Uint128;
    amount_1: Uint128;
    contract_version: number;
    factory: ContractInstanceFor_HumanAddr;
    liquidity_token: ContractInstanceFor_HumanAddr;
    pair: TokenPairFor_HumanAddr;
    total_liquidity: Uint128;
    [k: string]: unknown;
  };
  [k: string]: unknown;
};
export type Uint128 = string;
export type HumanAddr = string;
export type TokenPairFor_HumanAddr = [TokenTypeFor_HumanAddr, TokenTypeFor_HumanAddr];
export type TokenTypeFor_HumanAddr =
  | {
      custom_token: {
        contract_addr: HumanAddr;
        token_code_hash: string;
        [k: string]: unknown;
      };
      [k: string]: unknown;
    }
  | {
      native_token: {
        denom: string;
        [k: string]: unknown;
      };
      [k: string]: unknown;
    };

/**
 * Info needed to talk to a contract instance.
 */
export interface ContractInstanceFor_HumanAddr {
  address: HumanAddr;
  code_hash: string;
  [k: string]: unknown;
}
