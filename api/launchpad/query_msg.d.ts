/* tslint:disable */
/**
 * This file was automatically generated by json-schema-to-typescript.
 * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
 * and run json-schema-to-typescript to regenerate this file.
 */

export type QueryMsg =
  | ("status" | "launchpad_info")
  | {
      admin: AdminQueryMsg;
      [k: string]: unknown;
    }
  | {
      user_info: {
        address: HumanAddr;
        key: string;
        [k: string]: unknown;
      };
      [k: string]: unknown;
    };
export type AdminQueryMsg = "admin";
export type HumanAddr = string;
