import { ApiPromise, WsProvider, Keyring } from "@polkadot/api";
import {KeyringPair} from '@polkadot/keyring/types';
import {metadata} from '@polkadot/types/interfaces/essentials';
import '@polkadot/api-augment';

const WEB_SOCKET = 'ws://localhost:9944';
const sleep = ms => new Promise(resolve => setTimeout(resolve, ms));

const connectSubstrate =async () => {
    const wsProvider = new WsProvider(WEB_SOCKET);
    const api = await ApiPromise.create({provider: wsProvider,types:{}});
    await api.isReady;
    console.log("connection to substrate is OK.")
    return api;
};

//获取常量
const getConst =async (api:ApiPromise) => {
    const existentialDeposit = await api.consts.balances.existentialDeposit.toHuman();
    return existentialDeposit;
}

//获取balance
const getFreeBalance =async (api:ApiPromise, address: string) => {
    const aliceAccount = await api.query.system.account(address);
    return aliceAccount["data"]["free"].toHuman();
}
//打印balance
const printAliceBobBalance =async (api:ApiPromise) => {
    const keyring = new Keyring({ type: "sr25519"});
    const alice = keyring.addFromUri('//Alice');
    const bob = keyring.addFromUri('//Bob');
    console.log("alice balance is:", await getFreeBalance(api,alice.address))
    console.log("bob balance is:", await getFreeBalance(api, bob.address))
}

//转账
const transferFromAliceToBob =async (api:ApiPromise, amount: number) => {
    const keyring = new Keyring({type: 'sr25519'});
    const alice = keyring.addFromUri('//Alice');
    const bob = keyring.addFromUri('//Bob');
    await api.tx.balances.transfer(bob.address,amount)
        .signAndSend(alice, res => {
            console.log(`Tx status: ${res.status}`);
        });
}

const subscribeAliceBalance =async (api:ApiPromise) => {
    const keyring = new Keyring({ type:'sr25519'});
    const alice = keyring.addFromUri('//Alice');
    await api.query.system.account(alice.address, aliceAcct =>{
        console.log("订阅Alice账户信息");
        const aliceFreeSub = aliceAcct.data.free;
        console.log(`Alice 账户 (sub):${aliceFreeSub}`);
    });
};
const main =async () => {
    const api = await connectSubstrate();
    console.log("const value existentialDeposit is:",await getConst(api))
    
    await printAliceBobBalance(api);

    await transferFromAliceToBob(api, 10**12);
    await sleep(6000);
    await printAliceBobBalance(api);
    await subscribeAliceBalance(api);
    await sleep(60000)

    console.log('game over');
};

main()
    .then(() =>{
        console.log("sucessfully exited");
        process.exit(0);
    })
    .catch(err => {
        console.log('error occur:',err);
        process.exit(1);
    })





