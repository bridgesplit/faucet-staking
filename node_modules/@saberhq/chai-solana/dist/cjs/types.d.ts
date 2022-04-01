import "chai-bn";
import "chai-as-promised";
import type { Address } from "@project-serum/anchor";
import type { BigintIsh, TokenAmount } from "@saberhq/token-utils";
declare global {
    namespace Chai {
        interface TokenAmountComparer {
            (value: TokenAmount | BigintIsh, message?: string): void;
        }
        interface TokenAmountAssertion {
            equal: TokenAmountComparer;
            equals: TokenAmountComparer;
            eq: TokenAmountComparer;
            zero: () => void;
        }
        interface Assertion {
            eqAddress: (otherKey: Address, message?: string) => Assertion;
            eqAmount: (otherAmount: TokenAmount, message?: string) => Assertion;
            tokenAmount: TokenAmountAssertion;
        }
    }
}
//# sourceMappingURL=types.d.ts.map