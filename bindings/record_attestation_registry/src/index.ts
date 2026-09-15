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





export interface Attestation {
  content_hash: Buffer;
  issuer: string;
}


export interface Client {
  /**
   * Construct and simulate a get transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  get: ({record_ref}: {record_ref: Buffer}, options?: MethodOptions) => Promise<AssembledTransaction<Option<Attestation>>>

  /**
   * Construct and simulate a attest transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  attest: ({record_ref, content_hash, issuer}: {record_ref: Buffer, content_hash: Buffer, issuer: string}, options?: MethodOptions) => Promise<AssembledTransaction<null>>

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
      new ContractSpec([ "AAAAAQAAAAAAAAAAAAAAC0F0dGVzdGF0aW9uAAAAAAIAAAAAAAAADGNvbnRlbnRfaGFzaAAAA+4AAAAgAAAAAAAAAAZpc3N1ZXIAAAAAABM=",
        "AAAABQAAAAAAAAAAAAAADlJlY29yZEF0dGVzdGVkAAAAAAABAAAAD3JlY29yZF9hdHRlc3RlZAAAAAADAAAAAAAAAApyZWNvcmRfcmVmAAAAAAPuAAAAIAAAAAEAAAAAAAAABmlzc3VlcgAAAAAAEwAAAAEAAAAAAAAADGNvbnRlbnRfaGFzaAAAA+4AAAAgAAAAAAAAAAI=",
        "AAAAAAAAAAAAAAADZ2V0AAAAAAEAAAAAAAAACnJlY29yZF9yZWYAAAAAA+4AAAAgAAAAAQAAA+gAAAfQAAAAC0F0dGVzdGF0aW9uAA==",
        "AAAAAAAAAAAAAAAGYXR0ZXN0AAAAAAADAAAAAAAAAApyZWNvcmRfcmVmAAAAAAPuAAAAIAAAAAAAAAAMY29udGVudF9oYXNoAAAD7gAAACAAAAAAAAAABmlzc3VlcgAAAAAAEwAAAAA=" ]),
      options
    )
  }
  public readonly fromJSON = {
    get: this.txFromJSON<Option<Attestation>>,
        attest: this.txFromJSON<null>
  }
}