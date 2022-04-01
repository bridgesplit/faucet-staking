import type { InstructionLogs } from "@saberhq/solana-contrib";
import type { SendTransactionError } from "@solana/web3.js";
/**
 * Formats instruction logs to be printed to the console.
 * @param logs
 */
export declare const formatInstructionLogsForConsole: (logs: readonly InstructionLogs[]) => string;
export declare const printSendTransactionError: (err: SendTransactionError) => void;
//# sourceMappingURL=printInstructionLogs.d.ts.map