/* tslint:disable */
/**
 * This file was automatically generated by json-schema-to-typescript.
 * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
 * and run json-schema-to-typescript to regenerate this file.
 */

export type HandleMsg =
  | {
      set_status: {
        level: ContractStatusLevel;
        new_address?: HumanAddr | null;
        reason: string;
        [k: string]: unknown;
      };
      [k: string]: unknown;
    }
  | {
      receive: {
        amount: Uint128;
        from: HumanAddr;
        msg?: Binary | null;
        [k: string]: unknown;
      };
      [k: string]: unknown;
    }
  | {
      lock: {
        amount: Uint128;
        [k: string]: unknown;
      };
      [k: string]: unknown;
    }
  | {
      unlock: {
        entries: number;
        [k: string]: unknown;
      };
      [k: string]: unknown;
    }
  | {
      draw: {
        callback: ContractInstanceFor_HumanAddr;
        number: number;
        tokens: (HumanAddr | null)[];
        [k: string]: unknown;
      };
      [k: string]: unknown;
    }
  | {
      admin_add_token: {
        config: TokenSettings;
        [k: string]: unknown;
      };
      [k: string]: unknown;
    }
  | {
      admin_remove_token: {
        index: number;
        [k: string]: unknown;
      };
      [k: string]: unknown;
    }
  | {
      admin: AdminHandleMsg;
      [k: string]: unknown;
    }
  | {
      create_viewing_key: {
        entropy: string;
        padding?: string | null;
        [k: string]: unknown;
      };
      [k: string]: unknown;
    }
  | {
      set_viewing_key: {
        key: string;
        padding?: string | null;
        [k: string]: unknown;
      };
      [k: string]: unknown;
    };
/**
 * Possible states of a contract.
 */
export type ContractStatusLevel = "Operational" | "Paused" | "Migrating";
export type HumanAddr = string;
export type Uint128 = string;
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
export type AdminHandleMsg = {
  change_admin: {
    address: HumanAddr;
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
/**
 * Configuration for single token that can be locked into the launchpad
 */
export interface TokenSettings {
  bounding_period: number;
  segment: Uint128;
  token_type: TokenTypeFor_HumanAddr;
  [k: string]: unknown;
}
