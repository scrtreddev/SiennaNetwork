import { b64encode, b64decode, b64fromBuffer } from "@waiting/base64";
import { EnigmaUtils, ExecuteResult, InstantiateResult } from "secretjs";

export type Uint128 = string;
export type Address = string;
export type TokenType = CustomToken | NativeToken;
export type Decimal = string;
/**
 * Base64 encoded
 */
export type ViewingKey = string

export class TokenPair {
    constructor(
        readonly token_0: TokenType,
        readonly token_1: TokenType
    ) { }
}

export class TokenPairAmount {
    constructor(
        readonly pair: TokenPair,
        readonly amount_0: Uint128,
        readonly amount_1: Uint128
    ) { }
}

export class TokenTypeAmount {
    constructor(
        readonly token: TokenType,
        readonly amount: Uint128
    ) { }
}

export interface CustomToken {
    custom_token: {
        contract_addr: Address;
        token_code_hash: string;
    };
}

export interface NativeToken {
    native_token: {
        denom: string;
    };
}

// These two are not exported in secretjs...
export interface Coin {
    readonly denom: string;
    readonly amount: string;
}

export interface Fee {
    readonly amount: ReadonlyArray<Coin>
    readonly gas: Uint128
}

export function decode_data<T>(result: ExecuteResult | InstantiateResult): T {
    const b64string = b64fromBuffer(result.data)

    return JSON.parse(b64decode(b64string))
}

export function create_coin(amount: Uint128): Coin {
    return {
        denom: 'uscrt',
        amount: `${amount}`
    }
}

export function create_fee(amount: Uint128, gas?: Uint128 | undefined): Fee {
    if (gas === undefined) {
        gas = amount
    }

    return {
        amount: [{ amount: `${amount}`, denom: "uscrt" }],
        gas: `${gas}`,
    }
}

export function create_base64_msg(msg: object): string {
    return b64encode(JSON.stringify(msg))
}

export function create_entropy(): string {
    const rand = EnigmaUtils.GenerateNewSeed().toString()
    return b64encode(rand)
}

export function add_native_balance_pair(amount: TokenPairAmount): Coin[] | undefined {
    let result: Coin[] | undefined = []

    if (get_token_type(amount.pair.token_0) == TypeOfToken.Native) {
        result.push(create_coin(amount.amount_0))
    }
    else if (get_token_type(amount.pair.token_1) == TypeOfToken.Native) {
        result.push(create_coin(amount.amount_1))
    } else {
        result = undefined
    }

    return result
}

export function add_native_balance(amount: TokenTypeAmount): Coin[] | undefined {
    let result: Coin[] | undefined = []

    if (get_token_type(amount.token) == TypeOfToken.Native) {
        result.push(create_coin(amount.amount))
    }
    else {
        result = undefined
    }

    return result
}

export enum TypeOfToken {
    Native,
    Custom
}

export function get_token_type(token: TokenType): TypeOfToken {
    const raw = token as Object

    if (raw.hasOwnProperty('native_token')) {
        return TypeOfToken.Native
    }

    return TypeOfToken.Custom
}

export class ContractInfo {
    constructor(
        readonly code_hash: string,
        readonly address: Address
    ) { }
}

export class ContractInstantiationInfo {
    constructor(
        readonly code_hash: string,
        readonly id: number
    ) { }
}
