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





export interface Grant {
  expires_at: u64;
  grantor: string;
  recipient: string;
  revoked: boolean;
  scope_hash: Buffer;
  starts_at: u64;
  subject: Buffer;
}



export interface Client {
  /**
   * Construct and simulate a get transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  get: ({grant_ref}: {grant_ref: Buffer}, options?: MethodOptions) => Promise<AssembledTransaction<Option<Grant>>>

  /**
   * Construct and simulate a grant transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  grant: ({grant_ref, grantor, subject, recipient, scope_hash, starts_at, expires_at}: {grant_ref: Buffer, grantor: string, subject: Buffer, recipient: string, scope_hash: Buffer, starts_at: u64, expires_at: u64}, options?: MethodOptions) => Promise<AssembledTransaction<null>>

  /**
   * Construct and simulate a revoke transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  revoke: ({grant_ref}: {grant_ref: Buffer}, options?: MethodOptions) => Promise<AssembledTransaction<null>>

  /**
   * Construct and simulate a is_active transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  is_active: ({grant_ref}: {grant_ref: Buffer}, options?: MethodOptions) => Promise<AssembledTransaction<boolean>>

}
export class Client extends ContractClient {
  static async deploy<T = Client>(
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
    return ContractClient.deploy(null, options)
  }
  constructor(public readonly options: ContractClientOptions) {
    super(
      new ContractSpec([ "AAAAAQAAAAAAAAAAAAAABUdyYW50AAAAAAAABwAAAAAAAAAKZXhwaXJlc19hdAAAAAAABgAAAAAAAAAHZ3JhbnRvcgAAAAATAAAAAAAAAAlyZWNpcGllbnQAAAAAAAATAAAAAAAAAAdyZXZva2VkAAAAAAEAAAAAAAAACnNjb3BlX2hhc2gAAAAAA+4AAAAgAAAAAAAAAAlzdGFydHNfYXQAAAAAAAAGAAAAAAAAAAdzdWJqZWN0AAAAA+4AAAAg",
        "AAAABQAAAAAAAAAAAAAADkNvbnNlbnRHcmFudGVkAAAAAAABAAAAD2NvbnNlbnRfZ3JhbnRlZAAAAAABAAAAAAAAAAlncmFudF9yZWYAAAAAAAPuAAAAIAAAAAEAAAAC",
        "AAAABQAAAAAAAAAAAAAADkNvbnNlbnRSZXZva2VkAAAAAAABAAAAD2NvbnNlbnRfcmV2b2tlZAAAAAABAAAAAAAAAAlncmFudF9yZWYAAAAAAAPuAAAAIAAAAAEAAAAC",
        "AAAAAAAAAAAAAAADZ2V0AAAAAAEAAAAAAAAACWdyYW50X3JlZgAAAAAAA+4AAAAgAAAAAQAAA+gAAAfQAAAABUdyYW50AAAA",
        "AAAAAAAAAAAAAAAFZ3JhbnQAAAAAAAAHAAAAAAAAAAlncmFudF9yZWYAAAAAAAPuAAAAIAAAAAAAAAAHZ3JhbnRvcgAAAAATAAAAAAAAAAdzdWJqZWN0AAAAA+4AAAAgAAAAAAAAAAlyZWNpcGllbnQAAAAAAAATAAAAAAAAAApzY29wZV9oYXNoAAAAAAPuAAAAIAAAAAAAAAAJc3RhcnRzX2F0AAAAAAAABgAAAAAAAAAKZXhwaXJlc19hdAAAAAAABgAAAAA=",
        "AAAAAAAAAAAAAAAGcmV2b2tlAAAAAAABAAAAAAAAAAlncmFudF9yZWYAAAAAAAPuAAAAIAAAAAA=",
        "AAAAAAAAAAAAAAAJaXNfYWN0aXZlAAAAAAAAAQAAAAAAAAAJZ3JhbnRfcmVmAAAAAAAD7gAAACAAAAABAAAAAQ==" ]),
      options
    )
  }
  public readonly fromJSON = {
    get: this.txFromJSON<Option<Grant>>,
        grant: this.txFromJSON<null>,
        revoke: this.txFromJSON<null>,
        is_active: this.txFromJSON<boolean>
  }
}