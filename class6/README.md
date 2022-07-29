```typescript
const subscribeAliceBalance =async (api:ApiPromise) => {
    const keyring = new Keyring({ type:'sr25519'});
    const alice = keyring.addFromUri('//Alice');
    await api.query.system.account(alice.address, aliceAcct =>{
        console.log("订阅Alice账户信息");
        const aliceFreeSub = aliceAcct.data.free;
        console.log(`Alice 账户 (sub):${aliceFreeSub}`);
    });
};
```
![image](https://github.com/enginefuture/subtratework/blob/master/class5/%E8%BF%90%E8%A1%8C%E6%88%AA%E5%9B%BE.png)
![image](https://github.com/enginefuture/subtratework/blob/master/class5/%E8%BF%90%E8%A1%8C%E6%88%AA%E5%9B%BE2.png)
