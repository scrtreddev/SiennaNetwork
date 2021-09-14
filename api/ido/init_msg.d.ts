/* tslint:disable */
/**
 * This file was automatically generated by json-schema-to-typescript.
 * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
 * and run json-schema-to-typescript to regenerate this file.
 */

export type HumanAddr = string;
/**
 * Binary is a wrapper around Vec<u8> to add base64 de/serialization with serde. It also adds some helper methods to help encode inline.
 *
 * This is only needed as serde-json-{core,wasm} has a horrible encoding for Vec<u8>
 */
export type Binary = string;
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
export type Uint128 = string;
export type SaleType = "PreLockAndSwap" | "PreLockOnly" | "SwapOnly";

export interface InitMsg {
  /**
   * Should be the address of the original sender, since this is initiated by the factory.
   */
  admin: HumanAddr;
  /**
   * Used by the IDO to register itself with the factory.
   */
  callback: CallbackFor_HumanAddr;
  entropy: Binary;
  info: TokenSaleConfig;
  /**
   * Used by the IDO to fill the whitelist spots with random pics
   */
  launchpad?: WhitelistRequest | null;
  /**
   * Seed for creating viewkey
   */
  prng_seed: Binary;
  [k: string]: unknown;
}
/**
 * Info needed to have the other contract respond.
 */
export interface CallbackFor_HumanAddr {
  /**
   * Info about the contract requesting the callback.
   */
  contract: ContractInstanceFor_HumanAddr;
  /**
   * The message to call.
   */
  msg: Binary;
  [k: string]: unknown;
}
/**
 * Info needed to talk to a contract instance.
 */
export interface ContractInstanceFor_HumanAddr {
  address: HumanAddr;
  code_hash: string;
  [k: string]: unknown;
}
export interface TokenSaleConfig {
  /**
   * The token that will be used to buy the SNIP20.
   */
  input_token: TokenTypeFor_HumanAddr;
  /**
   * The total amount that each participant is allowed to buy.
   */
  max_allocation: Uint128;
  /**
   * The maximum number of participants allowed.
   */
  max_seats: number;
  /**
   * The minimum amount that each participant is allowed to buy.
   */
  min_allocation: Uint128;
  /**
   * The price for a single token.
   */
  rate: Uint128;
  /**
   * Sale type settings
   */
  sale_type?: SaleType | null;
  sold_token: ContractInstanceFor_HumanAddr;
  /**
   * The addresses that are eligible to participate in the sale.
   */
  whitelist: HumanAddr[];
  [k: string]: unknown;
}
export interface WhitelistRequest {
  /**
   * Launchpad contract instance information
   */
  launchpad: ContractInstanceFor_HumanAddr;
  /**
   * Vector of tokens address needs to have locked in order to be considered for a draw. Tokens need to be configured in the Launchpad as eligible. Option<> is because if None that will represent a native token.
   */
  tokens: (HumanAddr | null)[];
  [k: string]: unknown;
}
