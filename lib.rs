#![no_std]
//! Disposable authorization fixtures, never a production wallet or asset custodian.
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};
#[cfg(all(feature="sink",feature="vulnerable"))]
compile_error!("sink and vulnerable are separate fixture builds");

#[cfg(feature="sink")]
mod sink {
    use super::*;
    use soroban_sdk::{IntoVal, vec};
    #[contracttype]
    enum Key { Count(Address), Last(Address) }
    #[contract]
    pub struct FixtureSink;
    #[contractimpl]
    impl FixtureSink {
        /// Records test intent only. No token transfer, assets or financial balance.
        pub fn consume(env: Env, account: Address, recipient: Address, amount: i128) -> u32 {
            account.require_auth_for_args(vec![&env, account.clone().into_val(&env), recipient.clone().into_val(&env), amount.into_val(&env)]);
            let count: u32 = env.storage().persistent().get(&Key::Count(account.clone())).unwrap_or(0);
            env.storage().persistent().set(&Key::Count(account.clone()), &(count + 1));
            env.storage().persistent().set(&Key::Last(account), &(recipient, amount));
            count + 1
        }
        pub fn count(env: Env, account: Address) -> u32 {
            env.storage().persistent().get(&Key::Count(account)).unwrap_or(0)
        }
    }
}

#[cfg(not(feature="sink"))]
mod account {
    use super::*;
    use soroban_sdk::{auth::{Context, CustomAccountInterface}, contracterror, crypto::Hash, Symbol, TryFromVal, Vec};
    #[contracttype]
    #[derive(Clone)]
    pub struct Policy { pub delegate: Address, pub target: Address, pub recipient: Address, pub cap: i128 }
    #[contracttype]
    enum Key { Policy }
    #[contracterror]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
    #[repr(u32)]
    pub enum Error { Policy = 1, Delegate = 2, Context = 3, Target = 4, Arguments = 5, Recipient = 6, Amount = 7 }
    #[contract]
    pub struct PolicyAccount;
    #[contractimpl]
    impl PolicyAccount {
        pub fn __constructor(env: Env, delegate: Address, target: Address, recipient: Address, cap: i128) {
            assert!(cap > 0, "positive cap required");
            env.storage().instance().set(&Key::Policy, &Policy {delegate,target,recipient,cap});
        }
        pub fn policy(env: Env) -> Policy { env.storage().instance().get(&Key::Policy).unwrap() }
        pub fn fixture_kind() -> u32 { if cfg!(feature="vulnerable") { 1 } else { 0 } }
    }
    fn check_context(env: &Env, p: &Policy, contexts: Vec<Context>) -> Result<(), Error> {
        if contexts.len() != 1 { return Err(Error::Context); }
        match contexts.get(0).ok_or(Error::Context)? {
            Context::Contract(c) => {
                if c.contract != p.target || c.fn_name != Symbol::new(env,"consume") { return Err(Error::Target); }
                if c.args.len() != 3 { return Err(Error::Arguments); }
                let account = Address::try_from_val(env,&c.args.get(0).unwrap()).map_err(|_|Error::Arguments)?;
                let recipient = Address::try_from_val(env,&c.args.get(1).unwrap()).map_err(|_|Error::Arguments)?;
                let amount = i128::try_from_val(env,&c.args.get(2).unwrap()).map_err(|_|Error::Arguments)?;
                if account != env.current_contract_address() { return Err(Error::Arguments); }
                if recipient != p.recipient { return Err(Error::Recipient); }
                if amount <= 0 { return Err(Error::Amount); }
                // Intentional test mutation: vulnerable fixture omits only the cap check.
                if !cfg!(feature="vulnerable") && amount > p.cap { return Err(Error::Amount); }
                Ok(())
            },
            _ => Err(Error::Context),
        }
    }
    #[contractimpl]
    impl CustomAccountInterface for PolicyAccount {
        type Signature = ();
        type Error = Error;
        fn __check_auth(env: Env, _payload: Hash<32>, _signature: (), contexts: Vec<Context>) -> Result<(), Error> {
            let p: Policy = env.storage().instance().get(&Key::Policy).ok_or(Error::Policy)?;
            check_context(&env,&p,contexts)?;
            let delegates = env.custom_account().get_delegated_signers();
            if delegates.len()!=1 || delegates.get(0)!=Some(p.delegate.clone()) { return Err(Error::Delegate); }
            env.custom_account().delegate_auth(&p.delegate);
            Ok(())
        }
    }
    #[cfg(test)]
    mod tests {
        use super::*;
        use soroban_sdk::{auth::ContractContext, testutils::Address as _, vec, IntoVal};
        fn fixture() -> (Env, Address, Policy) {
            let env=Env::default(); let p=Policy {delegate:Address::generate(&env),target:Address::generate(&env),recipient:Address::generate(&env),cap:100};
            let id=env.register(PolicyAccount,(&p.delegate,&p.target,&p.recipient,p.cap)); (env,id,p)
        }
        fn context(env:&Env,id:&Address,p:&Policy,amount:i128)->Vec<Context>{vec![env,Context::Contract(ContractContext{contract:p.target.clone(),fn_name:Symbol::new(env,"consume"),args:vec![env,id.clone().into_val(env),p.recipient.clone().into_val(env),amount.into_val(env)]})]}
        #[test] fn accepts_cap_boundary(){let(e,id,p)=fixture();e.as_contract(&id,||assert_eq!(check_context(&e,&p,context(&e,&id,&p,100)),Ok(())));}
        #[test] fn cap_mutation_is_explicit(){let(e,id,p)=fixture();e.as_contract(&id,||assert_eq!(check_context(&e,&p,context(&e,&id,&p,101)),if cfg!(feature="vulnerable"){Ok(())}else{Err(Error::Amount)}));}
        #[test] fn rejects_nonpositive(){let(e,id,p)=fixture();e.as_contract(&id,||for n in [0,-1]{assert_eq!(check_context(&e,&p,context(&e,&id,&p,n)),Err(Error::Amount));});}
        #[test] fn rejects_wrong_target(){let(e,id,p)=fixture();let mut q=p.clone();q.target=Address::generate(&e);e.as_contract(&id,||assert_eq!(check_context(&e,&p,context(&e,&id,&q,1)),Err(Error::Target)));}
        #[test] fn rejects_wrong_recipient(){let(e,id,p)=fixture();let mut q=p.clone();q.recipient=Address::generate(&e);e.as_contract(&id,||assert_eq!(check_context(&e,&p,context(&e,&id,&q,1)),Err(Error::Recipient)));}
        #[test] fn rejects_empty_and_extra_contexts(){let(e,id,p)=fixture();e.as_contract(&id,||{assert_eq!(check_context(&e,&p,vec![&e]),Err(Error::Context));let mut c=context(&e,&id,&p,1);c.push_back(c.get(0).unwrap());assert_eq!(check_context(&e,&p,c),Err(Error::Context));});}
        #[test] fn rejects_wrong_account(){let(e,id,p)=fixture();let other=Address::generate(&e);e.as_contract(&id,||assert_eq!(check_context(&e,&p,context(&e,&other,&p,1)),Err(Error::Arguments)));}
    }
}
