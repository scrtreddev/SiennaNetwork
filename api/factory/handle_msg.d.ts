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
      set_config: {
        exchange_settings?: ExchangeSettingsFor_HumanAddr | null;
        ido_contract?: ContractInstantiationInfo | null;
        launchpad_contract?: ContractInstantiationInfo | null;
        lp_token_contract?: ContractInstantiationInfo | null;
        pair_contract?: ContractInstantiationInfo | null;
        snip20_contract?: ContractInstantiationInfo | null;
        [k: string]: unknown;
      };
      [k: string]: unknown;
    }
  | {
      create_exchange: {
        entropy: Binary;
        pair: TokenPairFor_HumanAddr;
        [k: string]: unknown;
      };
      [k: string]: unknown;
    }
  | {
      create_launchpad: {
        entropy: Binary;
        tokens: TokenSettings[];
        [k: string]: unknown;
      };
      [k: string]: unknown;
    }
  | {
      create_ido: {
        entropy: Binary;
        info: TokenSaleConfig;
        tokens?: (HumanAddr | null)[] | null;
        [k: string]: unknown;
      };
      [k: string]: unknown;
    }
  | {
      ido_whitelist: {
        addresses: HumanAddr[];
        [k: string]: unknown;
      };
      [k: string]: unknown;
    }
  | {
      register_exchange: {
        pair: TokenPairFor_HumanAddr;
        signature: Binary;
        [k: string]: unknown;
      };
      [k: string]: unknown;
    }
  | {
      register_launchpad: {
        signature: Binary;
        [k: string]: unknown;
      };
      [k: string]: unknown;
    }
  | {
      register_ido: {
        signature: Binary;
        [k: string]: unknown;
      };
      [k: string]: unknown;
    }
  | {
      add_exchanges: {
        exchanges: ExchangeFor_HumanAddr[];
        [k: string]: unknown;
      };
      [k: string]: unknown;
    }
  | {
      add_idos: {
        idos: HumanAddr[];
        [k: string]: unknown;
      };
      [k: string]: unknown;
    }
  | {
      add_launchpad: {
        launchpad: ContractInstanceFor_HumanAddr;
        [k: string]: unknown;
      };
      [k: string]: unknown;
    }
  | {
      admin: AdminHandleMsg;
      [k: string]: unknown;
    };
/**
 * Possible states of a contract.
 */
export type ContractStatusLevel = "Operational" | "Paused" | "Migrating";
export type HumanAddr = string;
/**
 * Binary is a wrapper around Vec<u8> to add base64 de/serialization with serde. It also adds some helper methods to help encode inline.
 *
 * This is only needed as serde-json-{core,wasm} has a horrible encoding for Vec<u8>
 */
export type Binary = string;
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
export type Uint128 = string;
export type SaleType = "PreLockAndSwap" | "PreLockOnly" | "SwapOnly";
export type AdminHandleMsg = {
  change_admin: {
    address: HumanAddr;
    [k: string]: unknown;
  };
  [k: string]: unknown;
};

export interface ExchangeSettingsFor_HumanAddr {
  sienna_burner?: HumanAddr | null;
  sienna_fee: Fee;
  swap_fee: Fee;
  [k: string]: unknown;
}
export interface Fee {
  denom: number;
  nom: number;
  [k: string]: unknown;
}
/**
 * Info needed to instantiate a contract.
 */
export interface ContractInstantiationInfo {
  code_hash: string;
  id: number;
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
/**
 * Info needed to talk to a contract instance.
 */
export interface ContractInstanceFor_HumanAddr {
  address: HumanAddr;
  code_hash: string;
  [k: string]: unknown;
}
/**
 * Represents the address of an exchange and the pair that it manages
 */
export interface ExchangeFor_HumanAddr {
  /**
   * Address of the contract that manages the exchange.
   */
  address: HumanAddr;
  /**
   * The pair that the contract manages.
   */
  pair: TokenPairFor_HumanAddr;
  [k: string]: unknown;
}
