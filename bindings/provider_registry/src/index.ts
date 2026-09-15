import { Buffer } from "buffer";
import { Address } from "@stellar/stellar-sdk";
import {
  AssembledTransaction,
  Client as ContractClient,
  ClientOptions as ContractClientOptions,
  MethodOptions,
  Result,
  Spec as ContractSpec,
} from "@stellar/stellar-sdk/contract";
import type {
  u32,
  i32,
  u64,
  i64,
  u128,
  i128,
  u256,
  i256,
  Option,
  Timepoint,
  Duration,
} from "@stellar/stellar-sdk/contract";
export * from "@stellar/stellar-sdk";
export * as contract from "@stellar/stellar-sdk/contract";
export * as rpc from "@stellar/stellar-sdk/rpc";

if (typeof window !== "undefined") {
  //@ts-ignore Buffer exists
  window.Buffer = window.Buffer || Buffer;
}





export interface Provider {
  metadata_hash: Buffer;
  verified: boolean;
}


export interface Client {
  /**
   * Construct and simulate a register transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  register: ({provider_ref, authority, metadata_hash}: {provider_ref: Buffer, authority: string, metadata_hash: Buffer}, options?: MethodOptions) => Promise<AssembledTransaction<null>>

  /**
   * Construct and simulate a get_status transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  get_status: ({provider_ref}: {provider_ref: Buffer}, options?: MethodOptions) => Promise<AssembledTransaction<boolean>>

  /**
   * Construct and simulate a set_status transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  set_status: ({provider_ref, verified}: {provider_ref: Buffer, verified: boolean}, options?: MethodOptions) => Promise<AssembledTransaction<null>>

}
export class Client extends ContractClient {
  static async deploy<T = Client>(
        /** Constructor/Initialization Args for the contract's `__constructor` method */
        {authority}: {authority: string},
    /** Options for initializing a Client as well as for calling a method, with extras specific to deploying. */
    options: MethodOptions &
      Omit<ContractClientOptions, "contractId"> & {
        /** The hash of the Wasm blob, which must already be installed on-chain. */
        wasmHash: Buffer | string;
        /** Salt used to generate the contract's ID. Passed through to {@link Operation.createCustomContract}. Default: random. */
        salt?: Buffer | Uint8Array;
        /** The format used to decode `wasmHash`, if it's provided as a string. */
        format?: "hex" | "base64";
      }
  ): Promise<AssembledTransaction<T>> {
    return ContractClient.deploy({authority}, options)
  }
  constructor(public readonly options: ContractClientOptions) {
    super(
      new ContractSpec([ "AAAAAQAAAAAAAAAAAAAACFByb3ZpZGVyAAAAAgAAAAAAAAANbWV0YWRhdGFfaGFzaAAAAAAAA+4AAAAgAAAAAAAAAAh2ZXJpZmllZAAAAAE=",
        "AAAABQAAAAAAAAAAAAAADlByb3ZpZGVyU3RhdHVzAAAAAAABAAAAD3Byb3ZpZGVyX3N0YXR1cwAAAAACAAAAAAAAAAxwcm92aWRlcl9yZWYAAAPuAAAAIAAAAAEAAAAAAAAACHZlcmlmaWVkAAAAAQAAAAAAAAAC",
        "AAAAAAAAAAAAAAAIcmVnaXN0ZXIAAAADAAAAAAAAAAxwcm92aWRlcl9yZWYAAAPuAAAAIAAAAAAAAAAJYXV0aG9yaXR5AAAAAAAAEwAAAAAAAAANbWV0YWRhdGFfaGFzaAAAAAAAA+4AAAAgAAAAAA==",
        "AAAAAAAAAAAAAAAKZ2V0X3N0YXR1cwAAAAAAAQAAAAAAAAAMcHJvdmlkZXJfcmVmAAAD7gAAACAAAAABAAAAAQ==",
        "AAAAAAAAAAAAAAAKc2V0X3N0YXR1cwAAAAAAAgAAAAAAAAAMcHJvdmlkZXJfcmVmAAAD7gAAACAAAAAAAAAACHZlcmlmaWVkAAAAAQAAAAA=",
        "AAAAAAAAAAAAAAANX19jb25zdHJ1Y3RvcgAAAAAAAAEAAAAAAAAACWF1dGhvcml0eQAAAAAAABMAAAAA" ]),
      options
    )
  }
  public readonly fromJSON = {
    register: this.txFromJSON<null>,
        get_status: this.txFromJSON<boolean>,
        set_status: this.txFromJSON<null>
  }
}