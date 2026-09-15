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





export interface Receipt {
  grant_ref: Buffer;
  purpose_hash: Buffer;
  timestamp: u64;
}


export interface Client {
  /**
   * Construct and simulate a get transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  get: ({access_ref}: {access_ref: Buffer}, options?: MethodOptions) => Promise<AssembledTransaction<Option<Receipt>>>

  /**
   * Construct and simulate a record_receipt transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  record_receipt: ({grant_ref, access_ref, purpose_hash}: {grant_ref: Buffer, access_ref: Buffer, purpose_hash: Buffer}, options?: MethodOptions) => Promise<AssembledTransaction<null>>

}
export class Client extends ContractClient {
  static async deploy<T = Client>(
        /** Constructor/Initialization Args for the contract's `__constructor` method */
        {recorder}: {recorder: string},
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
    return ContractClient.deploy({recorder}, options)
  }
  constructor(public readonly options: ContractClientOptions) {
    super(
      new ContractSpec([ "AAAAAQAAAAAAAAAAAAAAB1JlY2VpcHQAAAAAAwAAAAAAAAAJZ3JhbnRfcmVmAAAAAAAD7gAAACAAAAAAAAAADHB1cnBvc2VfaGFzaAAAA+4AAAAgAAAAAAAAAAl0aW1lc3RhbXAAAAAAAAAG",
        "AAAABQAAAAAAAAAAAAAADkFjY2Vzc1JlY29yZGVkAAAAAAABAAAAD2FjY2Vzc19yZWNvcmRlZAAAAAACAAAAAAAAAAphY2Nlc3NfcmVmAAAAAAPuAAAAIAAAAAEAAAAAAAAACXRpbWVzdGFtcAAAAAAAAAYAAAAAAAAAAg==",
        "AAAAAAAAAAAAAAADZ2V0AAAAAAEAAAAAAAAACmFjY2Vzc19yZWYAAAAAA+4AAAAgAAAAAQAAA+gAAAfQAAAAB1JlY2VpcHQA",
        "AAAAAAAAAAAAAAANX19jb25zdHJ1Y3RvcgAAAAAAAAEAAAAAAAAACHJlY29yZGVyAAAAEwAAAAA=",
        "AAAAAAAAAAAAAAAOcmVjb3JkX3JlY2VpcHQAAAAAAAMAAAAAAAAACWdyYW50X3JlZgAAAAAAA+4AAAAgAAAAAAAAAAphY2Nlc3NfcmVmAAAAAAPuAAAAIAAAAAAAAAAMcHVycG9zZV9oYXNoAAAD7gAAACAAAAAA" ]),
      options
    )
  }
  public readonly fromJSON = {
    get: this.txFromJSON<Option<Receipt>>,
        record_receipt: this.txFromJSON<null>
  }
}