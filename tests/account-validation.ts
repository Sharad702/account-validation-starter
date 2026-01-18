import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AccountValidation } from "../target/types/account_validation";
import { expect } from "chai";

describe("account-validation", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.AccountValidation as Program<AccountValidation>;

  it("Creates a profile with valid data", async () => {
    // TODO: Test profile creation
    // 1. Derive profile PDA
    // 2. Create profile with valid username
    // 3. Verify profile data is correct
  });

  it("Rejects invalid username", async () => {
    // TODO: Test username validation
    // 1. Try to create profile with empty username
    // 2. Try to create profile with too-long username
    // 3. Expect errors
  });

  it("Updates profile as owner", async () => {
    // TODO: Test profile update
    // 1. Create profile
    // 2. Update username
    // 3. Verify update succeeded
  });

  it("Prevents unauthorized update", async () => {
    // TODO: Test unauthorized access
    // 1. Create profile with user A
    // 2. Try to update as user B
    // 3. Expect Unauthorized error
  });

  it("Closes profile and returns rent", async () => {
    // TODO: Test profile closure
    // 1. Create profile
    // 2. Get initial balances
    // 3. Close profile
    // 4. Verify lamports returned
    // 5. Verify account no longer exists
  });

  it("Transfers ownership correctly", async () => {
    // TODO: Test ownership transfer
    // 1. Create profile with user A
    // 2. Transfer to user B
    // 3. Verify new owner can update
    // 4. Verify old owner cannot update
  });
});
