const anchor = require("@coral-xyz/anchor");
const assert = require("assert");

const {SystemProgram} = anchor.web3;


describe("counter_app", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.CounterApp;
  

  
  it("Initialize the count to 1", async()=>{
    const counter_account = anchor.web3.Keypair.generate();
    let current_value = 0;
    await program.rpc.initialize({
      accounts:{
        counterAccount: counter_account.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId
      },
      signers: [counter_account]
    })
    const account = await program.account.counterAccount.fetch(counter_account.publicKey);
    current_value = account.value;
    assert.ok(account.value == 1);
    _baseAccount = counter_account;
    _currentValue = current_value;
  })

  it("Increment", async()=>{
      const baseAccount = _baseAccount;
      const currentValue = _currentValue;
      await program.rpc.increment({
        accounts:{
          counterAccount: baseAccount.publicKey
        }
      })
      const account = await program.account.counterAccount.fetch(baseAccount.publicKey);
      assert.ok(account.value == (currentValue + 1));
  })
});

